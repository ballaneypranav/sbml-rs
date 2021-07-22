use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Ident, Result, Token, Type};

mod kw {
    syn::custom_keyword!(to);
    syn::custom_keyword!(with);
}

#[proc_macro]
pub fn attach(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as OpenInput);
    //println!("{:?}", input);

    let tag = &input.tag;
    let tag_str = input.tag.to_string();
    let parents = &input.parents;
    let mut parent_field = input.tag.to_string().to_case(Case::Snake);
    let parent_field_ident: Ident;

    // create expression
    let create_stream;
    if parents[0].to_string().starts_with("ListOf") {
        // make plural
        if !parent_field.ends_with('s') {
            parent_field.push('s');
        }
        // identifier for this object in it's parent (the new object created for this tag)
        parent_field_ident = Ident::new(&parent_field, Span::call_site());
        create_stream = quote! {
                    parent.#parent_field_ident.push(current.clone());
        }
    } else {
        // identifier for this object in it's parent (the new object created for this tag)
        parent_field_ident = Ident::new(&parent_field, Span::call_site());
        create_stream = quote! {
                    parent.#parent_field_ident = Some(current.clone());
        }
    }
    // attributes field names and types
    let attr_idents = input.attr_idents;
    let attr_types = input.attr_types;

    // also need strings for matching tokens
    let mut attr_str: Vec<String> = Vec::new();
    for ident in &attr_idents {
        attr_str.push(ident.to_string().to_case(Case::Camel));
    }

    let tokens = quote! {
        {
            // Create object outside match expression because
            // there are two repeats - one for each parent and
            // one for each attr. If object is created inside, these
            // loops will be nested and quote apparently doesn't like that

            // instantiate object of the tag that was found
            let mut #parent_field_ident = #tag::default();
            // parse any attributes, keeping their types in mind
            let attributes = e.attributes().map(|a| a.unwrap()).collect::<Vec<_>>();
            //println!("{:?}", attributes);
            for attribute in attributes {
                let key = str::from_utf8(attribute.key).unwrap();
                let value = attribute.unescape_and_decode_value(&reader).unwrap();
                match key {
                    #(#attr_str => {
                        #parent_field_ident.#attr_idents =
                            Some(value.parse::<#attr_types>().expect("Incorrect type"));
                    })*
                    _ => {
                        //errors.push(format!("Attribute '{}' not parsed for '{}'", key, #tag_str));
                        panic!("Attribute '{}' not parsed for '{}'", key, #tag_str);
                    }
                }
            }
            // match the current tag
            match nodes[current] {
                // with the parent
                #(Tag::#parents (ref mut parent) => {
                    // create Tag enum object
                    new_tag = Some(Tag::#tag(#parent_field_ident));
                    // update current pointer (which is really an int)
                    current = nodes_len;
                    // update parent pointer of new tag
                    //parent.#parent_field_ident.push(current.clone());
                    #create_stream
                    // push current pointer to stack
                    stack.push(current.clone());
                })*
                _ => {}
            }
        }
    };
    tokens.into()
}

#[derive(Debug)]
struct OpenInput {
    tag: Ident,
    parents: Vec<Ident>,
    attr_idents: Vec<Ident>,
    attr_types: Vec<Type>,
}

impl Parse for OpenInput {
    fn parse(input: ParseStream) -> Result<Self> {
        //println!("{:#?}", input);
        // parse tag
        let tag = syn::Ident::parse(input)?;
        // define lookahead function
        let mut lookahead = input.lookahead1();
        // define fields used later
        let mut attr_idents = Vec::new();
        let mut attr_types = Vec::new();

        // if attributes are specified
        if lookahead.peek(kw::with) {
            let _with = input.parse::<kw::with>()?;

            // loop over attributes and types
            loop {
                // parse attribute field name as ident
                let ident = syn::Ident::parse(input)?;
                attr_idents.push(ident);
                let _as = input.parse::<Token![as]>();
                // parse attribute type
                let ty = syn::Type::parse(input)?;
                attr_types.push(ty);

                // consume comma if it exists
                if input.peek(Token![,]) {
                    input.parse::<Token![,]>()?;
                }

                // break if found into
                // lookahead works only once
                lookahead = input.lookahead1();
                if lookahead.peek(kw::to) {
                    break;
                }
            }
        }
        let _to = input.parse::<kw::to>()?;

        // parse parent
        let mut parents = vec![syn::Ident::parse(input)?];

        // see if there are multiple parents
        loop {
            lookahead = input.lookahead1();
            if lookahead.peek(Token![|]) {
                input.parse::<Token![|]>()?;
            } else {
                break;
            }

            lookahead = input.lookahead1();
            if lookahead.peek(Ident) {
                parents.push(syn::Ident::parse(input)?);
            }
        }

        //println!("Parents: {:?}", parents);

        Ok(OpenInput {
            tag,
            parents,
            attr_idents,
            attr_types,
        })
    }
}

#[proc_macro]
pub fn attach_math(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as MathInput);
    let parents = &input.parents;

    let tokens = quote! {
        match nodes[current] {
            #(Tag::#parents(ref mut parent) => {
                let math_tag = MathTag::default()
                    .with_nodes(math_nodes)
                    .with_parent(current);
                new_tag = Some(Tag::MathTag(math_tag));
                parent.math = Some(nodes_len.clone());
            })*
            _ => {}
        }
    };
    tokens.into()
}

#[derive(Debug)]
struct MathInput {
    parents: Vec<Ident>,
}

impl Parse for MathInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let parsed_input =
            syn::punctuated::Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        let parents: Vec<Ident> = parsed_input.into_iter().collect();

        Ok(MathInput { parents })
    }
}

#[proc_macro]
pub fn close(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as CloseInput);
    //println!("{:?}", input);

    let tag = &input.tag;
    let tag_str = input.tag.to_string();

    let tokens = quote! {
        match nodes[current] {
            Tag::#tag (ref mut tag_field) => {
                stack.pop();
                current = stack.last().unwrap().to_owned();
                tag_field.parent = Some(current.clone());
                //println!("Closing {}", #tag_str);
            }
            _ => {
                panic!("Attempted to close {} but currently in {:?}", #tag_str, nodes[current]);
            }
        }
    };
    tokens.into()
}

#[derive(Debug)]
struct CloseInput {
    tag: Ident,
}

impl Parse for CloseInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let tag = syn::Ident::parse(input)?;
        Ok(CloseInput { tag })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

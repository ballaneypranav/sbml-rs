use std::env;
use std::str;

use quick_xml::events::Event;
use quick_xml::Reader;
use mathml_rs::MathNode;

use macros::*;
mod structs;
use structs::*;


#[allow(unused_variables, unused_assignments)]
fn parse(filename: &str) {
    println!("{}", filename);

    // read file
    //let file = File::open().unwrap();
    let mut reader = Reader::from_file(filename).expect("File error.");
    reader.trim_text(true);
    reader.expand_empty_elements(true);
    let mut buf = Vec::new();
    let mut txt = Vec::new();

    let mut stack: Vec<TagIndex> = Vec::new();
    let mut container = Vec::new();
    let mut container_len = 0;

    let model = Model::new();
    container.push(Tag::Model(model));
    container_len += 1;
    let mut current = 0;
    stack.push(current);
    println!("{:?}", current);

    loop {
        match reader.read_event(&mut buf) {
            // for each starting tag
            Ok(Event::Start(ref e)) => {
                let mut new_tag = None;
                match e.name() {
                    b"listOfSpecies" => attach!(ListOfSpecies to Model),
                    b"listOfReactions" => attach!(ListOfReactions to Model),
                    b"species" => {
                        push!(Species with 
                                name as String, 
                                compartment as String 
                            into ListOfSpecies)
                    }
                    b"reaction" => push!(Reaction into ListOfReactions),
                    b"kineticLaw" => attach!(KineticLaw to Reaction),
                    b"math" => {
                        let (math_tag, returned_reader) = mathml_rs::parse_fragment(reader);
                        reader = returned_reader;

                        match container[current] {
                            Tag::KineticLaw (ref mut parent) => {
                                new_tag = Some(Tag::MathTag(MathTag::new_from_node(math_tag)));
                                current = container_len;
                                parent.math = Some(current.clone());
                                stack.push(current.clone());
                            }
                            _ => {}
                        }

                    }
                    _ => {
                        println!("Tag not parsed: {}", str::from_utf8(e.name()).unwrap());
                    }
                }
                match new_tag {
                    Some(t) => {
                        container.push(t);
                        container_len += 1;
                    }
                    None => {}
                }
            }
            // for each closing tag
            Ok(Event::End(ref e)) => match e.name() {
                b"listOfSpecies" => close![ListOfSpecies],
                b"listOfReactions" => close![ListOfReactions],
                b"species" => close![Species],
                b"reaction" => close![Reaction],
                b"kineticLaw" => close![KineticLaw],
                b"math" => close![MathTag],
                _ => {}
            },
            // unescape and decode the text event using the reader encoding
            Ok(Event::Text(e)) => txt.push(e.unescape_and_decode(&reader).unwrap()),
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other `Event`s we do not consider here
        }
    }
    for item in container {
        println!("{:?}", item);
    }
    println!("{:?}", stack);
    println!("{:?}", current);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let filename = "../models/small.xml";

        parse(filename);
    }
}

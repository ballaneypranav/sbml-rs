use std::env;
use std::str;

use quick_xml::events::Event;
use quick_xml::Reader;

use sbml_macros::{attach, close, push};

mod structs;
use structs::compartments::*;
use structs::math::*;
use structs::model::*;
use structs::reactions::*;
use structs::species::*;
use structs::tag::*;
use structs::units::*;

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

    let model = Model::default();
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
                    b"listOfUnitDefinitions" => attach!(ListOfUnitDefinitions to Model),
                    b"unitDefinition" => push!(UnitDefinition with
                                                id as String
                                            into ListOfUnitDefinitions),
                    b"listOfUnits" => attach!(ListOfUnits to UnitDefinition),
                    b"unit" => push!(Unit with 
                                        kind as String,
                                        exponent as f64,
                                        scale as i64,
                                        multiplier as f64
                                        into ListOfUnits),
                    b"listOfCompartments" => attach!(ListOfCompartments to Model),
                    b"compartment" => push!(Compartment with
                                                name as String,
                                                id as String,
                                                units as String,
                                                constant as bool,
                                                spatial_dimensions as f64,
                                                sbo_term as String,
                                                size as f64
                                            into ListOfCompartments),
                    b"species" => push!(Species with
                                            id as String,
                                            name as String,
                                            meta_id as String,
                                            sbo_term as String,
                                            compartment as String,
                                            initial_concentration as f64,
                                            initial_amount as f64,
                                            substance_units as String,
                                            has_only_substance_units as bool,
                                            boundary_condition as bool,
                                            constant as bool,
                                            conversion_factor as String,
                                    into ListOfSpecies),
                    b"reaction" => push!(Reaction into ListOfReactions),
                    b"kineticLaw" => attach!(KineticLaw to Reaction),
                    b"math" => {
                        let (math_nodes, returned_reader) = mathml_rs::parse_fragment(reader);
                        reader = returned_reader;

                        match container[current] {
                            Tag::KineticLaw(ref mut parent) => {
                                let math_tag = MathTag::default()
                                    .with_nodes(math_nodes)
                                    .with_parent(current);
                                new_tag = Some(Tag::MathTag(math_tag));
                                parent.math = Some(current.clone());
                            }
                            _ => {}
                        }
                    }
                    b"sbml" => {}
                    b"model" => {}
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
                b"listOfUnitDefinitions" => close![ListOfUnitDefinitions],
                b"unitDefinition" => close![UnitDefinition],
                b"listOfUnits" => close![ListOfUnits],
                b"unit" => close![Unit],
                b"listOfCompartments" => close![ListOfCompartments],
                b"compartment" => close![Compartment],
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
        let filename = "../../testsuites/core-semantic/00001/00001-sbml-l3v2.xml";

        parse(filename);
    }
}

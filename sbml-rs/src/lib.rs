use std::str;

use quick_xml::events::Event;
use quick_xml::Reader;

use sbml_macros::{attach, attach_math, close};

pub mod structs;
pub use structs::compartments::*;
pub use structs::function_definitions::*;
pub use structs::initial_assignments::*;
pub use structs::math::*;
pub use structs::model::*;
pub use structs::parameters::*;
pub use structs::reactions::*;
pub use structs::root::*;
pub use structs::rules::*;
pub use structs::species::*;
pub use structs::tag::*;
pub use structs::units::*;

#[allow(unused_variables, unused_assignments, dead_code)]
pub fn parse(filename: &str) -> Result<Model, Vec<String>> {
    // read file
    //let file = File::open().unwrap();
    let mut reader = Reader::from_file(filename).expect("File error.");
    reader.trim_text(true);
    reader.expand_empty_elements(true);
    let mut buf = Vec::new();

    let mut stack: Vec<TagIndex> = Vec::new();
    let mut nodes = Vec::new();
    let mut nodes_len = 0;

    let root = Root::default();
    nodes.push(Tag::Root(root));
    nodes_len += 1;
    let mut current = 0;
    stack.push(current);

    loop {
        match reader.read_event(&mut buf) {
            // for each starting tag
            Ok(Event::Start(ref e)) => {
                let mut new_tag = None;
                match e.name() {
                    b"listOfUnitDefinitions" => attach!(ListOfUnitDefinitions to Root),
                    b"unitDefinition" => attach!(UnitDefinition with
                                                id as String
                                            to ListOfUnitDefinitions),
                    b"listOfUnits" => attach!(ListOfUnits to UnitDefinition),
                    b"unit" => attach!(Unit with 
                                        kind as String,
                                        exponent as f64,
                                        scale as i64,
                                        multiplier as f64
                                        to ListOfUnits),
                    b"listOfCompartments" => attach!(ListOfCompartments to Root),
                    b"compartment" => attach!(Compartment with
                                                name as String,
                                                id as String,
                                                units as String,
                                                constant as bool,
                                                spatial_dimensions as f64,
                                                sbo_term as String,
                                                size as f64
                                            to ListOfCompartments),
                    b"listOfParameters" => attach!(ListOfParameters to Root),
                    b"parameter" => attach!(Parameter with
                                            id as String,
                                            name as String,
                                            value as f64,
                                            units as String,
                                            constant as bool
                                        to ListOfParameters),
                    b"listOfSpecies" => attach!(ListOfSpecies to Root),
                    b"species" => attach!(Species with
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
                                    to ListOfSpecies),
                    b"listOfReactions" => attach!(ListOfReactions to Root),
                    b"reaction" => attach!(Reaction with
                                             id as String,
                                             reversible as bool,
                                             compartment as String,
                                             name as String,
                                             sbo_term as String
                                        to ListOfReactions),
                    b"listOfReactants" => attach!(ListOfReactants to Reaction),
                    b"listOfProducts" => attach!(ListOfProducts to Reaction),
                    b"speciesReference" => attach!(SpeciesReference with
                                                    id as String,
                                                    name as String,
                                                    species as String,
                                                    constant as bool,
                                                    sbo_term as String,
                                                    stoichiometry as f64,
                                        to ListOfReactants | ListOfProducts),
                    b"listOfModifiers" => attach!(ListOfModifiers to Reaction),
                    b"modifierSpeciesReference" => attach!(ModifierSpeciesReference with
                                                    id as String,
                                                    name as String,
                                                    species as String,
                                                    sbo_term as String,
                                        to ListOfModifiers),
                    b"kineticLaw" => attach!(KineticLaw to Reaction),
                    b"listOfLocalParameters" => attach!(ListOfLocalParameters to KineticLaw),
                    b"localParameter" => attach!(LocalParameter with
                                            id as String,
                                            value as f64,
                                            units as String,
                                            sbo_term as String,
                                        to ListOfLocalParameters),
                    b"math" => {
                        let (math_nodes, returned_reader) = mathml_rs::parse_fragment(reader);
                        reader = returned_reader;

                        attach_math![
                            KineticLaw,
                            FunctionDefinition,
                            InitialAssignment,
                            AssignmentRule
                        ];
                    }
                    b"listOfFunctionDefinitions" => attach!(ListOfFunctionDefinitions to Root),
                    b"functionDefinition" => {
                        attach!(FunctionDefinition with
                                    id as String,
                                    name as String,
                                    sbo_term as String
                                to ListOfFunctionDefinitions)
                    }
                    b"listOfInitialAssignments" => attach!(ListOfInitialAssignments to Root),
                    b"initialAssignment" => {
                        attach!(InitialAssignment with
                                    id as String,
                                    symbol as String,
                                    sbo_term as String
                                to ListOfInitialAssignments)
                    }
                    b"listOfRules" => attach!(ListOfRules to Root),
                    b"assignmentRule" => {
                        attach!(AssignmentRule with
                                    metaid as String,
                                    variable as String,
                                    sbo_term as String
                                to ListOfRules)
                    }
                    b"sbml" => {}
                    b"model" => {}
                    _ => {
                        panic!("Tag not parsed: {}", str::from_utf8(e.name()).unwrap());
                    }
                }
                if let Some(t) = new_tag {
                    nodes.push(t);
                    nodes_len += 1;
                }
            }
            // for each closing tag
            Ok(Event::End(ref e)) => match e.name() {
                b"listOfUnitDefinitions" => close![ListOfUnitDefinitions],
                b"unitDefinition" => close![UnitDefinition],
                b"listOfUnits" => close![ListOfUnits],
                b"unit" => close![Unit],
                b"listOfCompartments" => close![ListOfCompartments],
                b"compartment" => close![Compartment],
                b"listOfParameters" => close![ListOfParameters],
                b"parameter" => close![Parameter],
                b"listOfSpecies" => close![ListOfSpecies],
                b"species" => close![Species],
                b"listOfReactions" => close![ListOfReactions],
                b"reaction" => close![Reaction],
                b"listOfReactants" => close![ListOfReactants],
                b"listOfProducts" => close![ListOfProducts],
                b"speciesReference" => close![SpeciesReference],
                b"listOfModifiers" => close![ListOfModifiers],
                b"modifierSpeciesReference" => close![ModifierSpeciesReference],
                b"kineticLaw" => close![KineticLaw],
                b"listOfLocalParameters" => close![ListOfLocalParameters],
                b"localParameter" => close![LocalParameter],
                b"math" => close![MathTag],
                b"listOfFunctionDefinitions" => close![ListOfFunctionDefinitions],
                b"functionDefinition" => close![FunctionDefinition],
                b"listOfInitialAssignments" => close![ListOfInitialAssignments],
                b"initialAssignment" => close![InitialAssignment],
                _ => {}
            },
            // unescape and decode the text event using the reader encoding
            Ok(Event::Text(e)) => {
                let s = e.unescape_and_decode(&reader).unwrap();
                panic!("Unknown text found in {:?}", nodes[current]);
            }
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other `Event`s we do not consider here
        }
    }

    let model = Model::new(nodes);
    //dbg!(&model);

    Ok(model)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        for n in 25..26 {
            let filename = format!(
                "../../testsuites/function-definition-test-suite/{:0>5}/{:0>5}-sbml-l3v2.xml",
                n, n
            );
            println!("{}", filename);
            let result = parse(&filename);
            match result {
                Ok(model) => {
                    let function_definitions = model.function_definitions();
                    for function_definition in function_definitions {
                        println!("{}", function_definition.math_tag(&model).unwrap());
                    }
                }
                Err(errors) => {
                    println!("{:?}", errors);
                }
            }
        }
    }
}

use std::collections::HashMap;
use std::str;

use mathml_rs::Apply;
use mathml_rs::Ci;
use mathml_rs::Op;
use mathml_rs::OpNode;
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
    let mut model_attrs = HashMap::new();

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
                    b"sbml" => {}
                    b"model" => {
                        let attributes = e.attributes().map(|a| a.unwrap()).collect::<Vec<_>>();
                        for attribute in attributes {
                            let key = str::from_utf8(attribute.key).unwrap();
                            let value = attribute.unescape_and_decode_value(&reader).unwrap();
                            match key {
                                "id" | "substanceUnits" | "timeUnits" | "extentUnits"
                                | "volumeUnits" | "areaUnits" | "lengthUnits"
                                | "conversionFactor" | "metaid" | "name" => {
                                    model_attrs.insert(key.to_string(), value);
                                }
                                _ => panic!("Attribute {} not parsed for model.", key),
                            }
                        }
                    }
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
                                            sbo_term as String,
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
                            AssignmentRule,
                            RateRule,
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
                                    id as String,
                                    metaid as String,
                                    variable as String,
                                    sbo_term as String
                                to ListOfRules)
                    }
                    b"rateRule" => {
                        attach!(RateRule with
                                    id as String,
                                    metaid as String,
                                    variable as String,
                                    sbo_term as String
                                to ListOfRules)
                    }
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
                b"listOfRules" => close![ListOfRules],
                b"assignmentRule" => close![AssignmentRule],
                b"rateRule" => close![RateRule],
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

    let model = Model::new(nodes, model_attrs);

    Ok(model)
}

pub fn parse_with_converted_species(filename: &str) -> Result<Model, Vec<String>> {
    let mut model = parse(filename)?;

    let species = model.species();
    let mut species_compartment_id = HashMap::<String, String>::new();
    for sp in species {
        if let Some(species_id) = sp.id {
            if Some(false) == sp.has_only_substance_units {
                if let Some(compartment_id) = sp.compartment {
                    species_compartment_id.insert(species_id, compartment_id);
                }
            }
        }
    }
    //for node in &model.nodes {
    //if let Tag::MathTag(math_tag) = node {
    //println!("{}", math_tag);
    //}
    //}

    let mut new_nodes = model.nodes.clone();

    for i in 0..model.nodes.len() {
        match &model.nodes[i] {
            // perform replacement in each MathTag
            Tag::MathTag(math_tag) => {
                for j in 0..math_tag.nodes.len() {
                    match &math_tag.nodes[j] {
                        // replace each Ci that refers to a species
                        // with hasOnlySubstanceUnits = false
                        MathNode::Ci(ci) => {
                            if let Some(species_id) = &ci.name {
                                // check if the species is in the hashmap made earlier
                                if let Some(compartment) = species_compartment_id.get(species_id) {
                                    // if it is, make changes to the copy
                                    if let Tag::MathTag(math_tag_copy) = &mut new_nodes[i] {
                                        // replace Species Ci node with an Apply node and insert
                                        // Species Ci, Divide Op and Compartment Ci nodes at the end
                                        // create nodes Apply, Divide and Compartment
                                        let mut species_math_node = math_tag_copy.nodes[j].clone();
                                        let mut apply = Apply::default();
                                        let mut divide = OpNode::default();
                                        divide.op = Some(Op::Divide);
                                        let mut compartment = Ci::with_name(compartment.clone());

                                        // set child and parent pointers
                                        let length = math_tag_copy.nodes.len();
                                        apply.parent = ci.parent;
                                        apply.children = vec![length, length + 1, length + 2];
                                        apply.operator = Some(length);
                                        apply.operands = vec![length + 1, length + 2];
                                        divide.parent = Some(j);
                                        compartment.parent = Some(j);
                                        if let MathNode::Ci(species) = &mut species_math_node {
                                            species.parent = Some(j);
                                        }

                                        let apply_math_node = MathNode::Apply(apply);
                                        let divide_math_node = MathNode::Op(divide);
                                        let compartment_math_node = MathNode::Ci(compartment);
                                        math_tag_copy.nodes[j] = apply_math_node;
                                        math_tag_copy.nodes.push(divide_math_node);
                                        math_tag_copy.nodes.push(species_math_node);
                                        math_tag_copy.nodes.push(compartment_math_node);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    model.nodes = new_nodes;

    //for node in &model.nodes {
    //if let Tag::MathTag(math_tag) = node {
    //println!("{}", math_tag);
    //}
    //}

    Ok(model)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        for n in 1..2 {
            let filename = format!(
                "../../testsuites/core-semantic/{:0>5}/{:0>5}-sbml-l3v2.xml",
                n, n
            );
            println!("{}", filename);
            let result = parse_with_converted_species(&filename);
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

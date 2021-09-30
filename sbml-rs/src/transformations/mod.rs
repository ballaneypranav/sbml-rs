use crate::{MathTag, Model, Tag};
use mathml_rs::{self, Apply, Ci, MathNode, Op, OpNode};
use std::collections::HashMap;

pub fn transform(mut model: Model) -> Result<Model, Vec<String>> {
    // replace all S1 that have hasOnlySubstanceUnits = false with (S1 / C1)
    // so that species always refer to their amounts
    model = convert_species_to_amounts(model)?;

    // Due to this, RateRules also need to be changed
    // For example,
    // S = [S] * C
    // dS/dt = C * d[S]/dt + [S] * dC/dt
    // dS/dt = C * d[S]/dt + (S/C) * dC/dt
    // Thus,
    // modified speciesRateRule = C * speciesRateRule + (S/C) * compartmentRateRule
    model = transform_species_rate_rules(model)?;

    Ok(model)
}

// Converts all S to S/C in all MathML elements
pub fn convert_species_to_amounts(mut model: Model) -> Result<Model, Vec<String>> {
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

    let mut new_nodes = model.nodes.clone();

    for i in 0..model.nodes.len() {
        match &model.nodes[i] {
            // perform replacement in each MathTag
            Tag::MathTag(math_tag) => {
                for j in 0..math_tag.nodes.len() {
                    match &math_tag.nodes[j] {
                        // replace each Ci that refers to a species
                        // that has hasOnlySubstanceUnits = false
                        // with Species / Compartment
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
                                        let compartment_math_node =
                                            MathNode::Ci(compartment.clone());
                                        math_tag_copy.nodes[j] = apply_math_node;
                                        math_tag_copy.nodes.push(divide_math_node);
                                        math_tag_copy.nodes.push(species_math_node);
                                        math_tag_copy.nodes.push(compartment_math_node);

                                        //println!(
                                        //"Changed {} to {}/{}",
                                        //species_id, species_id, compartment
                                        //);
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

    Ok(model)
}

pub fn transform_species_rate_rules(mut model: Model) -> Result<Model, Vec<String>> {
    let species = model.species();
    let mut species_ids = Vec::<String>::new();
    for sp in &species {
        if Some(false) == sp.has_only_substance_units {
            if let Some(id) = &sp.id {
                species_ids.push(id.clone());
            }
        }
    }

    let mut species_compartment_id = HashMap::<String, String>::new();
    let mut compartment_ids = Vec::<String>::new();
    for sp in &species {
        if let Some(species_id) = &sp.id {
            if Some(false) == sp.has_only_substance_units {
                if let Some(compartment_id) = &sp.compartment {
                    species_compartment_id.insert(species_id.clone(), compartment_id.clone());
                    compartment_ids.push(compartment_id.clone());
                }
            }
        }
    }

    let rate_rules = model.rate_rules();
    let mut species_rate_rules = HashMap::<String, MathTag>::new();
    let mut compartment_rate_rules = HashMap::<String, MathTag>::new();

    for rate_rule in &rate_rules {
        // ensure valid variable and math_tag
        if let Some(var) = &rate_rule.variable {
            if let Some(math_tag) = rate_rule.math_tag(&model) {
                if species_ids.contains(var) {
                    species_rate_rules.insert(var.clone(), math_tag);
                } else if compartment_ids.contains(var) {
                    compartment_rate_rules.insert(var.clone(), math_tag);
                }
            }
        }
    }

    let mut transformed_species_rate_rules = HashMap::<String, MathTag>::new();
    for (species_id, species_rate_rule) in species_rate_rules {
        if let Some(compartment_id) = species_compartment_id.get(&species_id) {
            let transformed_species_rate_rule;
            if let Some(compartment_rate_rule) = compartment_rate_rules.get(compartment_id) {
                transformed_species_rate_rule = transform_species_rate_rule(
                    species_id.clone(),
                    species_rate_rule,
                    compartment_id.clone(),
                    Some(compartment_rate_rule.to_owned()),
                );
            } else {
                transformed_species_rate_rule = transform_species_rate_rule(
                    species_id.clone(),
                    species_rate_rule,
                    compartment_id.clone(),
                    None,
                );
            }
            transformed_species_rate_rules
                .insert(species_id.clone(), transformed_species_rate_rule);
        } else {
            panic!("Species without compartment!");
        }
    }

    for i in 0..model.nodes.len() {
        match &model.nodes[i] {
            Tag::RateRule(rate_rule) => {
                if let Some(var) = &rate_rule.variable {
                    if rate_rule.math_tag(&model).is_some() {
                        if species_ids.contains(var) {
                            if let Some(transformed_math_tag) =
                                transformed_species_rate_rules.get(var)
                            {
                                let rate_rule_math_idx = rate_rule.math.unwrap();
                                model.nodes[rate_rule_math_idx] =
                                    Tag::MathTag(transformed_math_tag.clone());
                            } else {
                                panic!("Could not transform math tag for rate rule");
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    Ok(model)
}

// Takes two MathTags for SpeciesRateRule and CompartmentRateRule and returns
// modified speciesRateRule = C * speciesRateRule + (S/C) * compartmentRateRule
pub fn transform_species_rate_rule(
    species_id: String,
    species_rate_rule: MathTag,
    compartment_id: String,
    compartment_rate_rule: Option<MathTag>,
) -> MathTag {
    let mut transformed_species_rate_rule_nodes = Vec::<MathNode>::new();
    let mut root = mathml_rs::Root::default();
    root.children = vec![1];
    transformed_species_rate_rule_nodes.push(MathNode::Root(root));

    let mut species_rate_rule_elements = species_rate_rule.nodes.clone();
    species_rate_rule_elements.remove(0);

    let a = species_rate_rule_elements.len();

    // modified speciesRateRule = C * speciesRateRule + (S/C) * compartmentRateRule
    if compartment_rate_rule.is_some() {
        // shift indices in species math node
        for math_node in &mut species_rate_rule_elements {
            math_node.shift_indices(5);
        }

        // shift indices in compartment math node
        let mut compartment_rate_rule_elements = compartment_rate_rule.unwrap().nodes.clone();
        compartment_rate_rule_elements.remove(0);
        for math_node in &mut compartment_rate_rule_elements {
            math_node.shift_indices(5 + (a as i32));
        }
        // replace Species Ci node with an Apply node and insert
        // Species Ci, Divide Op and Compartment Ci nodes at the end
        // create nodes Apply, Divide and Compartment
        let mut apply_1 = Apply::default();
        apply_1.parent = Some(0);
        apply_1.children = vec![2, 3, a + 6];
        apply_1.operator = Some(apply_1.children[0]);
        apply_1.operands = vec![3, a + 6];
        transformed_species_rate_rule_nodes.push(MathNode::Apply(apply_1));

        let mut plus_2 = OpNode::default();
        plus_2.op = Some(Op::Plus);
        plus_2.parent = Some(1);
        transformed_species_rate_rule_nodes.push(MathNode::Op(plus_2));

        let mut apply_3 = Apply::default();
        apply_3.parent = Some(1);
        apply_3.children = vec![4, 5, 6];
        apply_3.operator = Some(4);
        apply_3.operands = vec![5, 6];
        transformed_species_rate_rule_nodes.push(MathNode::Apply(apply_3));

        let mut times_4 = OpNode::default();
        times_4.op = Some(Op::Times);
        times_4.parent = Some(3);
        transformed_species_rate_rule_nodes.push(MathNode::Op(times_4));

        let mut compartment_5 = Ci::with_name(compartment_id.clone());
        compartment_5.parent = Some(3);
        transformed_species_rate_rule_nodes.push(MathNode::Ci(compartment_5));

        // nodes 6 to a + 5 are speciesRateRule
        for element in species_rate_rule_elements {
            transformed_species_rate_rule_nodes.push(element);
        }

        let mut apply_a_plus_6 = Apply::default();
        apply_a_plus_6.parent = Some(1);
        apply_a_plus_6.children = vec![a + 7, a + 8, a + 12];
        apply_a_plus_6.operator = Some(a + 7);
        apply_a_plus_6.operands = vec![a + 8, a + 12];
        transformed_species_rate_rule_nodes.push(MathNode::Apply(apply_a_plus_6));

        let mut times_a_plus_7 = OpNode::default();
        times_a_plus_7.op = Some(Op::Times);
        times_a_plus_7.parent = Some(a + 6);
        transformed_species_rate_rule_nodes.push(MathNode::Op(times_a_plus_7));

        let mut apply_a_plus_8 = Apply::default();
        apply_a_plus_8.parent = Some(1);
        apply_a_plus_8.children = vec![a + 9, a + 10, a + 11];
        apply_a_plus_8.operator = Some(a + 9);
        apply_a_plus_8.operands = vec![a + 10, a + 11];
        transformed_species_rate_rule_nodes.push(MathNode::Apply(apply_a_plus_8));

        let mut divide_a_plus_9 = OpNode::default();
        divide_a_plus_9.op = Some(Op::Divide);
        divide_a_plus_9.parent = Some(a + 8);
        transformed_species_rate_rule_nodes.push(MathNode::Op(divide_a_plus_9));

        let mut species_a_plus_10 = Ci::with_name(species_id.clone());
        species_a_plus_10.parent = Some(a + 8);
        transformed_species_rate_rule_nodes.push(MathNode::Ci(species_a_plus_10));

        let mut compartment_a_plus_11 = Ci::with_name(compartment_id.clone());
        compartment_a_plus_11.parent = Some(a + 8);
        transformed_species_rate_rule_nodes.push(MathNode::Ci(compartment_a_plus_11));

        // a + 12 wil be compartmentRateRule
        for element in compartment_rate_rule_elements {
            transformed_species_rate_rule_nodes.push(element);
        }
    }
    // modified speciesRateRule = C * speciesRateRule
    else {
        dbg!(species_id, &compartment_id);
        for math_node in &mut species_rate_rule_elements {
            math_node.shift_indices(3);
        }

        let mut apply_1 = Apply::default();
        apply_1.parent = Some(0);
        apply_1.children = vec![2, 3, 4];
        apply_1.operator = Some(2);
        apply_1.operands = vec![3, 4];
        transformed_species_rate_rule_nodes.push(MathNode::Apply(apply_1));

        let mut times_2 = OpNode::default();
        times_2.op = Some(Op::Times);
        times_2.parent = Some(1);
        transformed_species_rate_rule_nodes.push(MathNode::Op(times_2));

        let mut compartment_3 = Ci::with_name(compartment_id.clone());
        compartment_3.parent = Some(1);
        transformed_species_rate_rule_nodes.push(MathNode::Ci(compartment_3));

        // nodes 6 to a + 5 are speciesRateRule
        for element in species_rate_rule_elements {
            transformed_species_rate_rule_nodes.push(element);
        }
    }

    MathTag::default().with_nodes(transformed_species_rate_rule_nodes)
}

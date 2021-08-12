use super::math::MathTag;
use super::model::Model;
use super::tag::Tag;
use super::tag::TagIndex;
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct ListOfReactions {
    pub reactions: Vec<TagIndex>,
    pub parent: Option<TagIndex>,
}

#[derive(Debug, Clone, Default)]
pub struct Reaction {
    pub id: Option<String>,
    pub list_of_reactants: Option<TagIndex>,
    pub list_of_products: Option<TagIndex>,
    pub list_of_modifiers: Option<TagIndex>,
    pub reversible: Option<bool>,
    pub kinetic_law: Option<TagIndex>,
    pub compartment: Option<String>,
    pub name: Option<String>,
    pub sbo_term: Option<String>,
    pub parent: Option<TagIndex>,
}

impl Reaction {
    pub fn reactants(&self, model: &Model) -> Vec<SpeciesReference> {
        let mut result = Vec::new();
        if let Some(reactants_idx) = self.list_of_reactants {
            if let Tag::ListOfReactants(list_of_reactants) = &model.nodes[reactants_idx] {
                for reactant_idx in &list_of_reactants.species_references {
                    if let Tag::SpeciesReference(sp_ref) = &model.nodes[reactant_idx.to_owned()] {
                        result.push(sp_ref.clone());
                    }
                }
            }
        }
        result
    }

    pub fn reactant_ids(&self, model: &Model) -> Vec<String> {
        let reactants = &self.reactants(&model);
        reactants
            .iter()
            .map(|r| r.species.as_ref().unwrap().to_owned())
            .collect::<Vec<String>>()
    }

    pub fn products(&self, model: &Model) -> Vec<SpeciesReference> {
        let mut result = Vec::new();
        if let Some(products_idx) = self.list_of_products {
            if let Tag::ListOfProducts(list_of_products) = &model.nodes[products_idx] {
                for products_idx in &list_of_products.species_references {
                    if let Tag::SpeciesReference(sp_ref) = &model.nodes[products_idx.to_owned()] {
                        result.push(sp_ref.clone());
                    }
                }
            }
        }
        result
    }

    pub fn modifiers(&self, model: &Model) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        if let Some(lo_modifier_idx) = self.list_of_modifiers {
            if let Tag::ListOfModifiers(list_of_modifiers) = &model.nodes[lo_modifier_idx] {
                for modifier_idx in &list_of_modifiers.modifier_species_references {
                    if let Tag::ModifierSpeciesReference(modifier) =
                        &model.nodes[modifier_idx.to_owned()]
                    {
                        result.push(modifier.species.clone().expect(
                            "Species attribute is mandatory for ModifierSpeciesReferences.",
                        ));
                    }
                }
            }
        }
        result
    }

    pub fn product_ids(&self, model: &Model) -> Vec<String> {
        let products = &self.products(&model);
        products
            .iter()
            .map(|p| p.species.as_ref().unwrap().to_owned())
            .collect::<Vec<String>>()
    }

    pub fn kinetic_law(&self, model: &Model) -> Option<MathTag> {
        let mut result = None;
        if let Some(kinetic_law_idx) = self.kinetic_law {
            if let Tag::KineticLaw(kinetic_law) = &model.nodes[kinetic_law_idx] {
                if let Some(math_tag_idx) = kinetic_law.math {
                    if let Tag::MathTag(math_tag) = &model.nodes[math_tag_idx] {
                        result = Some(math_tag.clone());
                    }
                }
            }
        }
        result
    }

    pub fn local_parameters(&self, model: &Model) -> Vec<LocalParameter> {
        let mut result: Vec<LocalParameter> = Vec::new();
        if let Some(kinetic_law_idx) = self.kinetic_law {
            if let Tag::KineticLaw(kinetic_law) = &model.nodes[kinetic_law_idx] {
                result = kinetic_law.local_parameters(model);
            }
        }
        result
    }

    pub fn local_parameter_values(&self, model: &Model) -> HashMap<String, f64> {
        let mut result = HashMap::new();
        if let Some(kinetic_law_idx) = self.kinetic_law {
            if let Tag::KineticLaw(kinetic_law) = &model.nodes[kinetic_law_idx] {
                result = kinetic_law.local_parameter_values(model);
            }
        }
        result
    }
}

#[derive(Clone, Debug, Default)]
pub struct ListOfReactants {
    pub species_references: Vec<TagIndex>,
    pub parent: Option<TagIndex>,
}

#[derive(Clone, Debug, Default)]
pub struct ListOfProducts {
    pub species_references: Vec<TagIndex>,
    pub parent: Option<TagIndex>,
}

#[derive(Clone, Debug, Default)]
pub struct SpeciesReference {
    pub id: Option<String>,
    pub name: Option<String>,
    pub species: Option<String>,
    pub constant: Option<bool>,
    pub sbo_term: Option<String>,
    pub stoichiometry: Option<f64>,
    pub parent: Option<TagIndex>,
}

#[derive(Clone, Debug, Default)]
pub struct ListOfModifiers {
    pub modifier_species_references: Vec<TagIndex>,
    pub parent: Option<TagIndex>,
}

#[derive(Clone, Debug, Default)]
pub struct ModifierSpeciesReference {
    pub id: Option<String>,
    pub name: Option<String>,
    pub species: Option<String>,
    pub sbo_term: Option<String>,
    pub parent: Option<TagIndex>,
}

#[derive(Clone, Debug, Default)]
pub struct KineticLaw {
    pub math: Option<TagIndex>,
    pub list_of_local_parameters: Option<TagIndex>,
    pub parent: Option<TagIndex>,
}

impl KineticLaw {
    pub fn local_parameters(&self, model: &Model) -> Vec<LocalParameter> {
        let mut result = Vec::<LocalParameter>::new();
        if let Some(idx) = self.list_of_local_parameters {
            if let Tag::ListOfLocalParameters(list_of_local_parameters) = &model.nodes[idx] {
                for param_idx in &list_of_local_parameters.local_parameters {
                    if let Tag::LocalParameter(param) = &model.nodes[param_idx.to_owned()] {
                        result.push(param.clone());
                    }
                }
            }
        }
        result
    }

    pub fn local_parameter_values(&self, model: &Model) -> HashMap<String, f64> {
        let mut hm = HashMap::<String, f64>::new();
        for param in self.local_parameters(model) {
            if let Some(id) = param.id.to_owned() {
                if let Some(value) = param.value {
                    hm.insert(id, value);
                }
            }
        }
        hm
    }
}

#[derive(Clone, Debug, Default)]
pub struct ListOfLocalParameters {
    pub local_parameters: Vec<TagIndex>,
    pub parent: Option<TagIndex>,
}

#[derive(Clone, Debug, Default)]
pub struct LocalParameter {
    pub id: Option<String>,
    pub value: Option<f64>,
    pub units: Option<String>,
    pub sbo_term: Option<String>,
    pub parent: Option<TagIndex>,
}

#[derive(Debug)]
// used in a reaction matrix
// specifies whether a particular species
// is a reactant or a product in a particular reaction
// along with its stoichiometry
pub enum SpeciesStatus {
    Reactant(f64),
    Product(f64),
    None,
}

impl Default for SpeciesStatus {
    fn default() -> Self {
        SpeciesStatus::None
    }
}

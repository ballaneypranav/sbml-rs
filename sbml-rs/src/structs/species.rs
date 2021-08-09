use super::model::Model;
use super::tag::TagIndex;

#[derive(Debug, Default)]
pub struct ListOfSpecies {
    pub species: Vec<TagIndex>,
    pub parent: Option<TagIndex>,
}

#[derive(Debug, Default, Clone)]
pub struct Species {
    pub id: Option<String>,
    pub name: Option<String>,
    pub meta_id: Option<String>,
    pub sbo_term: Option<String>,
    pub compartment: Option<String>,
    pub initial_concentration: Option<f64>,
    pub initial_amount: Option<f64>,
    pub substance_units: Option<String>,
    pub has_only_substance_units: Option<bool>,
    pub boundary_condition: Option<bool>,
    pub constant: Option<bool>,
    pub conversion_factor: Option<String>,
    pub parent: Option<TagIndex>,
}

impl Species {
    pub fn id(&self) -> String {
        self.id.to_owned().unwrap()
    }

    pub fn compartment_size(&self, model: &Model) -> Result<f64, String> {
        let compartments = model.compartments();
        for compartment in compartments {
            let compartment_id = self.compartment.as_ref().unwrap().to_owned();
            if compartment_id == compartment.id.unwrap() {
                if let Some(compartment_size) = compartment.size {
                    return Ok(compartment_size);
                }
            }
        }
        Err("Not found".to_string())
    }
}

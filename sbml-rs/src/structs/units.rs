use super::tag::TagIndex;

#[derive(Debug)]
pub struct ListOfUnitDefinitions {
    pub unit_definitions: Vec<TagIndex>, // UnitDefinitions
    pub parent: Option<TagIndex>
}

impl ListOfUnitDefinitions {
    pub fn new() -> Self {
        ListOfUnitDefinitions {
            unit_definitions: Vec::new(),
            parent: None,
        }
    }
}

#[derive(Debug)]
pub struct UnitDefinition {
    pub id: Option<String>,
    pub list_of_units: Option<TagIndex>, // ListOfUnits
    pub parent: Option<TagIndex>
}

impl UnitDefinition {
    pub fn new() -> Self {
        UnitDefinition {
            list_of_units: None,
            id: None,
            parent: None,
        }
    }
}

#[derive(Debug)]
pub struct ListOfUnits {
    pub units: Vec<TagIndex>, // Units
    pub parent: Option<TagIndex>
}

impl ListOfUnits {
    pub fn new() -> Self {
        ListOfUnits {
            units: Vec::new(),
            parent: None
        }
    }
}

#[derive(Debug)]
pub struct Unit {
    pub kind: Option<String>, // UnitSId
    pub exponent: Option<f64>,
    pub scale: Option<i64>,
    pub multiplier: Option<f64>,
    pub parent: Option<TagIndex>
}

impl Unit {
    pub fn new() -> Self {
        Unit {
            kind: None,
            exponent: None,
            scale: None,
            multiplier: None,
            parent: None
        }
    }
}

#[derive(Debug)]
enum UnitSId {
    Ampere,
    Avogadro,
    Coulomb,
    Gray,
    Joule,
    Litre,
    Mole,
    Radian,
    Steradian,
    Weber,
    Dimensionless,
    Henry,
    Katal,
    Lumen,
    Newton,
    Tesla,
    Becquerel,
    Farad,
    Hertz,
    Kelvin,
    Lux,
    Ohm,
    Siemens,
    Volt,
    Candela,
    Gram,
    Item,
    Kilogram,
    Metre,
    Pascal,
    Sievert,
    Watt,
    Second,
}

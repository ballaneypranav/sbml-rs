use super::tag::TagIndex;

#[derive(Debug, Default)]
pub struct ListOfUnitDefinitions {
    pub unit_definitions: Vec<TagIndex>, // UnitDefinitions
    pub parent: Option<TagIndex>,
}

#[derive(Debug, Default)]
pub struct UnitDefinition {
    pub id: Option<String>,
    pub list_of_units: Option<TagIndex>, // ListOfUnits
    pub parent: Option<TagIndex>,
}

#[derive(Debug, Default)]
pub struct ListOfUnits {
    pub units: Vec<TagIndex>, // Units
    pub parent: Option<TagIndex>,
}

#[derive(Debug, Default)]
pub struct Unit {
    pub kind: Option<String>, // UnitSId
    pub exponent: Option<f64>,
    pub scale: Option<i64>,
    pub multiplier: Option<f64>,
    pub parent: Option<TagIndex>,
}

#[derive(Debug)]
#[allow(dead_code)]
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

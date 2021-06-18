pub type TagIndex = usize;
use super::compartments::*;
use super::math::*;
use super::model::*;
use super::parameters::*;
use super::reactions::*;
use super::species::*;
use super::units::*;

#[derive(Debug)]
pub enum Tag {
    Model(Model),
    ListOfUnitDefinitions(ListOfUnitDefinitions),
    UnitDefinition(UnitDefinition),
    ListOfUnits(ListOfUnits),
    Unit(Unit),
    ListOfCompartments(ListOfCompartments),
    Compartment(Compartment),
    ListOfParameters(ListOfParameters),
    Parameter(Parameter),
    ListOfSpecies(ListOfSpecies),
    Species(Species),
    ListOfReactions(ListOfReactions),
    Reaction(Reaction),
    ListOfReactants(ListOfReactants),
    ListOfProducts(ListOfProducts),
    SpeciesReference(SpeciesReference),
    KineticLaw(KineticLaw),
    MathTag(MathTag),
}

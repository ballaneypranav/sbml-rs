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
    ListOfSpecies(ListOfSpecies),
    ListOfReactions(ListOfReactions),
    Species(Species),
    Reaction(Reaction),
    KineticLaw(KineticLaw),
    MathTag(MathTag),
    ListOfUnitDefinitions(ListOfUnitDefinitions),
    UnitDefinition(UnitDefinition),
    ListOfUnits(ListOfUnits),
    Unit(Unit),
    ListOfCompartments(ListOfCompartments),
    Compartment(Compartment),
    ListOfParameters(ListOfParameters),
    Parameter(Parameter),
}

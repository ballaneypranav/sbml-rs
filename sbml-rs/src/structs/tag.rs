pub type TagIndex = usize;

use super::compartments::*;
use super::function_definitions::*;
use super::initial_assignments::*;
use super::math::*;
use super::parameters::*;
use super::reactions::*;
use super::root::*;
use super::rules::*;
use super::species::*;
use super::units::*;
use std::fmt;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Tag {
    Root(Root),
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
    ListOfModifiers(ListOfModifiers),
    ModifierSpeciesReference(ModifierSpeciesReference),
    ListOfLocalParameters(ListOfLocalParameters),
    LocalParameter(LocalParameter),
    KineticLaw(KineticLaw),
    MathTag(MathTag),
    ListOfFunctionDefinitions(ListOfFunctionDefinitions),
    FunctionDefinition(FunctionDefinition),
    ListOfInitialAssignments(ListOfInitialAssignments),
    InitialAssignment(InitialAssignment),
    ListOfRules(ListOfRules),
    AssignmentRule(AssignmentRule),
    RateRule(RateRule),
}

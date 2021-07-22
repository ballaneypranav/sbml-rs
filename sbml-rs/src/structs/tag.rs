pub type TagIndex = usize;
use super::compartments::*;
use super::function_definitions::*;
use super::initial_assignments::*;
use super::math::*;
use super::parameters::*;
use super::reactions::*;
use super::root::*;
use super::species::*;
use super::units::*;
use std::fmt;

#[allow(dead_code)]
#[derive(Debug)]
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
    KineticLaw(KineticLaw),
    MathTag(MathTag),
    ListOfFunctionDefinitions(ListOfFunctionDefinitions),
    FunctionDefinition(FunctionDefinition),
    ListOfInitialAssignments(ListOfInitialAssignments),
    InitialAssignment(InitialAssignment),
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tag::Root(node) => write!(f, "Root: {}", node),
            //Tag::ListOfUnitDefinitions(node) => write!(f, "LoUnitDef: {}", node),
            //Tag::UnitDefinition(node) => write!(f, "UnitDef: {}", node),
            //Tag::ListOfUnits(node) => write!(f, "LoUnits: {}", node),
            //Tag::Unit(node) => write!(f, "Unit: {}", node),
            //Tag::ListOfCompartments(node) => write!(f, "LoComp: {}", node),
            //Tag::Compartment(node) => write!(f, "Comp: {}", node),
            //Tag::ListOfParameters(node) => write!(f, "LoParam: {}", node),
            //Tag::Parameter(node) => write!(f, "Param: {}", node),
            //Tag::ListOfSpecies(node) => write!(f, "LoSp: {}", node),
            //Tag::Species(node) => write!(f, "Sp: {}", node),
            //Tag::ListOfReactions(node) => write!(f, "LoRxn: {}", node),
            //Tag::Reaction(node) => write!(f, "Rxn: {}", node),
            //Tag::ListOfReactants(node) => write!(f, "LoReactants: {}", node),
            //Tag::ListOfProducts(node) => write!(f, "LoProducts: {}", node),
            //Tag::SpeciesReference(node) => write!(f, "SpRef: {}", node),
            //Tag::KineticLaw(node) => write!(f, "KineticLaw: {}", node),
            Tag::MathTag(node) => writeln!(f, "Math: \n{}", node),
            //Tag::ListOfFunctionDefinitions(node) => write!(f, "LoFnDef: {}", node),
            _ => Ok(()),
        }
    }
}

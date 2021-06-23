## SBML Parser written in Rust

Supports the following SBML constructs at the moment: 
1. List of Unit Definitions, Unit Definition
1. List of Units, Unit
1. List of Compartments, Compartment
1. List of Parameters, Parameter
1. List of Species, Species
1. List of Reactions, Reaction
1. List of Reactants, List of Products
1. Species Reference
1. Kinetic Law
1. Math

Uses [mathml-rs](https://github.com/ballaneypranav/mathml-rs) for parsing math.

Only tested on the first 24 test cases from the Core Semantic SBML test suite.
To see it in action, run `cargo test` and make sure the path in the test function 
corresponds to where you have the models.

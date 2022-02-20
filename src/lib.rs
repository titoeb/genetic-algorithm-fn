#![deny(rustdoc::missing_doc_code_examples)]
#![deny(missing_docs)]
//! # Genetic algorithms for solving TSPs.
//!
//! This crates contains utitlities to run genetic algorithms and solve Traveling Salesman Problems.
/// Represent a distance Matrix as a Vec<Vec<f64>>.
pub mod function;
/// The `route`-module contains the `Route`-class, the individual element of the TSP that implements
/// important methods like `crossover` or `mutate`.
pub mod solution;
/// The `routes`-module contains the main class of this crate which is the `Routes`-class that contains
/// your current subset of routes and with which you can evolve them.
pub mod solutions;
/// Testing functions to optimize.
pub mod test_functions;
/// functions to create default objects for testing.
/// Both for convenience (not having to create them over and over again)
/// as well as standardization (test against the same common cases).
mod test_objects;

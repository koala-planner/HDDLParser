mod analyzer;
mod duplicate_detection;
mod undefined_elements;
mod cycle_detection;
mod type_checker;
mod tests;

use crate::syntactic_analyzer::*;
use crate::output::*;
use duplicate_detection::*;
use undefined_elements::*;
use type_checker::TypeChecker;

extern crate petgraph;

pub use analyzer::SemanticAnalyzer;
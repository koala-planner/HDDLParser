use std::collections::{HashMap, HashSet};

pub enum DefinitionTypes <'a> {
    Domain(&'a str),
    Problem(ProblemDefinition<'a>)
}

pub struct ProblemDefinition <'a> {
    pub problem_name: &'a str,
    pub domain_name: &'a str
}
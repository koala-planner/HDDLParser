use std::collections::{HashMap, HashSet};

pub struct TypedList <'a> {
    pub arguments: Vec<Variable<'a>>,
}

pub struct Variable<'a> {
    pub name: &'a str,
    pub var_type: Option<&'a str>  
}

impl <'a> Variable<'a> {
    pub fn new(name: &'a str, var_type: Option<&'a str>) -> Variable<'a> {
        Variable {
            name,
            var_type
        }
    }
}
use crate::lexical_analyzer::{RequirementType, TokenPosition};
use std::fmt;

#[derive(Debug)]
pub enum SemanticErrorType {
    // Duplicate Errors
    DuplicateObjectDeclaration(DuplicateError),
    DuplicateRequirementDeclaration(RequirementType),
    DuplicatePredicateDeclaration(DuplicateError),
    DuplicateActionDeclaration(DuplicateError),
    DuplicateCompoundTaskDeclaration(DuplicateError),
    DuplicateMethodDeclaration(DuplicateError),
    // Undefined Entities
    UndefinedPredicate(UndefinedSymbolError),
    UndefinedType(String),
    UndefinedSubtask(String),
    UndefinedTask(String),
    UndefinedParameter(String),
    UndefinedObject(String),
    // Inconsistency Error
    InconsistentPredicateArity(ArityError),
    InconsistentTaskArity(ArityError),
    InconsistentPredicateArgType(TypeError),
    InconsistentTaskArgType(TypeError),
    // Ordering Errors
    CyclicTypeDeclaration,
    CyclicOrderingDeclaration,
}

impl fmt::Display for SemanticErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Duplicate Errors
            SemanticErrorType::DuplicateObjectDeclaration(duplicate) => {
                write!(f, "object {}", duplicate)
            }
            SemanticErrorType::DuplicateRequirementDeclaration(req) => {
                write!(f, "requirement {}", req)
            }
            SemanticErrorType::DuplicatePredicateDeclaration(duplicate) => {
                write!(f, "predicate {}", duplicate)
            }
            SemanticErrorType::DuplicateActionDeclaration(duplicate) => {
                write!(f, "action {}", duplicate)
            }
            SemanticErrorType::DuplicateCompoundTaskDeclaration(duplicate) => {
                write!(f, "compound task {}", duplicate)
            }
            SemanticErrorType::DuplicateMethodDeclaration(duplicate) => {
                write!(f, "method {}", duplicate)
            }
            // Undefined Entities
            SemanticErrorType::UndefinedPredicate(undefined) => {
                write!(
                    f,
                    "line {}: predicate {} is not defined.",
                    undefined.position.line, undefined.symbol
                )
            }
            SemanticErrorType::UndefinedType(typ) => write!(f, "type {} is not defined.", typ),
            SemanticErrorType::UndefinedSubtask(subtask) => {
                write!(f, "subtask {} is not defined.", subtask)
            }
            SemanticErrorType::UndefinedTask(task) => write!(f, "task {} is not defined.", task),
            SemanticErrorType::UndefinedParameter(param) => {
                write!(f, "parameter {} is not defined.", param)
            }
            SemanticErrorType::UndefinedObject(object) => {
                write!(f, "object {} is not defined.", object)
            }
            // Inconsistency Error
            SemanticErrorType::InconsistentPredicateArity(ar_error) => {
                write!(
                    f,
                    "line {}: predicate {} takes {} parameters, but {} are given.",
                    ar_error.position.line, ar_error.symbol, ar_error.expected_arity, ar_error.found_arity
                )
            }
            SemanticErrorType::InconsistentTaskArity(ar_error) => {
                write!(
                    f,
                    "Task {} takes {} parameters, but {} are given.",
                    ar_error.symbol, ar_error.expected_arity, ar_error.found_arity
                )
            }
            SemanticErrorType::InconsistentPredicateArgType(type_error) => {
                write!(f, "{}", type_error)
            }
            SemanticErrorType::InconsistentTaskArgType(type_error) => write!(f, "{}", type_error),
            // Ordering Errors
            SemanticErrorType::CyclicTypeDeclaration => write!(f, "Cyclic type declaration"),
            SemanticErrorType::CyclicOrderingDeclaration => {
                write!(f, "Cyclic ordering declaration")
            }
        }
    }
}

#[derive(Debug)]
pub struct TypeError {
    pub expected: Option<String>,
    pub found: Option<String>,
    pub var_name: String,
}

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Type error for variable {}. ", self.var_name)?;
        match (&self.expected, &self.found) {
            (Some(expected), Some(found)) => {
                write!(
                    f,
                    "Expected object of type '{}', but found '{}'.",
                    expected, found
                )
            }
            (Some(expected), None) => {
                write!(
                    f,
                    "Expected object of type '{}', but did not find any typing.",
                    expected
                )
            }
            (None, Some(found)) => {
                write!(f, "Expected no type, but found '{}'.", found)
            }
            (None, None) => {
                unreachable!()
            }
        }
    }
}

#[derive(Debug)]
pub struct ArityError {
    pub symbol: String,
    pub expected_arity: u32,
    pub found_arity: u32,
    pub position: TokenPosition,
}

#[derive(Debug)]
pub struct DuplicateError {
    pub symbol: String,
    pub first_pos: TokenPosition,
    pub second_pos: TokenPosition,
}

impl fmt::Display for DuplicateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "'{}' is first defined in line {}, and then redefined in line {}.",
            self.symbol, self.first_pos.line, self.second_pos.line
        )
    }
}

#[derive(Debug)]
pub struct UndefinedSymbolError {
    pub symbol: String,
    pub position: TokenPosition,
}

use std::collections::HashSet;

use cycle_detection::check_ordering_acyclic;
use petgraph::{algo::toposort, prelude::GraphMap, Directed};

use super::*;

pub fn verify_semantics<'a>(ast: &'a SyntaxTree<'a>) -> Result<(), SemanticError<'a>> {
    let _ = check_duplicate_objects(&ast.objects)?;
    let _ = check_duplicate_requirements(&ast.requirements)?;

    let mut types: Option<GraphMap<&str, (), Directed>> = None;
    match &ast.types {
        None => {},
        Some(typing) => {
            let mut type_graph = GraphMap::<_, (), Directed>::new();
            for delcared_type in typing {
                if !type_graph.contains_node(delcared_type.name) {
                    type_graph.add_node(delcared_type.name);
                }
                match &delcared_type.var_type {
                    None => {},
                    Some(parent) => {
                        if !type_graph.contains_node(parent) {
                            type_graph.add_node(parent);
                        }
                        type_graph.add_edge(delcared_type.name, parent, ());
                    }
                }
            }
            match toposort(&type_graph, None) {
                Ok(_) => {},
                Err(cycle_item) => {
                    return Err(SemanticError::CyclicTypeDeclaration(cycle_item.node_id()));
                }
            }
            types = Some(type_graph);
        }
    }
    
    // assert predicates are correct
    let mut declared_predicates = HashSet::new();
    for predicate in ast.predicates.iter() {
        if !declared_predicates.insert(predicate.name) {
            return Err(SemanticError::DuplicatePredicateDeclaration(&predicate.name));
        }
        let _ = check_type_declarations(&predicate.variables, &types)?;
    }

    // assert compound tasks are correct
    let mut declared_tasks = HashSet::new();
    for task in ast.compound_tasks.iter() {
        if !declared_tasks.insert(task.name) {
            return Err(SemanticError::DuplicateCompoundTaskDeclaration(task.name));
        }
        // assert parameter types are declared
        let _ = check_type_declarations(&task.parameters, &types)?;
    }

    // assert actions are correct
    let mut declared_actions = HashSet::new();
    for action in ast.actions.iter() {
        if !declared_actions.insert(action.name) {
            return Err(SemanticError::DuplicateActionDeclaration(action.name));
        }
        // assert parameter types are declared
        let _ = check_type_declarations(&action.parameters, &types)?;
        // assert precondition predicates are declared
        match &action.preconditions {
            Some(precondition) => {
                check_predicate_declarations(precondition, &ast.predicates)?;
            }
            _ => {}
        }
        // assert effect predicates are declared
        match &action.effects {
            Some(effect) => {
                check_predicate_declarations(effect, &ast.predicates)?;
            }
            _ => {}
        }
    }

    // assert methods are correct
    let mut declared_methods = HashSet::new();
    for method in ast.methods.iter() {
        if !declared_methods.insert(method.name) {
            return Err(SemanticError::DuplicateMethodDeclaration(method.name));
        }
        // assert parameter types are declared
        let _ = check_type_declarations(&method.params, &types)?;
        // Assert preconditions are valid
        match &method.precondition {
            Some(precondition) => {
                check_predicate_declarations(precondition, &ast.predicates)?;
            }
            _ => {}
        }
        let mut is_method_task_declared = false;
        for declared_compound_task in ast.compound_tasks.iter() {
            if method.task_name == declared_compound_task.name {
                if method.task_terms.len() != declared_compound_task.parameters.len() {
                    return Err(SemanticError::InconsistentTaskArity(&method.task_name));
                } else {
                    is_method_task_declared = true;
                    break;
                }
            }
        }
        if !is_method_task_declared {
            return Err(SemanticError::UndefinedTask(&method.task_name));
        }
        // Assert subtasks are valid
        check_subtask_declarations(&method.tn.subtasks, &ast.compound_tasks, &ast.actions)?;
        // Assert orderings are acyclic
        check_ordering_acyclic(&method.tn)?;
    }
    Ok(())
}

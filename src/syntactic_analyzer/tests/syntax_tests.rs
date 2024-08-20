use super::*;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    #[ignore = "stupid test, rewrite from scratch"]
    pub fn file_type_test() {
        let program = String::from("(define (domain jajaja) (:predicates ()) ) ").into_bytes();
        let lexer = LexicalAnalyzer::new(program);
        let parser = Parser::new(&lexer);
        match parser.parse() {
            Ok(_) => {}
            _ => panic!("parsing error"),
        }
        let program = String::from("(define (problem jajaja2) (domain blahblah)) ").into_bytes();
        let lexer = LexicalAnalyzer::new(program);
        let parser = Parser::new(&lexer);
        match parser.parse() {
            Ok(_) => {}
            _ => panic!("parsing error"),
        }
    }

    #[test]
    pub fn objects_list_test() {
        let program =
            String::from("(define (problem p1) (domain bal)
                            (:objects a b c - d s - f t)
                          )")
                .into_bytes();
        let lexer = LexicalAnalyzer::new(program);
        match Parser::new(&lexer).parse() {
            Ok(symbols) => {
                assert_eq!(symbols.objects[0].name, "a");
                assert_eq!(symbols.objects[0].var_type.unwrap(), "d");
                assert_eq!(symbols.objects[1].name, "b");
                assert_eq!(symbols.objects[1].var_type.unwrap(), "d");
                assert_eq!(symbols.objects[2].name, "c");
                assert_eq!(symbols.objects[2].var_type.unwrap(), "d");
                assert_eq!(symbols.objects[3].name, "s");
                assert_eq!(symbols.objects[3].var_type.unwrap(), "f");
                assert_eq!(symbols.objects[4].name, "t");
                assert_eq!(symbols.objects[4].var_type.is_none(), true);
            }
            Err(_) => panic!("parsing errors"),
        }
    }

    #[test]
    pub fn untyped_objects_list_test() {
        let program =
            String::from("(define (problem p1) (domain bal) (:objects a b c) )").into_bytes();
        let lexer = LexicalAnalyzer::new(program);
        match Parser::new(&lexer).parse() {
            Ok(symbols) => {
                assert_eq!(symbols.objects[0].name, "a");
                assert_eq!(symbols.objects[1].name, "b");
                assert_eq!(symbols.objects[2].name, "c");
            }
            Err(_) => panic!("parsing errors"),
        }
    }

    #[test]
    pub fn requirement_parsing_test() {
        let program = String::from(
            "(define (problem p1) (domain bal)
                (:requirements :hierarchy :method-preconditions :typing :negative-preconditions)

             ) ",
        )
        .into_bytes();
        let lexer = LexicalAnalyzer::new(program);
        match Parser::new(&lexer).parse() {
            Ok(symbols) => {
                assert_eq!(symbols.requirements.len(), 4);
                assert_eq!(
                    symbols.requirements.contains(&RequirementType::Hierarchy),
                    true
                );
                assert_eq!(
                    symbols
                        .requirements
                        .contains(&RequirementType::MethodPreconditions),
                    true
                );
                assert_eq!(
                    symbols
                        .requirements
                        .contains(&RequirementType::NegativePreconditions),
                    true
                );
                assert_eq!(
                    symbols
                        .requirements
                        .contains(&RequirementType::TypedObjects),
                    true
                );
            }
            Err(_) => panic!("parsing errors"),
        }
    }

    #[test]
    pub fn predicate_parsing_test() {
        let program = String::from(
            "(define (domain bal)
                (:predicates 
                    (pred_1 ?a_1 ?a_2 - t_1 ?a_3 - t_2)
                    (pred_2)
                    (pred_3 a_1 a_2)
                )
             ) ",
        )
        .into_bytes();
        let lexer = LexicalAnalyzer::new(program);
        match Parser::new(&lexer).parse() {
            Ok(symbols) => {
                assert_eq!(symbols.predicates.len(), 3);
                for predicate in symbols.predicates {
                    let items: Vec<(&str, Option<&str>)> = predicate
                        .variables
                        .iter()
                        .map(|x| (x.name, x.var_type))
                        .collect();
                    if predicate.name == "pred_1" {
                        assert_eq!(
                            items,
                            vec![
                                ("a_1", Some("t_1")),
                                ("a_2", Some("t_1")),
                                ("a_3", Some("t_2"))
                            ]
                        );
                    } else if predicate.name == "pred_2" {
                        assert_eq!(predicate.variables.len(), 0);
                    } else if predicate.name == "pred_3" {
                        let items: Vec<(&str, Option<&str>)> = predicate
                            .variables
                            .iter()
                            .map(|x| (x.name, x.var_type))
                            .collect();
                        assert_eq!(items, vec![("a_1", None), ("a_2", None)]);
                    } else {
                        panic!("parsing error")
                    }
                }
            }
            Err(_) => panic!("parsing errors"),
        }
    }

    #[test]
    pub fn method_parsing_test() {
        let program = String::from(
            "(define (domain bal)
                (:method m_1
                    :parameters (?p1 - p ?l1 ?l2 ?l3 - loc) 
                    :task (deliver_abs ?p1 ?l1 ?l2)
                    :subtasks (and
                        (pickup ?p1 ?l1)
                        (deliver_abs ?p1 ?l2 ?l3)
                    )
                )
             ) ",
        )
        .into_bytes();
        let lexer = LexicalAnalyzer::new(program);
        match Parser::new(&lexer).parse() {
            Ok(ast) => {
                assert_eq!(ast.methods.len(), 1);
                let method = &ast.methods[0];
                assert_eq!(method.name, "m_1");
                assert_eq!(method.task_name, "deliver_abs");
                assert_eq!(method.task_terms.len(), 3);
                assert_eq!(method.task_terms[0].name, "p1");
                assert_eq!(method.task_terms[1].name, "l1");
                assert_eq!(method.task_terms[2].name, "l2");
                assert_eq!(method.tn.subtasks[0].task_symbol, "pickup");
                assert_eq!(method.tn.subtasks[0].terms[0], "p1");
                assert_eq!(method.tn.subtasks[0].terms[1], "l1");
                assert_eq!(method.tn.subtasks[1].task_symbol, "deliver_abs");
                assert_eq!(method.tn.subtasks[1].terms[0], "p1");
                assert_eq!(method.tn.subtasks[1].terms[1], "l2");
                assert_eq!(method.tn.subtasks[1].terms[2], "l3");
                assert_eq!(method.precondition.is_none(), true);
            }
            _ => panic!("AST not created"),
        }
    }

    #[test]
    pub fn method_precondition_test() {
        let program = String::from(
            "(define (domain bal)
                (:method m_1
                    :parameters (?p1 - p ?l1 ?l2 ?l3 - loc) 
                    :task (deliver_abs ?p1 ?l1 ?l2)
                    :precondition (and (at ?p1 ?l3) (driver ?l1) (not (= ?l1 ?l2)))
                    :subtasks (and
                        (pickup ?p1 ?l1)
                        (deliver_abs ?p1 ?l2 ?l3)
                    )
                )
             ) ",
        )
        .into_bytes();
        let lexer = LexicalAnalyzer::new(program);
        match Parser::new(&lexer).parse() {
            Ok(ast) => {
                assert_eq!(ast.methods.len(), 1);
                let method = &ast.methods[0];
                assert_eq!(method.name, "m_1");
                assert_eq!(method.task_name, "deliver_abs");
                assert_eq!(method.task_terms.len(), 3);
                assert_eq!(method.task_terms[0].name, "p1");
                assert_eq!(method.task_terms[1].name, "l1");
                assert_eq!(method.task_terms[2].name, "l2");
                assert_eq!(method.tn.subtasks[0].task_symbol, "pickup");
                assert_eq!(method.tn.subtasks[0].terms[0], "p1");
                assert_eq!(method.tn.subtasks[0].terms[1], "l1");
                assert_eq!(method.tn.subtasks[1].task_symbol, "deliver_abs");
                assert_eq!(method.tn.subtasks[1].terms[0], "p1");
                assert_eq!(method.tn.subtasks[1].terms[1], "l2");
                assert_eq!(method.tn.subtasks[1].terms[2], "l3");
                match &method.precondition {
                    Some(formula) => {
                        match formula {
                            Formula::And(predicates) => {
                                assert_eq!(predicates.len(), 3);
                                let pred1 = &*predicates[0];
                                match pred1 {
                                    Formula::Atom(pred) => {
                                        assert_eq!(pred.name, "at");
                                        assert_eq!(pred.variables.len(), 2);
                                    },
                                    _ => {
                                        panic!("wrong formula parsing")
                                    }
                                }
                                let pred2 = &*predicates[1];
                                match pred2 {
                                    Formula::Atom(pred) => {
                                        assert_eq!(pred.name, "driver");
                                        assert_eq!(pred.variables.len(), 1);
                                    },
                                    _ => {
                                        panic!("wrong formula parsing")
                                    }
                                }

                                let neq = &*predicates[2];
                                match neq {
                                    Formula::Not(equality) => {
                                        match **equality {
                                            Formula::Equals(a, b) => {
                                                assert_eq!(a, "l1");
                                                assert_eq!(b, "l2");
                                            }
                                            _ => { panic!("equality constraint not parsed successfully")}
                                        }
                                    },
                                    _ => {
                                        panic!("wrong formula parsing")
                                    }
                                }
                            }
                            _ => {
                                panic!("wrong formula parsing")
                            }
                        }
                    }
                    _ => {
                        panic!("wrong formula parsing")
                    }
                }
            }
            _ => panic!("AST not created"),
        }
    }

    #[test]
    pub fn universal_quantification_test() {
        let program = String::from(
            "(define (domain bal)
                (:method m_1
                    :parameters (?p1 - p ?l1 ?l2 ?l3 - loc) 
                    :task (deliver_abs ?p1 ?l1 ?l2)
                    :precondition (forall (?l1 ?l2 - loc) (= ?l1 ?l2))
                    :subtasks (and
                        (pickup ?p1 ?l1)
                        (deliver_abs ?p1 ?l2 ?l3)
                    )
                )
             ) ",
        )
        .into_bytes();
        let lexer = LexicalAnalyzer::new(program);
        match Parser::new(&lexer).parse() {
            Ok(ast) => {
                assert_eq!(ast.methods.len(), 1);
                let method = &ast.methods[0];
                assert_eq!(method.name, "m_1");
                assert_eq!(method.task_name, "deliver_abs");
                assert_eq!(method.task_terms.len(), 3);
                assert_eq!(method.task_terms[0].name, "p1");
                assert_eq!(method.task_terms[1].name, "l1");
                assert_eq!(method.task_terms[2].name, "l2");
                match &method.precondition {
                    Some(formula) => {
                        match formula {
                            Formula::ForAll(params, exp) => {
                                assert_eq!(params.len(), 2);
                                assert_eq!(params[0].name, "l1");
                                match params[0].var_type {
                                    Some(x) => {
                                        assert_eq!(x, "loc");
                                    }
                                    _ => { panic!("wrong parameter type") }
                                }
                                assert_eq!(params[1].name, "l2");
                                match params[1].var_type {
                                    Some(x) => {
                                        assert_eq!(x, "loc");
                                    }
                                    _ => { panic!("wrong parameter type") }
                                }
                                match **exp {
                                    Formula::Equals(a,b ) => {
                                        assert_eq!(a, "l1");
                                        assert_eq!(b, "l2");
                                    }
                                    _ => {
                                        panic!("wrong expression parsing")
                                    }
                                }
                            }
                            _ => {
                                panic!("wrong formula parsing")
                            }
                        }
                    }
                    _ => {
                        panic!("wrong formula parsing")
                    }
                }
            }
            _ => panic!("AST not created"),
        }
    }

    #[test]
    pub fn init_state_parsing_test() {
        let program = String::from(
            "(define (problem p1) (domain bal)
             (:init
                (pred1 arg1 arg2)
                (pred2 arg1 arg2 arg3)
            ) ",
        )
        .into_bytes();
        let lexer = LexicalAnalyzer::new(program);
        match Parser::new(&lexer).parse() {
            Ok(ast) => {
                assert_eq!(ast.init_state.len(), 2);
                let pred1 = &ast.init_state[0];
                let pred2 = &ast.init_state[1];
                assert_eq!(pred1.name, "pred1");
                assert_eq!(pred1.variables.len(), 2);
                assert_eq!(pred1.variables[0].name, "arg1");
                assert_eq!(pred1.variables[1].name, "arg2");
                assert_eq!(pred2.name, "pred2");
                assert_eq!(pred2.variables.len(), 3);
                assert_eq!(pred2.variables[0].name, "arg1");
                assert_eq!(pred2.variables[1].name, "arg2");
                assert_eq!(pred2.variables[2].name, "arg3");
            }
            _ => {
                panic!("wrong AST")
            }
        }
    }

    #[test]
    pub fn init_tn_parsing_test() {
        let program = String::from(
            "(define (problem p1) (domain bal)
             (:htn
                :parameters (?d)
                :subtasks (and
                    (task0 (deliver package_0 city_loc_0))
                    (task1 (retrieve package_1 city_loc_2 truck3))
                )
                :ordering (and
                    (< task0 task1)
                )
                :constraints (not (= term1 term2))
            ) ",
        )
        .into_bytes();
        let lexer = LexicalAnalyzer::new(program);
        match Parser::new(&lexer).parse() {
            Ok(ast) => match ast.init_tn {
                Some(tn) => {
                    match tn.parameters {
                        Some(p) => {
                            assert_eq!(p.len(), 1);
                            assert_eq!(p[0].name, "d");
                            assert_eq!(p[0].var_type.is_none(), true);
                        }
                        _ => panic!("wrong set of params")
                    }
                    match tn.tn.orderings {
                        TaskOrdering::Partial(o) => {
                            assert_eq!(o.contains(&("task0", "task1")), true);
                            assert_eq!(o.len(), 1);
                        }
                        _ => {
                            panic!("ordering is not total")
                        }
                    }
                    assert_eq!(tn.tn.subtasks.len(), 2);
                    assert_eq!(tn.tn.subtasks[0].id, Some("task0"));
                    assert_eq!(tn.tn.subtasks[0].task_symbol, "deliver");
                    assert_eq!(tn.tn.subtasks[0].terms.len(), 2);
                    assert_eq!(tn.tn.subtasks[0].terms[0], "package_0");
                    assert_eq!(tn.tn.subtasks[0].terms[1], "city_loc_0");
                    assert_eq!(tn.tn.subtasks[1].id, Some("task1"));
                    assert_eq!(tn.tn.subtasks[1].task_symbol, "retrieve");
                    assert_eq!(tn.tn.subtasks[1].terms.len(), 3);
                    assert_eq!(tn.tn.subtasks[1].terms[0], "package_1");
                    assert_eq!(tn.tn.subtasks[1].terms[1], "city_loc_2");
                    assert_eq!(tn.tn.subtasks[1].terms[2], "truck3");

                    match tn.tn.constraints {
                        Some(constraint) => {
                            assert_eq!(constraint.len(), 1);
                            match constraint[0] {
                                Constraint::NotEqual("term1", "term2") => {},
                                _ => { panic!("constraint not parsed correctly")}
                            }
                        }
                        _ => {
                            panic!("constraints are not parsed")
                        }
                    }
                }
                None => {
                    panic!("init tn not parsed")
                }
            },
            _ => {
                panic!("failed to create AST")
            }
        }
    }

    #[test]
    pub fn init_total_order_tn_parsing_test() {
        let program = String::from(
            "(define (problem p1) (domain bal)
             (:htn
                :ordered-tasks (and
                    (deliver package_0 city_loc_0)
                    (retrieve package_1 city_loc_2 truck3)
                )
            ) ",
        )
        .into_bytes();
        let lexer = LexicalAnalyzer::new(program);
        match Parser::new(&lexer).parse() {
            Ok(ast) => match ast.init_tn {
                Some(tn) => {
                    assert_eq!(tn.parameters.is_none(), true);
                    match tn.tn.orderings {
                        TaskOrdering::Total => {}
                        _ => {
                            panic!("ordering is not partial")
                        }
                    }
                    assert_eq!(tn.tn.subtasks.len(), 2);
                    assert_eq!(tn.tn.subtasks[0].id, None);
                    assert_eq!(tn.tn.subtasks[0].task_symbol, "deliver");
                    assert_eq!(tn.tn.subtasks[0].terms.len(), 2);
                    assert_eq!(tn.tn.subtasks[0].terms[0], "package_0");
                    assert_eq!(tn.tn.subtasks[0].terms[1], "city_loc_0");
                    assert_eq!(tn.tn.subtasks[1].id, None);
                    assert_eq!(tn.tn.subtasks[1].task_symbol, "retrieve");
                    assert_eq!(tn.tn.subtasks[1].terms.len(), 3);
                    assert_eq!(tn.tn.subtasks[1].terms[0], "package_1");
                    assert_eq!(tn.tn.subtasks[1].terms[1], "city_loc_2");
                    assert_eq!(tn.tn.subtasks[1].terms[2], "truck3");
                }
                None => panic!("tn not found"),
            },
            _ => panic!("false parsing"),
        }
    }

    #[test]
    pub fn compound_task_parsing_test() {
        let program = String::from(
            "(define (domain bal)
                (:task c_1
                 :parameters (p_1 p_2 - t1 p_3 - t2)
                )
             ) ",
        )
        .into_bytes();
        let lexer = LexicalAnalyzer::new(program);
        match Parser::new(&lexer).parse() {
            Ok(ast) => {
                assert_eq!(ast.compound_tasks.len(), 1);
                let c_1 = &ast.compound_tasks[0];
                assert_eq!(c_1.name, "c_1");
                let c1_term_names: Vec<&str> =
                    c_1.parameters.iter().map(|x| x.name).collect();
                let c1_term_types: Vec<&str> = c_1
                    .parameters
                    .iter()
                    .map(|x| x.var_type.unwrap())
                    .collect();
                assert_eq!(c1_term_names, vec!["p_1", "p_2", "p_3"]);
                assert_eq!(c1_term_types, vec!["t1", "t1", "t2"]);
            }
            Err(_) => panic!("parsing errors"),
        }
    }

    #[test]
    pub fn action_parsing_test() {
        let program = String::from(
            "(define (domain bal)
                (:action a_1
                 :parameters (p_1 p_2 - t1 p_3 - t2)
                 :precondition (not (at p_1))
                 :effect (and (not (hold p_2 p_3)) (at p_2))
                )
             ) ",
        )
        .into_bytes();
        let lexer = LexicalAnalyzer::new(program);
        match Parser::new(&lexer).parse() {
            Ok(ast) => {
                assert_eq!(ast.actions.len(), 1);
                let action = &ast.actions[0];
                assert_eq!(action.name, "a_1");
                let a1_vars: Vec<&str> =
                    action.parameters.iter().map(|x| x.name).collect();
                let a1_var_types: Vec<&str> = action
                    .parameters                    
                    .iter()
                    .map(|x| x.var_type.unwrap())
                    .collect();
                assert_eq!(a1_vars, vec!["p_1", "p_2", "p_3"]);
                assert_eq!(a1_var_types, vec!["t1", "t1", "t2"]);
                match &action.preconditions.as_ref().unwrap() {
                    Formula::Not(formula) => match &**formula {
                        Formula::Atom(predicate) => {
                            assert_eq!(predicate.name, "at");
                            assert_eq!(predicate.variables.len(), 1);
                            assert_eq!(predicate.variables[0].name, "p_1");
                        }
                        _ => {
                            panic!("wrong formula")
                        }
                    },
                    _ => panic!("wrong formula"),
                }
                match &action.effects.as_ref().unwrap() {
                    Formula::And(formula) => {
                        assert_eq!(formula.len(), 2);
                        if let Formula::Not(exp) = formula[0].as_ref() {
                            if let Formula::Atom(pred) = exp.as_ref() {
                                assert_eq!(pred.name, "hold");
                                assert_eq!(pred.variables.len(), 2);
                                assert_eq!(pred.variables[0].name, "p_2");
                                assert_eq!(pred.variables[1].name, "p_3");
                            } else {
                                panic!("wrong formula")
                            }
                        } else {
                            panic!("wrong formula")
                        };
                        if let Formula::Atom(pred) = formula[1].as_ref() {
                            assert_eq!(pred.name, "at");
                            assert_eq!(pred.variables.len(), 1);
                            assert_eq!(pred.variables[0].name, "p_2");
                        } else {
                            panic!("wrong formula")
                        }
                    }
                    _ => panic!("wrong formula"),
                }
            }
            Err(_) => panic!("parsing errors"),
        }
    }

    #[test]
    pub fn non_deterministic_action_parsing_test() {
        let program = String::from(
            "(define (domain bal)
                (:action a_1
                 :parameters (p_1 p_2 - t1 p_3 - t2)
                 :precondition (at p1)
                 :effect (oneof (not (hold p_2 p_3)) (at p_2))
                )
             ) ",
        )
        .into_bytes();
        let lexer = LexicalAnalyzer::new(program);
        match Parser::new(&lexer).parse() {
            Ok(ast) => {
                assert_eq!(ast.actions.len(), 1);
                let action = &ast.actions[0];
                assert_eq!(action.name, "a_1");
                let a1_vars: Vec<&str> =
                    action.parameters.iter().map(|x| x.name).collect();
                let a1_var_types: Vec<&str> = action
                    .parameters                    
                    .iter()
                    .map(|x| x.var_type.unwrap())
                    .collect();
                assert_eq!(a1_vars, vec!["p_1", "p_2", "p_3"]);
                assert_eq!(a1_var_types, vec!["t1", "t1", "t2"]);
                match &action.effects.as_ref().unwrap() {
                    Formula::Xor(formula) => {
                        assert_eq!(formula.len(), 2);
                        if let Formula::Not(exp) = formula[0].as_ref() {
                            if let Formula::Atom(pred) = exp.as_ref() {
                                assert_eq!(pred.name, "hold");
                                assert_eq!(pred.variables.len(), 2);
                                assert_eq!(pred.variables[0].name, "p_2");
                                assert_eq!(pred.variables[1].name, "p_3");
                            } else {
                                panic!("wrong formula")
                            }
                        } else {
                            panic!("wrong formula")
                        };
                        if let Formula::Atom(pred) = formula[1].as_ref() {
                            assert_eq!(pred.name, "at");
                            assert_eq!(pred.variables.len(), 1);
                            assert_eq!(pred.variables[0].name, "p_2");
                        } else {
                            panic!("wrong formula")
                        }
                    }
                    _ => panic!("wrong formula"),
                }
            }
            Err(_) => panic!("parsing errors"),
        }
    }

    #[test]
    pub fn var_type_declaration_test() {
        let program = String::from(
            "(define (domain blahblah)
                (:types
                    Port AbstractDevice - Object
                    AbstractCable Device - AbstractDevice
                    PlugType PlugFace PlugDirection SignalType - Enum
                )
             ) ",
        )
        .into_bytes();
        let lexer = LexicalAnalyzer::new(program);
        match Parser::new(&lexer).parse() {
            Ok(ast) => {
                let types = ast.types.unwrap();
                assert_eq!(types.len(), 8);
                assert_eq!(types[0].name, "Port");
                assert_eq!(types[0].var_type.unwrap(), "Object");
                assert_eq!(types[1].name, "AbstractDevice");
                assert_eq!(types[1].var_type.unwrap(), "Object");
                assert_eq!(types[2].name, "AbstractCable");
                assert_eq!(types[2].var_type.unwrap(), "AbstractDevice");
                assert_eq!(types[3].name, "Device");
                assert_eq!(types[3].var_type.unwrap(), "AbstractDevice");
                assert_eq!(types[4].name, "PlugType");
                assert_eq!(types[4].var_type.unwrap(), "Enum");
                assert_eq!(types[5].name, "PlugFace");
                assert_eq!(types[5].var_type.unwrap(), "Enum");
                assert_eq!(types[6].name, "PlugDirection");
                assert_eq!(types[6].var_type.unwrap(), "Enum");
                assert_eq!(types[7].name, "SignalType");
                assert_eq!(types[7].var_type.unwrap(), "Enum");
            }
            _ => panic!("parsing erro")
        }
    }


    #[test]
    pub fn comment_test() {
        let program = String::from(
            ";author: me
            ; domain bal
            (define (domain bal)
                ;task c_1
                (:task c_1
                 :parameters (p_1 p_2 - t1 p_3 - t2) ;task parameters are defined here
                )
             ) ",
        )
        .into_bytes();
        let lexer = LexicalAnalyzer::new(program);
        match Parser::new(&lexer).parse() {
            Ok(ast) => {
                assert_eq!(ast.compound_tasks.len(), 1);
                let c_1 = &ast.compound_tasks[0];
                assert_eq!(c_1.name, "c_1");
                let c1_term_names: Vec<&str> =
                    c_1.parameters.iter().map(|x| x.name).collect();
                let c1_term_types: Vec<&str> = c_1
                    .parameters
                    .iter()
                    .map(|x| x.var_type.unwrap())
                    .collect();
                assert_eq!(c1_term_names, vec!["p_1", "p_2", "p_3"]);
                assert_eq!(c1_term_types, vec!["t1", "t1", "t2"]);
            }
            Err(_) => panic!("parsing errors"),
        }
    }

    #[test]
    pub fn constants_declaration_test() {
        let program = String::from(
            "(define (domain blahblah)
                (:constants
                    Port AbstractDevice - Object
                    AbstractCable Device - AbstractDevice
                    PlugType PlugFace PlugDirection SignalType - Enum
                )
             ) ",
        )
        .into_bytes();
        let lexer = LexicalAnalyzer::new(program);
        match Parser::new(&lexer).parse() {
            Ok(ast) => {
                let constants = ast.constants.unwrap();
                assert_eq!(constants.len(), 8);
                assert_eq!(constants[0].name, "Port");
                assert_eq!(constants[0].var_type.unwrap(), "Object");
                assert_eq!(constants[1].name, "AbstractDevice");
                assert_eq!(constants[1].var_type.unwrap(), "Object");
                assert_eq!(constants[2].name, "AbstractCable");
                assert_eq!(constants[2].var_type.unwrap(), "AbstractDevice");
                assert_eq!(constants[3].name, "Device");
                assert_eq!(constants[3].var_type.unwrap(), "AbstractDevice");
                assert_eq!(constants[4].name, "PlugType");
                assert_eq!(constants[4].var_type.unwrap(), "Enum");
                assert_eq!(constants[5].name, "PlugFace");
                assert_eq!(constants[5].var_type.unwrap(), "Enum");
                assert_eq!(constants[6].name, "PlugDirection");
                assert_eq!(constants[6].var_type.unwrap(), "Enum");
                assert_eq!(constants[7].name, "SignalType");
                assert_eq!(constants[7].var_type.unwrap(), "Enum");
            }
            _ => panic!("parsing erro")
        }
    }
}

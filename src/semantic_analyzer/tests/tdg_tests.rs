use std::collections::HashSet;

use tdg::TDG;

use super::*;

#[test]
pub fn tdg_correctness_test () {
    let program = String::from(
        "(define (domain bal)
            (:predicates 
                (at ?l)
            )
            (:action p_1
            :parameters(?l1)
            :precondition (at ?l1)
            )
            (:action p_2
            :parameters(?l1)
            :precondition (at ?l1)
            )
            (:task abs_1 :parameters(?a))
            (:task abs_2 :parameters(?a))
            (:task abs_3 :parameters(?a))

            (:method m_1
                :parameters (?p1) 
                :task (abs_1 ?p1)
                :ordered-subtasks (and
                    (t1 (abs_1 ?p1))
                    (t2 (abs_2 ?p1))
                    (t3 (abs_3 ?p1))
                    (t4 (p_1 ?p1))
                )
            )
        ) ",
    )
    .into_bytes();
    let problem = String::from("
        (define (problem p-1-2-2)
            (:domain barman_htn)
            (:htn
                :parameters ()
                :ordered-subtasks (and
                    (abs_1)
                )
            )
    ").into_bytes();
    let lexer = LexicalAnalyzer::new(&program);
    let parser = Parser::new(lexer);
    let ast = parser.parse().unwrap();
    match ast {
        AbstractSyntaxTree::Domain(d) => {
            let p_lexer = LexicalAnalyzer::new(&problem);
            let p_parser = Parser::new(p_lexer);
            let p_ast = p_parser.parse().unwrap();
            match p_ast {
                AbstractSyntaxTree::Problem(p_ast) => {
                    let tdg = TDG::new(&d);
                    let reachable_abs_1 = tdg.reachable("abs_1");
                    assert_eq!(reachable_abs_1.len(), 4);
                    assert_eq!(reachable_abs_1.contains("abs_1"), true);
                    assert_eq!(reachable_abs_1.contains("abs_2"), true);
                    assert_eq!(reachable_abs_1.contains("abs_3"), true);
                    assert_eq!(reachable_abs_1.contains("p_1"), true);

                    let reachable_p_2 = tdg.reachable("p_2");
                    assert_eq!(reachable_p_2.len(), 1);
                    assert_eq!(reachable_p_2.contains("p_2"), true);
                }
                _ => panic!()
            }
        }
        AbstractSyntaxTree::Problem(_) => panic!()
    }
}


#[test]
pub fn tdg_non_recursive_test () {
    let program = String::from(
        "(define (domain bal)
            (:predicates 
                (at ?l)
            )
            (:action p_1
            :parameters(?l1)
            :precondition (at ?l1)
            )
            (:action p_2
            :parameters(?l1)
            :precondition (at ?l1)
            )
            (:task abs_1 :parameters(?a))
            (:task abs_2 :parameters(?a))
            (:task abs_3 :parameters(?a))

            (:method m_1
                :parameters (?p1) 
                :task (abs_1 ?p1)
                :ordered-subtasks (and
                    (t2 (abs_2 ?p1))
                    (t3 (abs_3 ?p1))
                    (t4 (p_1 ?p1))
                )
            )
        ) ",
    )
    .into_bytes();
    let problem = String::from("
        (define (problem p-1-2-2)
            (:domain barman_htn)
            (:htn
                :parameters ()
                :ordered-subtasks (and
                    (abs_1)
                )
            )
    ").into_bytes();
    let lexer = LexicalAnalyzer::new(&program);
    let parser = Parser::new(lexer);
    let ast = parser.parse().unwrap();
    match ast {
        AbstractSyntaxTree::Domain(d) => {
            let p_lexer = LexicalAnalyzer::new(&problem);
            let p_parser = Parser::new(p_lexer);
            let p_ast = p_parser.parse().unwrap();
            match p_ast {
                AbstractSyntaxTree::Problem(p_ast) => {
                    let tdg = TDG::new(&d);
                    let nullables = tdg.compute_nullables();
                    assert_eq!(tdg.get_recursion_type(&nullables), RecursionType::NonRecursive)
                }
                _ => panic!()
            }
        }
        AbstractSyntaxTree::Problem(_) => panic!()
    }
}

#[test]
pub fn tdg_recursive_test () {
    let program = String::from(
        "(define (domain bal)
            (:predicates 
                (at ?l)
            )
            (:action p_1
            :parameters(?l1)
            :precondition (at ?l1)
            )
            (:action p_2
            :parameters(?l1)
            :precondition (at ?l1)
            )
            (:task abs_1 :parameters(?a))
            (:task abs_2 :parameters(?a))
            (:task abs_3 :parameters(?a))

            (:method m_1
                :parameters (?p1) 
                :task (abs_1 ?p1)
                :ordered-subtasks (and
                    (t0 (p_2 ?p1))
                    (t1 (abs_2 ?p1))
                    (t2 (abs_3 ?p1))
                    (t3 (abs_1 ?p1))
                    (t4 (p_1 ?p1))
                )
            )
        ) ",
    )
    .into_bytes();
    let problem = String::from("
        (define (problem p-1-2-2)
            (:domain barman_htn)
            (:htn
                :parameters ()
                :ordered-subtasks (and
                    (abs_1)
                )
            )
    ").into_bytes();
    let lexer = LexicalAnalyzer::new(&program);
    let parser = Parser::new(lexer);
    let ast = parser.parse().unwrap();
    match ast {
        AbstractSyntaxTree::Domain(d) => {
            let p_lexer = LexicalAnalyzer::new(&problem);
            let p_parser = Parser::new(p_lexer);
            let p_ast = p_parser.parse().unwrap();
            match p_ast {
                AbstractSyntaxTree::Problem(p_ast) => {
                    let tdg = TDG::new(&d);
                    let nullables = tdg.compute_nullables();
                    assert_eq!(tdg.get_recursion_type(&nullables), RecursionType::Recursive)
                }
                _ => panic!()
            }
        }
        AbstractSyntaxTree::Problem(_) => panic!()
    }
}

#[test]
pub fn tdg_grow_and_shrink_cycle_test () {
    let program = String::from(
        "(define (domain bal)
            (:predicates 
                (at ?l)
            )
            (:action p_1
            :parameters(?l1)
            :precondition (at ?l1)
            )
            (:action p_2
            :parameters(?l1)
            :precondition (at ?l1)
            )
            (:task abs_1 :parameters(?a))
            (:task abs_2 :parameters(?a))
            (:task abs_3 :parameters(?a))

            (:method m_1
                :parameters (?p1) 
                :task (abs_1 ?p1)
                :ordered-subtasks (and
                    (t0 (abs_3 ?p1))
                    (t1 (abs_1 ?p1))
                    (t2 (abs_3 ?p1))
                    (t3 (abs_3 ?p1))
                )
            )

            (:method m_2
                :parameters (?p1) 
                :task (abs_3 ?p1)
                :ordered-subtasks ()
            )
        ) ",
    )
    .into_bytes();
    let problem = String::from("
        (define (problem p-1-2-2)
            (:domain barman_htn)
            (:htn
                :parameters ()
                :ordered-subtasks (and
                    (abs_1)
                )
            )
    ").into_bytes();
    let lexer = LexicalAnalyzer::new(&program);
    let parser = Parser::new(lexer);
    let ast = parser.parse().unwrap();
    match ast {
        AbstractSyntaxTree::Domain(d) => {
            let p_lexer = LexicalAnalyzer::new(&problem);
            let p_parser = Parser::new(p_lexer);
            let p_ast = p_parser.parse().unwrap();
            match p_ast {
                AbstractSyntaxTree::Problem(p_ast) => {
                    let tdg = TDG::new(&d);
                    let nullables = tdg.compute_nullables();
                    assert_eq!(tdg.get_recursion_type(&nullables), RecursionType::GrowAndShrinkRecursion)
                }
                _ => panic!()
            }
        }
        AbstractSyntaxTree::Problem(_) => panic!()
    }
}


#[test]
pub fn tdg_growing_cycle_test () {
    let program = String::from(
        "(define (domain bal)
            (:predicates 
                (at ?l)
            )
            (:action p_1
            :parameters(?l1)
            :precondition (at ?l1)
            )
            (:action p_2
            :parameters(?l1)
            :precondition (at ?l1)
            )
            (:task abs_1 :parameters(?a))
            (:task abs_2 :parameters(?a))
            (:task abs_3 :parameters(?a))

            (:method m_1
                :parameters (?p1) 
                :task (abs_1 ?p1)
                :ordered-subtasks (and
                    (t1 (abs_1 ?p1))
                    (t2 (abs_3 ?p1))
                    (t3 (abs_1 ?p1))
                    (t4 (p_1 ?p1))
                )
            )

            (:method m_2
                :parameters (?p1) 
                :task (abs_3 ?p1)
                :ordered-subtasks (and
                    (t1 (p_1 ?p2))
                    (t2 (p_1 ?p2))
                )
            )
        ) ",
    )
    .into_bytes();
    let problem = String::from("
        (define (problem p-1-2-2)
            (:domain barman_htn)
            (:htn
                :parameters ()
                :ordered-subtasks (and
                    (abs_1)
                )
            )
    ").into_bytes();
    let lexer = LexicalAnalyzer::new(&program);
    let parser = Parser::new(lexer);
    let ast = parser.parse().unwrap();
    match ast {
        AbstractSyntaxTree::Domain(d) => {
            let p_lexer = LexicalAnalyzer::new(&problem);
            let p_parser = Parser::new(p_lexer);
            let p_ast = p_parser.parse().unwrap();
            match p_ast {
                AbstractSyntaxTree::Problem(p_ast) => {
                    let tdg = TDG::new(&d);
                    let nullables = tdg.compute_nullables();
                    assert_eq!(tdg.get_recursion_type(&nullables), RecursionType::GrowingEmptyPrefixRecursion)
                }
                _ => panic!()
            }
        }
        AbstractSyntaxTree::Problem(_) => panic!()
    }
}

#[test]
pub fn tdg_nullables_test () {
    let program = String::from(
        "(define (domain bal)
            (:predicates 
                (at ?l)
            )
            (:action p_1
            :parameters(?l1)
            :precondition (at ?l1)
            )
            (:action p_2
            :parameters(?l1)
            :precondition (at ?l1)
            )
            (:task abs_1 :parameters(?a))
            (:task abs_2 :parameters(?a))
            (:task abs_3 :parameters(?a))

            (:method m_1
                :parameters (?p1) 
                :task (abs_1 ?p1)
                :ordered-subtasks (and
                    (t0 (abs_3 ?p1))
                    (t1 (abs_2 ?p1))
                    (t2 (abs_3 ?p1))
                    (t3 (abs_3 ?p1))
                )
            )

            (:method m_2
                :parameters (?p1) 
                :task (abs_2 ?p1)
                :ordered-subtasks ()
            )

            (:method m_3
                :parameters (?p1) 
                :task (abs_3 ?p1)
                :ordered-subtasks (
                    abs_2 ?p1
                )
            )

            (:method m_3
                :parameters (?1) 
                :task (abs_3 ?p1)
                :ordered-subtasks (and
                    (p_1 ?l1)
                    (p_2 ?l1)
                )
            )
        ) ",
    )
    .into_bytes();
    let problem = String::from("
        (define (problem p-1-2-2)
            (:domain barman_htn)
            (:htn
                :parameters ()
                :ordered-subtasks (and
                    (abs_1)
                )
            )
    ").into_bytes();
    let lexer = LexicalAnalyzer::new(&program);
    let parser = Parser::new(lexer);
    let ast = parser.parse().unwrap();
    match ast {
        AbstractSyntaxTree::Domain(d) => {
            let p_lexer = LexicalAnalyzer::new(&problem);
            let p_parser = Parser::new(p_lexer);
            let p_ast = p_parser.parse().unwrap();
            match p_ast {
                AbstractSyntaxTree::Problem(p_ast) => {
                    let tdg = TDG::new(&d);
                    assert_eq!(tdg.compute_nullables(), HashSet::from(["abs_1", "abs_2", "abs_3"]))
                }
                _ => panic!()
            }
        }
        AbstractSyntaxTree::Problem(_) => panic!()
    }
}
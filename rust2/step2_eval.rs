#[macro_use]
extern crate lazy_static;

use std::io;
use crate::reader::{tokenize, Reader, create_reader, start_to_ast};
use crate::printer::pr_str;
use crate::types::{MalSimpleAST, MalType};
use std::collections::HashMap;

mod types;
mod reader;
mod printer;

enum StopReason {
    EOF
}

enum REPLState {
    Running,
    Stopping(StopReason),
}

fn main() {
    loop {
        match rep() {
            REPLState::Stopping(_) => return,
            _ => (),
        }
    }
}

fn read() -> Option<MalSimpleAST> {
    let mut input = String::new();
    let n_bytes = io::stdin().read_line(&mut input).expect("Something went wrong while trying to read input");
    let tokens = tokenize(&input);
    let mut reader = create_reader(tokens, 1);
    let ast = start_to_ast(&mut reader);
    ast
}

fn getSym(ast: &MalSimpleAST) -> String
{
    match ast {
        MalSimpleAST::Atom(atom) => {
            if let MalType::Symbol(sym) = atom {
                sym.clone()
            } else {
                panic!("expected symbol")
            }
        }
        _ => { panic!("expected atom") }
    }
}

fn eval(input: Option<MalSimpleAST>) -> Option<MalSimpleAST> {
    let mut map = HashMap::new();
    let add = |args: Vec<MalType>| -> MalType {
        //TODO find common base type
        let res = args.iter().fold(0isize, |sum, val| {
            if let MalType::Integer(i) = val {
                sum + i
            } else {
                panic!()
            }
        });
        MalType::Integer(res)
    };
    map.insert("+".to_string(), add);

    let res: MalSimpleAST = match input {
        Some(ast) => {
            match ast {
                MalSimpleAST::Atom(_) => { panic!("atom") }
                MalSimpleAST::List(list) => {
                    if list.len() == 0 {
                        MalSimpleAST::List(list)
                    } else {
                        let a = list.get(0).unwrap();
                        let sym = getSym(a);

                        let args =
                            list.iter()
                                .enumerate()
                                .filter(|&(i, _)| i != 0)
                                .map(|(_, v)| {
                                    match v {
                                        MalSimpleAST::Atom(atom) => {
                                            if let MalType::Integer(i) = atom {
                                                MalType::Integer(*i)
                                            } else {
                                                panic!()
                                            }
                                        }
                                        _ => { panic!("not implemented") }
                                    }
                                })
                                .collect::<Vec<_>>();
                        //apply
                        let fun = *map.get(&sym).expect("Could not find control symbol");
                        let res = fun(args);
                        MalSimpleAST::Atom(res)
                    }
                }
                _ => { panic!("not implemented") }
            }
        }
        None => {
            panic!();
        }
    };

    Some(res)
}

fn eval_ast(ast: &MalSimpleAST) -> &MalSimpleAST
{
    ast
}

fn print(ast: &Option<MalSimpleAST>) {
    match ast {
        None => {
            print!("nope")
        }
        Some(s) => {
            pr_str(&s);
        }
    }
    println!();
}

fn rep() -> REPLState {
    println!("user> ");
    let ast = read();
    if ast.is_none() {
        return REPLState::Stopping(StopReason::EOF);
    }
    let result = eval(ast);
    print(&result);
    REPLState::Running
}
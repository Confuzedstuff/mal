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

fn eval(input: &Option<MalSimpleAST>) -> Option<MalSimpleAST> {
    let mut map = HashMap::new();
    let add = |args: Vec<MalType>| -> MalType {
        let res = args.iter().fold(0isize, |sum, val| {
            if let MalType::Integer(i) = val {
                let res = sum + i;
                println!("sum {}", res);
                res
            } else {
                panic!()
            }
        });
        MalType::Integer(res)
    };
    map.insert("+", add);

    let res: MalSimpleAST = match input {
        Some(ast) => {
            match ast {
                MalSimpleAST::Atom(_) => { panic!("atom") }
                MalSimpleAST::List(list) => {
//                    if list.len() == 0 {
//                        Some(ast)
//                    } else {
                    println!("len = {}", list.len());
                    let func = list.get(0).unwrap();
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
                    let fun = *map.get("+").unwrap();
                    let res = fun(args);
                    MalSimpleAST::Atom(res)
//                    }
                }
                _ => { panic!("not implemented") }
            }
        }
        _ => {
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
    let result = eval(&ast);
    print(&result);
    REPLState::Running
}
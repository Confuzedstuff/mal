#[macro_use]
extern crate lazy_static;

use crate::printer::pr_str;
use crate::reader::{create_reader, start_to_ast, tokenize};
use crate::types::*;
use core::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
use std::io;
use std::convert::TryInto;
use std::io::Write;

mod printer;
mod reader;
mod types;

enum StopReason {
    EOF,
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

fn read() -> Option<AST> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Something went wrong while trying to read input");
    let tokens = tokenize(&input);
    let mut reader = create_reader(tokens, 1);
    let ast = start_to_ast(&mut reader);
    ast
}

fn add(args: Vec<MalType>) -> MalType {
    //TODO find common base type
    let res = args.iter().fold(0isize, |sum, val| {
        if let MalType::Integer(i) = val {
            sum + i
        } else {
            panic!("not implemented number type")
        }
    });
    MalType::Integer(res)
}

fn multiply(args: Vec<MalType>) -> MalType {
    //TODO find common base type
    if let MalType::Integer(first) =args.get(0).expect("At least one arg is required"){
        let mut res = *first;
        for x in args.iter().enumerate().filter(|(i,_)| *i != 0).map(|(_,x)|x) {
            if let MalType::Integer(i) = x{
                     res *= i;
            }else{
                panic!()
            }
        }
        MalType::Integer(res)
    }else{
        panic!("not impl mult")
    }
}

fn subtract(args: Vec<MalType>) -> MalType {
    //TODO find common base type
    let first = args.get(0).expect("Require 2 args");
    let second = args.get(1).expect("Require 2 args");

    if let MalType::Integer(i) = first {
        if let MalType::Integer(j) = second {
            MalType::Integer(i - j)
        } else {
            panic!("not implemented number type")
        }
    } else {
        panic!("not implemented number type")
    }
}

fn divide(args: Vec<MalType>) -> MalType {
    //TODO find common base type
    let first = args.get(0).expect("Require 2 args");
    let second = args.get(1).expect("Require 2 args");

    if let MalType::Integer(i) = first {
        if let MalType::Integer(j) = second {
            MalType::Integer(i / j)
        } else {
            panic!("not implemented number type")
        }
    } else {
        panic!("not implemented number type")
    }
}


fn get_repl_env() -> ReplEnv {
    let mut map: ReplEnv = HashMap::new();
    map.insert("+".to_string(), add);
    map.insert("-".to_string(), subtract);
    map.insert("*".to_string(), multiply);
    map.insert("/".to_string(), divide);
    map
}

fn eval(ast: AST, repl_env: &mut ReplEnv) -> ASTResult {
    match ast {
        AST::Atom(_) => eval_ast(&ast, repl_env),
        AST::List(ref list) if list.len() == 0 => {
            Ok(ast)
        }
        AST::List(_) => {
            if let AST::List(list) = eval_ast(&ast, repl_env)? {
                Ok(apply(*list))
            } else {
                Err(fail("must return list"))
            }
        }
        _ => Err(fail("not implemented")),
    }
}

fn fail(s:&str) -> MalError {
    MalError::Panic {description:s.to_string()}
}

fn apply(list: Vec<AST>) -> AST {
    if let AST::Atom(atom) = list.get(0).unwrap() {
        if let MalType::Symbol(_, opt) = atom {
            let fun = opt.expect("fun");
            let args: Vec<_> =
                list.iter()
                    .enumerate()
                    .filter(|(i, _)| *i != 0)
                    .map(|(_, x)| match x {
                        AST::Atom(atom) => {
                            atom.clone()
                        }
                        _ => panic!("invalid")
                    })
                    .collect();

            AST::Atom(fun(args))
        } else {
            AST::Atom(atom.clone())
        }
    } else {
        panic!("invalid")
    }
}

fn eval_ast(ast: &AST, repl_env: &mut ReplEnv) ->  ASTResult {
    match ast {
        AST::Atom(atom) => {
            if let MalType::Symbol(sym, _) = atom {
                match repl_env.get(sym) {
                    None => {
                        Err(fail(&format!("Could not find control symbol {}",sym) ))
                    },
                    Some(fun) => {
                        Ok(AST::Atom(MalType::Symbol(sym.clone(), Some(*fun))))
                    },
                }
            } else {
                Ok(ast.clone())
            }
        }
        AST::List(list) => {
            let res_list :Vec<ASTResult> = list.iter().map(|x| eval(x.clone(), repl_env)).collect();
            let errors: Vec<_> = res_list.iter().filter(|x| x.is_err()).collect();
            if errors.len() == 0{
                let oks: Vec<_> = res_list.iter().map(|x| x.clone().unwrap()).collect();
                Ok(AST::List(Box::new(oks)))
            }else{
                let r = errors.first().unwrap().to_owned().to_owned();
                r
            }

        }
        _ => {
            panic!("todo")
        }
    }
}

fn print(ast: &Option<AST>) {
    match ast {
        None => print!("nope"),
        Some(s) => {
            pr_str(&s);
        }
    }
    println!();
}

fn rep() -> REPLState {
    print!("user> ");
    std::io::stdout().flush();

    let mut ast = match read() {
        Some(ast) => ast,
        None => {
            return REPLState::Stopping(StopReason::EOF);
        }
    };
    let ast = eval(ast, &mut get_repl_env());
    match ast {
        Ok(ok) => {
            print(&Some(ok));
        },
        Err(err) => {
            println!("{}", err);
        },
    }

    REPLState::Running
}

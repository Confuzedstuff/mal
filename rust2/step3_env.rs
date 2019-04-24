#[macro_use]
extern crate lazy_static;

use core::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
use std::io;
use std::convert::TryInto;
use std::io::Write;

mod printer;

use crate::printer::pr_str;

mod reader;

use crate::reader::{create_reader, start_to_ast, tokenize};

mod types;

use crate::types::*;
use crate::types::MalToken::*;
use crate::types::AST::*;
use crate::types::MalType::*;
use crate::env::Env;

mod env;

enum StopReason {
    EOF,
}

enum REPLState {
    Running,
    Stopping(StopReason),
}

fn main() {
    let mut env = get_repl_env();
    loop {
        match rep(&mut env) {
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
        if let Integer(i) = val {
            sum + i
        } else {
            panic!("not implemented number type")
        }
    });
    Integer(res)
}

fn multiply(args: Vec<MalType>) -> MalType {
    //TODO find common base type
    if let Integer(first) = args.get(0).expect("At least one arg is required") {
        let mut res = *first;
        for x in args.iter().enumerate().filter(|(i, _)| *i != 0).map(|(_, x)| x) {
            if let Integer(i) = x {
                res *= i;
            } else {
                panic!()
            }
        }
        Integer(res)
    } else {
        panic!("not impl mult")
    }
}

fn subtract(args: Vec<MalType>) -> MalType {
    //TODO find common base type
    let first = args.get(0).expect("Require 2 args");
    let second = args.get(1).expect("Require 2 args");

    if let Integer(i) = first {
        if let Integer(j) = second {
            Integer(i - j)
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

    if let Integer(i) = first {
        if let Integer(j) = second {
            Integer(i / j)
        } else {
            panic!("not implemented number type")
        }
    } else {
        panic!("not implemented number type")
    }
}


fn get_repl_env() -> Env {
    let mut env = Env::new(None);
    env.set(&Symbol("+".to_string(), None), Func(add));
    env.set(&Symbol("-".to_string(), None), Func(subtract));
    env.set(&Symbol("*".to_string(), None), Func(multiply));
    env.set(&Symbol("/".to_string(), None), Func(divide));
    env
}

fn eval(ast: AST, repl_env: &mut Env) -> ASTResult {
    match ast {
        Atom(_) => eval_ast(&ast, repl_env),
        List(ref list) if list.len() == 0 => {
            Ok(ast)
        }
        List(_) => {
            if let List(list) = eval_ast(&ast, repl_env)? {
                Ok(apply(*list, repl_env))
            } else {
                Err(fail("must return list"))
            }
        }
        _ => Err(fail("not implemented")),
    }
}

fn fail(s: &str) -> MalError {
    MalError::Panic { description: s.to_string() }
}

fn apply(list: Vec<AST>, env: &mut Env) -> AST {
    if let Atom(atom) = list.get(0).expect("op lookup") {
        match atom {
            Symbol(_, opt) => {
                let fun = opt.expect("fun");
                let args: Vec<_> =
                    list.iter()
                        .enumerate()
                        .filter(|(i, _)| *i != 0)
                        .map(|(_, x)| match x {
                            Atom(atom) => {
                                atom.clone()
                            }
                            _ => panic!("invalid")
                        })
                        .collect();

                Atom(fun(args))
            }
            Def => {
                let key = list.get(1).expect("require 2 args");
                let value = list.get(2).expect("require 2 args");
                match value {
                    Atom(atom) => {
                        let key = match key {
                            AST::Atom(key) => {
                                key
                            }
                            _ => { panic!("invalid") }
                        };
                        println!("key {:?} value {:?}", key, atom.clone());
                        env.set(key, atom.clone());
                        Atom(atom.clone())
                    }
                    List(list) => {
                        panic!("todo")
                    }
                    _ => {
                        panic!("todo")
                    }
                }
            }
            x => {
                println!("{:?}", x);
                if let Some(value) = env.find(x) {
                    Atom(value.clone())
                } else {
                    Atom(atom.clone()) // TODO ? return nil?
                }
            }
        }
    } else {
        panic!("invalid")
    }
}

fn eval_ast(ast: &AST, repl_env: &mut Env) -> ASTResult {
    match ast {
        Atom(atom) => {
            match atom {
                _ => {
                    match repl_env.find(atom) {
                        None => {
                            //Err(fail(&format!("Could not find {:?} in env", atom)))
                            Ok(ast.clone())
                        }
                        Some(value) => {
                            match value {
                                Func(fun) => {
                                    let sym = match atom {
                                        Symbol(sym, _) => {
                                            sym
                                        }
                                        _ => {
                                            panic!("no")
                                        }
                                    };
                                    Ok(Atom(Symbol(sym.to_string(), Some(*fun))))
                                }
                                _ => {
                                    Ok(Atom(value.clone()))
                                }
                            }
                        }
                    }
                }
            }
        }
        List(list) => {
            let res_list: Vec<ASTResult> = list.iter().map(|x| eval(x.clone(), repl_env)).collect();
            let errors: Vec<_> = res_list.iter().filter(|x| x.is_err()).collect();
            if errors.len() == 0 {
                let oks: Vec<_> = res_list.iter().map(|x| x.clone().unwrap()).collect();
                Ok(List(Box::new(oks)))
            } else {
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

fn rep(env: &mut Env) -> REPLState {
    print!("user> ");
    std::io::stdout().flush();

    let mut ast = match read() {
        Some(ast) => ast,
        None => {
            return REPLState::Stopping(StopReason::EOF);
        }
    };
    let ast = eval(ast, env);
    match ast {
        Ok(ok) => {
            print(&Some(ok));
            println!("env {:?}", env);
        }
        Err(err) => {
            println!("{}", err);
            println!("env {:?}", env);
        }

    }

    REPLState::Running
}

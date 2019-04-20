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

fn read() -> Option<MalSimpleAST> {
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

fn get_repl_env() -> ReplEnv {
    let mut map: ReplEnv = HashMap::new();
    map.insert("+".to_string(), add);
    map
}

fn eval(ast: MalSimpleAST, repl_env: &mut ReplEnv) -> MalSimpleAST {
    match ast {
        MalSimpleAST::Atom(_) => eval_ast(&ast, repl_env),
        MalSimpleAST::List(ref list) if list.len() == 0 => {
            ast
        }
        MalSimpleAST::List(_) => {
            if let MalSimpleAST::List(list) = eval_ast(&ast, repl_env) {
                apply(*list)
            } else {
                panic!("must return list")
            }
        }
        _ => panic!("not implemented"),
    }
}

fn apply(list: Vec<MalSimpleAST>) -> MalSimpleAST {
    if let MalSimpleAST::Atom(atom) = list.get(0).unwrap() {
        if let MalType::Symbol(_, opt) = atom {
            let fun = opt.expect("fun");
            let args:Vec<_> =
                list.iter()
                    .enumerate()
                    .filter(|(i,_)| *i != 0)
                    .map(|(_, x)| match x{
                        MalSimpleAST::Atom(atom)=>{
                            atom.clone()
                        }
                        _=> panic!("invalid")
                    })
                    .collect();

            MalSimpleAST::Atom(fun(args))
        } else {
            panic!("invalid")
        }
    } else {
        panic!("invalid")
    }
}

fn eval_ast(ast: &MalSimpleAST, repl_env: &mut ReplEnv) -> MalSimpleAST {
    match ast {
        MalSimpleAST::Atom(atom) => {
            if let MalType::Symbol(sym, _) = atom {
                let fun = *repl_env.get(sym).expect("Could not find control symbol");
                MalSimpleAST::Atom(MalType::Symbol(sym.clone(), Some(fun)))
            } else {
                ast.clone()
            }
        }
        MalSimpleAST::List(list) => {
            let res_list = list.iter().map(|x| eval(x.clone(), repl_env)).collect();
            MalSimpleAST::List(Box::new(res_list))
        }
        _ => {
            panic!("todo")
        }
    }
}

fn print(ast: &Option<MalSimpleAST>) {
    match ast {
        None => print!("nope"),
        Some(s) => {
            pr_str(&s);
        }
    }
    println!();
}

fn rep() -> REPLState {
    println!("user> ");
    //std::io::stdout().flush();

    let mut ast = match read() {
        Some(ast) => ast,
        None => {
            return REPLState::Stopping(StopReason::EOF);
        }
    };
    let ast = eval(ast, &mut get_repl_env());
    print(&Some(ast));
    REPLState::Running
}

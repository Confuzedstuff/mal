#[macro_use]
extern crate lazy_static;

use std::io;
use crate::reader::{tokenize, create_reader, start_to_ast};
use crate::printer::pr_str;
use crate::types::*;
use std::collections::HashMap;
use core::borrow::Borrow;

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
    io::stdin().read_line(&mut input).expect("Something went wrong while trying to read input");
    let tokens = tokenize(&input);
    let mut reader = create_reader(tokens, 1);
    let ast = start_to_ast(&mut reader);
    ast
}

fn get_sym(ast: &MalSimpleAST) -> String
{
    match ast {
        MalSimpleAST::Atom(atom) => {
            if let MalType::Symbol(sym, None) = atom {
                sym.clone()
            } else {
                panic!("expected symbol")
            }
        }
        _ => { panic!("expected atom") }
    }
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

fn eval(ast: MalSimpleAST, repl_env: &mut ReplEnv) -> Option<MalSimpleAST> {
    let res: MalSimpleAST =
        match &ast {
            MalSimpleAST::Atom(_) => { panic!("atom") }
            MalSimpleAST::List(list) => {
                if list.len() == 0 {
                    //MalSimpleAST::List(list)
                    panic!();
                } else {
                    let newast = eval_ast(&ast, repl_env);
                    if let MalSimpleAST::List(list) = &newast {
                        println!("{:?}", newast);
                        panic!()
                    } else {
                        println!("{:?}", newast);
                        panic!("result must be a list ?")
                    }
                }
            }
            _ => { panic!("not implemented") }
        };

    Some(res)
}


//let first = list.get(0).unwrap();
//
//let args =
//list.iter()
//.enumerate()
//.filter(|&(i, _)| i != 0)
//.map(|(_, v)| {
//match v {
//MalSimpleAST::Atom(atom) => {
////                                if let MalType::Integer(i) = atom {
////                                    MalType::Integer(*i)
////                                } else {
////                                    panic!("non integer")
////                                }
//}
//MalSimpleAST::List(list) => {
//panic!()
//}
//
//_ => { panic!("not implemented") }
//}
//})
//.collect::<Vec<_>>();
//
//
////apply
////            let sym =get_sym(first);
////            let fun = *repl_env.get(&sym).expect("Could not find control symbol");
////            let res = fun(args);
////            MalSimpleAST::Atom(res)

fn eval_ast(ast: &MalSimpleAST, repl_env: &mut ReplEnv) -> MalSimpleAST
{
    let r:Option<MalSimpleAST> = match ast {
        MalSimpleAST::Atom(atom) => {
            if let MalType::Symbol(sym, _) = atom {
                let sym = get_sym(&ast);
                let fun = *repl_env.get(&sym).expect("Could not find control symbol");
                Some( MalSimpleAST::Atom(MalType::Symbol(sym, Some(fun))))
            } else {
                None
            }
        }
        MalSimpleAST::List(list) => {
            let mut transformed_list = Vec::new();
            for x in list.iter() {
                let v = eval_ast(x, repl_env);
                transformed_list.push(v);
            }

            Some(MalSimpleAST::List(Box::new(transformed_list)))
        }
        _ => {
            panic!()
        }
    };

    let rr = match r {
        Some(s)=>{s}
        None => ast
    };
    rr
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
    let ast = match read() {
        Some(ast) => { ast }
        None => {
            return REPLState::Stopping(StopReason::EOF);
        }
    };
    let result = eval(ast, &mut get_repl_env());
    print(&result);
    REPLState::Running
}
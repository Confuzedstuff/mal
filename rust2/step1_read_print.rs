#[macro_use]
extern crate lazy_static;

use std::io;
use crate::reader::{tokenize, Reader, create_reader, start_to_ast};
use crate::printer::pr_str;
use crate::types::MalSimpleAST;

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
    let mut reader = create_reader(tokens, 0);
    let ast = start_to_ast(&mut reader);
    ast
}

fn eval(input: &Option<MalSimpleAST>) -> &Option<MalSimpleAST> {
    input
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
    print(result);
    REPLState::Running
}
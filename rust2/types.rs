use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum MalSimpleAST {
    Atom(MalType),
    List(Box<Vec<MalSimpleAST>>),
    Vector(Box<Vec<MalSimpleAST>>),
    HashMap(Box<Vec<MalSimpleAST>>)
}

#[derive(Debug)]
pub enum MalToken {
    SpecialTwo(String),
    SpecialOne(char),
    StringLiteral(String),
    Comment(String),
    NonSpecial(String)
}

#[derive(Debug,Clone)]
pub enum MalType{
    Comment(String),
    Deref(String),
    IncompleteDeref,
    Something(String),
    UnbalancedString(String),
    StringLiteral(String),
    UnbalancedListEnd,
    Quote,
    QuasiQuote,
    UnQuote,
    SpliceUnQuote,
    TEMPNOTHING(String),
    Meta,
    Integer(isize),
    Symbol(String,Option<EnvFunc>)
}

pub type EnvFunc = fn(Vec<MalType>)->MalType;
pub type ReplEnv = HashMap<String, fn(Vec<MalType>)->MalType>;
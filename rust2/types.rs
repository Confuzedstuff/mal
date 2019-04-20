use std::collections::HashMap;
use failure::Fail;

#[derive(Debug, Clone)]
pub enum AST {
    Atom(MalType),
    List(Box<Vec<AST>>),
    Vector(Box<Vec<AST>>),
    HashMap(Box<Vec<AST>>)
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

#[derive(Debug, Fail,Clone)]
pub enum MalError {
    #[fail(display = "fail: {}", description)]
    Panic {
        description: String,
    },
}

//impl From<std::option::NoneError> for MalError{
//    fn from(_: std::option::NoneError) -> Self {
//        MalError::Panic {description:"None".to_string()}
//    }
//}
pub type ASTResult = Result<AST,MalError>;

pub type EnvFunc = fn(Vec<MalType>)->MalType;
pub type ReplEnv = HashMap<String, fn(Vec<MalType>)->MalType>;
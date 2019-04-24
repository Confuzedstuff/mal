use std::collections::HashMap;
use failure::Fail;

#[derive(Debug, Clone)]
pub enum AST {
    Atom(MalType),
    List(Box<Vec<AST>>),
    Vector(Box<Vec<AST>>),
    HashMap(Box<Vec<AST>>),
}

#[derive(Debug)]
pub enum MalToken {
    SpecialTwo(String),
    SpecialOne(char),
    StrLiteral(String),
    CommentToken(String),
    NonSpecial(String),
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum MalType {
    Comment(String),
    Deref(String),
    IncompleteDeref,
    Something(String),
    UnbalancedString(String),
    Str(String),
    UnbalancedListEnd,
    Quote,
    QuasiQuote,
    UnQuote,
    SpliceUnQuote,
    TEMPNOTHING(String),
    Meta,
    Integer(isize),
    Symbol(String, Option<EnvFunc>), //TODO remove func
    Def,
    Func(EnvFunc),
    Variable(String),
}

#[derive(Debug, Fail, Clone)]
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
pub type ASTResult = Result<AST, MalError>;

pub type EnvFunc = fn(Vec<MalType>) -> MalType;
pub type ReplEnv = HashMap<MalType, MalType>;
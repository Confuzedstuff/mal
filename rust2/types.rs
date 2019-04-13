#[derive(Debug)]
pub enum MalSimpleAST {
    MalAtom(MalType),
    MalList(Box<Vec<MalSimpleAST>>),
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

#[derive(Debug)]
pub enum MalType{
    Comment(String),
    Deref(String),
    String(String),
    UnbalancedString(String),
    StringLiteral(String),
    UnbalancedListEnd,
    Quote,
    QuasiQuote,
    UnQuote
}
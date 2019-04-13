#[derive(Debug)]
pub enum MalSimpleAST {
    MalAtom(MalType),
    MalList(Box<Vec<MalSimpleAST>>),
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
    String(String)
}
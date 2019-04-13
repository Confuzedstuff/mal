extern crate regex;

use regex::Regex;
use crate::types::{MalToken, MalSimpleAST, MalType};
use crate::read;

lazy_static! {
    static ref RE: Regex = Regex::new(r#"(?x)
    (?P<whitespace>[\s,]*)
    (
    (?P<specialtwo>~@)
    |(?P<specialone>[\[\]{}()'`~^@])
    |(?P<stringliteral>"(?:\\.|[^\\"])*"?)
    |(?P<comment>;.*)
    |(?P<nonspecial>[^\s\[\]{}('"`,;)]*)
    )"#).unwrap();
}

pub fn tokenize(input: &str) -> Vec<MalToken> {
    let mut tokens: Vec<MalToken> = Vec::new();
    for cap in RE.captures_iter(input) {
//        if let Some(whitespace) = cap.name("whitespace") {
//            println!("whitespace {}", whitespace.as_str());
//        }

        if let Some(specialone) = cap.name("specialone") {
            println!("specialone {}", specialone.as_str());
            tokens.push(MalToken::SpecialOne(specialone.as_str().chars().next().unwrap()));
            continue;
        }

        if let Some(stringliteral) = cap.name("stringliteral") {
            //println!("stringliteral {}", stringliteral.as_str());
            tokens.push(MalToken::StringLiteral(String::from(stringliteral.as_str())));
            continue;
        }

        if let Some(comment) = cap.name("comment") {
            //println!("comment {}", comment.as_str());
            tokens.push(MalToken::Comment(String::from(comment.as_str())));
            continue;
        }

        if let Some(nonspecial) = cap.name("nonspecial") {
            //println!("nonspecial {}", nonspecial.as_str());
            tokens.push(MalToken::NonSpecial(String::from(nonspecial.as_str())));
            continue;
        }
    }
    tokens
}

pub struct Reader {
    tokens: Vec<MalToken>,
    pos: usize,
}

pub fn create_reader(tokens: Vec<MalToken>, pos: usize) -> Reader{
    Reader{
        tokens,
        pos
    }
}

fn peek(tokens: &Vec<MalToken>, pos: usize) -> Option<&MalToken>
{
    if pos >= tokens.len() {
        None
    } else {
        Some(&tokens[pos])
    }
}

 impl Reader {
    fn next(&mut self) -> Option<&MalToken>
    {
        let token = peek(&self.tokens, self.pos);
        if token.is_some() {
            self.pos += 1;
        }
        token
    }
    fn peeks(&self) -> Option<&MalToken> {
        peek(&self.tokens, self.pos)
    }
}


pub fn start_to_ast(reader: &mut Reader) -> Option<MalSimpleAST>
{
    if let Some(token) = reader.next() {
        match token {
            MalToken::SpecialOne(c) => { to_ast_list(reader) }
            a => {
                to_ast_elem(a)
            }
        }
    } else {
        None
    }
}

fn to_ast_elem(token: &MalToken) -> Option<MalSimpleAST>{
    match token {
        MalToken::SpecialTwo(_) => {
            None
        },
        MalToken::SpecialOne(_) => {
            None

        },
        MalToken::StringLiteral(_) => {
            None

        },
        MalToken::Comment(_) => {
            None

        },
        MalToken::NonSpecial(x) => {
            let s = String::from(x.trim());
//            println!("ATOM {}",s);
            Some(MalSimpleAST::MalAtom(MalType::String(s)))
        },
    }
}


fn to_ast_list(reader: &mut Reader) -> Option<MalSimpleAST>
{
    None
}
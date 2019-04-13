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
            tokens.push(MalToken::SpecialOne(specialone.as_str().chars().next().unwrap()));
            continue;
        }

        if let Some(stringliteral) = cap.name("stringliteral") {
            tokens.push(MalToken::StringLiteral(String::from(stringliteral.as_str())));
            continue;
        }

        if let Some(comment) = cap.name("comment") {
            tokens.push(MalToken::Comment(String::from(comment.as_str())));
            continue;
        }

        if let Some(nonspecial) = cap.name("nonspecial") {
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

pub fn create_reader(tokens: Vec<MalToken>, pos: usize) -> Reader {
    Reader {
        tokens,
        pos,
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
        self.pos += 1;
        let token = peek(&self.tokens, self.pos - 1);
        token
    }
    fn peek(&self) -> Option<&MalToken> {
        peek(&self.tokens, self.pos - 1)
    }
}


pub fn start_to_ast(reader: &mut Reader) -> Option<MalSimpleAST>
{
    if let Some(token) = reader.next() {
        match token {
            MalToken::SpecialOne(c) => {
                if *c == '(' || *c == '[' {
                    to_ast_list(reader)
                } else if *c == '\'' || *c == '`' || *c == '~' {
                    let mut  v: Vec<MalSimpleAST> = Vec::new();
                    if *c == '\'' {
                        v.push(MalSimpleAST::MalAtom(MalType::Quote));
                    }else if *c == '`'{
                        v.push(MalSimpleAST::MalAtom(MalType::QuasiQuote));
                    }else if *c == '~'{
                        v.push(MalSimpleAST::MalAtom(MalType::UnQuote));
                    }
                    if let Some(ast) = start_to_ast(reader){
                        v.push(ast);
                    }else{
                        v.push(MalSimpleAST::MalAtom(MalType::UnbalancedListEnd));
                    }
                    Some(MalSimpleAST::MalList(Box::new(v)))
                }
                else {
                    to_ast_elem(&reader)
                }
            }
            a => {
                to_ast_elem(&reader)
            }
        }
    } else {
        None
    }
}

fn to_ast_elem(reader: &Reader) -> Option<MalSimpleAST> {
    if let Some(token) = reader.peek() {
        match token {
            MalToken::SpecialTwo(_) => {
                None
            }
            MalToken::SpecialOne(_) => {
                None
            }
            MalToken::StringLiteral(x) => {
                let s = String::from(x.trim());
                Some(MalSimpleAST::MalAtom(MalType::StringLiteral(s)))
            }
            MalToken::Comment(comment) => {
                let c = comment.clone();
                Some(MalSimpleAST::MalAtom(MalType::Comment(c)))
            }
            MalToken::NonSpecial(x) => {
                let s = String::from(x.trim());
                Some(MalSimpleAST::MalAtom(MalType::String(s)))
            }
        }
    } else {
        None
    }
}

fn to_ast_list(reader: &mut Reader) -> Option<MalSimpleAST>
{
    let mut items: Vec<MalSimpleAST> = Vec::new();
    loop {
        let token = reader.next();
        if let Some(token) = token {
            match token {
                MalToken::SpecialOne(x) => {
                    if (*x == ')') || (*x == ']') {
                        break;
                    }
                    if (*x == '(') || (*x == '[') {
                        let list = to_ast_list(reader);
                        if let Some(list) = list {
                            items.push(list);
                        }
                    }
                }
                _ => {
                    let r = to_ast_elem(reader);
                    if let Some(r) = r {
                        items.push(r);
                    }
                }
            }
        } else {
            items.push(MalSimpleAST::MalAtom(MalType::UnbalancedListEnd));
            break;
        }
    }
    Some(MalSimpleAST::MalList(Box::new(items)))
}
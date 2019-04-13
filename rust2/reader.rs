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
        if let Some(specialtwo) = cap.name("specialtwo") {
            tokens.push(MalToken::SpecialTwo(specialtwo.as_str().to_string()));
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
    if let Some(token) = reader.peek() {
        match token {
            MalToken::SpecialOne(c) => {
                if *c == '(' || *c == '[' || *c == '{' { //identify list start
                    to_ast_list(reader)
                } else if *c == '\'' || *c == '`' || *c == '~' { //identify quote start
                    to_ast_quote(reader)
                } else if *c == '@' {
                    to_ast_deref(reader)
                } else if *c == '^' {
                    to_ast_metadata(reader)
                } else {
                    to_ast_elem(&reader)
                }
            }
            MalToken::SpecialTwo(s2) => {
                to_ast_quote(reader)
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
            MalToken::SpecialTwo(x) => {
                Some(MalSimpleAST::Atom(MalType::TEMPNOTHING(x.to_string())))
            }
            MalToken::SpecialOne(x) => {
                Some(MalSimpleAST::Atom(MalType::TEMPNOTHING(x.to_string())))
            }
            MalToken::StringLiteral(x) => {
                let s = String::from(x.trim());

                if open_equals_close(&s) {
                    Some(MalSimpleAST::Atom(MalType::StringLiteral(s)))
                } else {
                    Some(MalSimpleAST::Atom(MalType::UnbalancedString(s)))
                }
            }
            MalToken::Comment(comment) => {
                let c = comment.clone();
                Some(MalSimpleAST::Atom(MalType::Comment(c)))
            }
            MalToken::NonSpecial(x) => {
                let t = x.trim();
                let s = String::from(t);

                Some(MalSimpleAST::Atom(MalType::Something(s)))
            }
        }
    } else {
        None
    }
}

fn open_equals_close(s: &str) -> bool {
    if s.len() > 0 {
        let first = s.chars().next().unwrap();
        if first != '"'
            && first != '\''
        {
            return true; // todo move to separate fn
        }
        let last = s.chars().rev().next().unwrap();
        first == last
    } else {
        true
    }
}

fn to_ast_list(reader: &mut Reader) -> Option<MalSimpleAST>
{
    let mut items: Vec<MalSimpleAST> = Vec::new();
    let res: Option<MalSimpleAST>;
    let open_brace = match reader.peek().unwrap() {
        MalToken::SpecialOne(x) => {
            *x
        }
        _ => {
            panic!()
        }
    };
    loop {
        let token = reader.next();
        if let Some(token) = token {
            match token {
                MalToken::SpecialOne(x) => {
                    if *x == ')' { // todo check close brace match
                        res = Some(MalSimpleAST::List(Box::new(items)));
                        break;
                    }
                    if *x == ']' {
                        res = Some(MalSimpleAST::Vector(Box::new(items)));
                        break;
                    }
                    if *x == '}' {
                        res = Some(MalSimpleAST::HashMap(Box::new(items)));
                        break;
                    }
                }
                _ => {}
            }
            let ast = start_to_ast(reader);
            if let Some(ast) = ast {
                items.push(ast);
            }
        } else {
            items.push(MalSimpleAST::Atom(MalType::UnbalancedListEnd));
            res = Some(MalSimpleAST::List(Box::new(items)));
            break;
        }
    }
    res
}

fn to_ast_quote(reader: &mut Reader) -> Option<MalSimpleAST>
{
    let quote: String =
        match reader.peek().unwrap() {
            MalToken::SpecialOne(c) => {
                (*c).to_string()
            }
            MalToken::SpecialTwo(s2) => {
                s2.to_string()
            }
            _ => { panic!() }
        };

    let mut v: Vec<MalSimpleAST> = Vec::new();

    if quote == "'" {
        v.push(MalSimpleAST::Atom(MalType::Quote));
    } else if quote == "`" {
        v.push(MalSimpleAST::Atom(MalType::QuasiQuote));
    } else if quote == "~" {
        v.push(MalSimpleAST::Atom(MalType::UnQuote));
    } else if quote == "~@" {
        v.push(MalSimpleAST::Atom(MalType::SpliceUnQuote));
    }
    if let Some(token) = reader.next() {
        if let Some(ast) = start_to_ast(reader) {
            v.push(ast);
        } else {
            v.push(MalSimpleAST::Atom(MalType::UnbalancedListEnd));
        }
        Some(MalSimpleAST::List(Box::new(v)))
    } else {
        None // todo incomplete quote
    }
}

fn to_ast_deref(reader: &mut Reader) -> Option<MalSimpleAST> {
    if let Some(token) = reader.next() {
        match token {
            MalToken::NonSpecial(x) => {
                Some(MalSimpleAST::Atom(MalType::Deref(x.to_string())))
            }
            _ => { None }
        }
    } else {
        Some(MalSimpleAST::Atom(MalType::IncompleteDeref))
    }
}

fn to_ast_metadata(reader: &mut Reader) -> Option<MalSimpleAST> {
    if let Some(token) = reader.next() {
        let mut v: Vec<MalSimpleAST> = Vec::new();
        v.push(MalSimpleAST::Atom(MalType::Meta));
        let ast = start_to_ast(reader);
        if let Some(ast) = ast {
            v.push(ast);
        }
        Some(MalSimpleAST::List(Box::new(v)))
    } else {
        None
    }
}
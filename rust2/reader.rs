use regex::Regex;
use crate::types::*;
use crate::types::MalToken::*;
use crate::types::AST::*;
use crate::types::MalType::*;

lazy_static! {
    static ref RE: Regex = Regex::new(
        r#"(?x)
    (?P<whitespace>[\s,]*)
    (
    (?P<specialtwo>~@)
    |(?P<specialone>[\[\]{}()'`~^@])
    |(?P<stringliteral>"(?:\\.|[^\\"])*"?)
    |(?P<comment>;.*)
    |(?P<nonspecial>[^\s\[\]{}('"`,;)]*)
    )"#
    )
    .unwrap();
}

pub fn tokenize(input: &str) -> Vec<MalToken> {
    let mut tokens: Vec<MalToken> = Vec::new();
    for cap in RE.captures_iter(input) {
        //        if let Some(whitespace) = cap.name("whitespace") {
        //            println!("whitespace {}", whitespace.as_str());
        //        }

        if let Some(specialone) = cap.name("specialone") {
            tokens.push(SpecialOne(
                specialone.as_str().chars().next().unwrap(),
            ));
            continue;
        }
        if let Some(specialtwo) = cap.name("specialtwo") {
            tokens.push(SpecialTwo(specialtwo.as_str().to_string()));
            continue;
        }

        if let Some(stringliteral) = cap.name("stringliteral") {
            tokens.push(StrLiteral(String::from(
                stringliteral.as_str(),
            )));
            continue;
        }

        if let Some(comment) = cap.name("comment") {
            tokens.push(CommentToken(String::from(comment.as_str())));
            continue;
        }

        if let Some(nonspecial) = cap.name("nonspecial") {
            tokens.push(NonSpecial(String::from(nonspecial.as_str())));
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
    Reader { tokens, pos }
}

fn peek(tokens: &Vec<MalToken>, pos: usize) -> Option<&MalToken> {
    if pos >= tokens.len() {
        None
    } else {
        Some(&tokens[pos])
    }
}

impl Reader {
    fn next(&mut self) -> Option<&MalToken> {
        self.pos += 1;
        let token = peek(&self.tokens, self.pos - 1);
        token
    }
    fn peek(&self) -> Option<&MalToken> {
        peek(&self.tokens, self.pos - 1)
    }
}

pub fn start_to_ast(reader: &mut Reader) -> Option<AST> {
    if let Some(token) = reader.peek() {
        match token {
            SpecialOne(c) => {
                let c = *c;
                if c == '(' || c == '[' || c == '{' {
                    //identify list start
                    to_ast_list(reader)
                } else if c == '\'' || c == '`' || c == '~' {
                    //identify quote start
                    to_ast_quote(reader)
                } else if c == '@' {
                    to_ast_deref(reader)
                } else if c == '^' {
                    to_ast_metadata(reader)
                } else {
                    to_ast_elem(&reader)
                }
            }
            SpecialTwo(_) => to_ast_quote(reader),
            _ => to_ast_elem(&reader),
        }
    } else {
        None
    }
}

fn to_ast_elem(reader: &Reader) -> Option<AST> {
    if let Some(token) = reader.peek() {
        match token {
            SpecialTwo(x) => {
                Some(Atom(TEMPNOTHING(x.to_string())))
            }
            SpecialOne(x) => {
                Some(Atom(TEMPNOTHING(x.to_string())))
            }
            StrLiteral(x) => {
                let s = String::from(x.trim());

                if open_equals_close(&s) {
                    Some(Atom(Str(s)))
                } else {
                    Some(Atom(UnbalancedString(s)))
                }
            }
            CommentToken(comment) => {
                let c = comment.clone();
                Some(Atom(Comment(c)))
            }
            NonSpecial(x) => {
                let t = x.trim();
                let s = String::from(t);

                lazy_static! {
                    static ref SYMBOLS: Regex = Regex::new(r"^[\+\-\*/]$").unwrap();
                }

                lazy_static! {
                    static ref NUMBERS: Regex = Regex::new(r"^(-?)[0-9]+$").unwrap();
                }

                if SYMBOLS.is_match(&s) {
                    //print!("sym #{}# ", s);
                    Some(Atom(Symbol(s, None)))
                }else if s.len() == 0 {
                    None
                }else if s == "def!"{
                    Some(Atom(Def))
                }else if s == "let*"{
                    None
                } else {

                    //print!("else #{}# ", s);
                    //this assumes only integers
                    if NUMBERS.is_match(&s){
                        if let Ok(i) = s.parse::<isize>(){
                            Some(Atom(Integer(i)))
                        }else{
                            panic!("nan")
                        }
                    }
                    else{
                        Some(Atom(Something(s)))
                    }
                }
            }
        }
    } else {
        None
    }
}

fn open_equals_close(s: &str) -> bool {
    if s.len() > 0 {
        let first = s.chars().next().unwrap();
        if first != '"' && first != '\'' {
            return true; // todo move to separate fn
        }
        let last = s.chars().rev().next().unwrap();
        first == last
    } else {
        true
    }
}

fn to_ast_list(reader: &mut Reader) -> Option<AST> {
    let mut items: Vec<AST> = Vec::new();
    let res: Option<AST>;
    let open_brace = match reader.peek().unwrap() {
        SpecialOne(x) => *x,
        _ => panic!(),
    };
    loop {
        let token = reader.next();
        if let Some(token) = token {
            match token {
                SpecialOne(c) => {
                    let c = *c;
                    if c == ')' {
                        // todo check close brace match
                        res = Some(List(Box::new(items)));
                        break;
                    }
                    if c == ']' {
                        res = Some(Vector(Box::new(items)));
                        break;
                    }
                    if c == '}' {
                        res = Some(HashMap(Box::new(items)));
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
            items.push(Atom(UnbalancedListEnd));
            res = Some(List(Box::new(items)));
            break;
        }
    }
    res
}

fn to_ast_quote(reader: &mut Reader) -> Option<AST> {
    let quote: String = match reader.peek().unwrap() {
        SpecialOne(c) => (*c).to_string(),
        SpecialTwo(s2) => s2.to_string(),
        _ => panic!(),
    };

    let mut v: Vec<AST> = Vec::new();

    if quote == "'" {
        v.push(Atom(Quote));
    } else if quote == "`" {
        v.push(Atom(QuasiQuote));
    } else if quote == "~" {
        v.push(Atom(UnQuote));
    } else if quote == "~@" {
        v.push(Atom(SpliceUnQuote));
    }
    if let Some(token) = reader.next() {
        if let Some(ast) = start_to_ast(reader) {
            v.push(ast);
        } else {
            v.push(Atom(UnbalancedListEnd));
        }
        Some(List(Box::new(v)))
    } else {
        None // todo incomplete quote
    }
}

fn to_ast_deref(reader: &mut Reader) -> Option<AST> {
    if let Some(token) = reader.next() {
        match token {
            NonSpecial(x) => Some(Atom(Deref(x.to_string()))),
            _ => None,
        }
    } else {
        Some(Atom(IncompleteDeref))
    }
}

fn to_ast_metadata(reader: &mut Reader) -> Option<AST> {
    if let Some(token) = reader.next() {
        let mut v: Vec<AST> = Vec::new();
        v.push(Atom(Meta));
        let ast = start_to_ast(reader);
        if let Some(ast) = ast {
            v.push(ast);
        }
        Some(List(Box::new(v)))
    } else {
        None
    }
}

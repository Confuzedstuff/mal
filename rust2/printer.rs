use crate::types::*;
use crate::types::MalToken::*;
use crate::types::AST::*;
use crate::types::MalType::*;
use crate::env::Env;

pub fn pr_str(ast: &AST, env: &Env) {
    match ast {
        Atom(atom) => {
            print_atom(atom, env)
        }
        List(list) => {
            print!("(");
            let mut open = true;
            for x in list.iter() {
                if !open {
                    print!(" ");
                }
                open = false;
                pr_str(x, env)
            }
            print!(")");
        }
        Vector(vector) => {
            print!("[");
            let mut open = true;
            for x in vector.iter() {
                if !open {
                    print!(" ");
                }
                open = false;
                pr_str(x, env)
            }
            print!("]");
        }
        HashMap(hashmap) => {
            print!("{{");
            let mut open = true;
            for x in hashmap.iter() {
                if !open {
                    print!(" ");
                }
                open = false;
                pr_str(x, env)
            }
            print!("}}");
        }
    }
}

fn print_atom(atom: &MalType, env: &Env) {
    let mut to_print = atom;
    if let Some(value) = env.find(atom) {
        to_print = value;
    }
    match to_print {
        Something(s) => {
            print!("{}", s);
        }
        Str(sl) => {
            print!("{}", sl)
        }
        Comment(_) => {
            //ignore comments
        }
        UnbalancedListEnd => {
            print!("unbalanced list")
        }
        Deref(x) => {
            print!("(deref {})", x) //TODO list hack
        }
        Quote => {
            print!("quote")
        }
        QuasiQuote => {
            print!("quasiquote")
        }
        UnQuote => {
            print!("unquote")
        }
        UnbalancedString(_) => {
            print!("unbalanced string")
        }
        SpliceUnQuote => {
            print!("splice-unquote")
        }
        TEMPNOTHING(x) => {
            print!("nothing {}", x)
        }
        IncompleteDeref => {
            print!("incomplete deref")
        }
        Meta => {
            print!("with-meta")
        }
        Integer(i) => {
            print!("{}", *i)
        }
        Symbol(s, _) => {
            print!("{}", s)
        }
        Def => {
            print!("def")
        }
        Func(func) => {
            print!("func {:?}", func)
        }
        x => {
            print!("{:?}", x)
        }
    }
}
use crate::types::{MalToken, MalSimpleAST, MalType};

pub fn pr_str(ast: &MalSimpleAST) {
    match ast {
        MalSimpleAST::MalAtom(atom) => {
            print_atom(atom)
        }
        MalSimpleAST::MalList(list) => {
            print!("(");
            let mut open = true;
            for x in list.iter() {
                if !open {
                    print!(" ");
                }
                open = false;
                pr_str(x)
            }
            print!(")");
        }
        MalSimpleAST::Vector(vector) => {
            print!("[");
            let mut open = true;
            for x in vector.iter() {
                if !open {
                    print!(" ");
                }
                open = false;
                pr_str(x)
            }
            print!("]");
        }
        MalSimpleAST::HashMap(hashmap) => {
            print!("{{");
            let mut open = true;
            for x in hashmap.iter() {
                if !open {
                    print!(" ");
                }
                open = false;
                pr_str(x)
            }
            print!("}}");
        }
    }
}

fn print_atom(atom: &MalType) {
    match atom {
        MalType::String(s) => {
            print!("{}", s);
        }
        MalType::StringLiteral(sl) => {
            print!("{}", sl)
        }
        MalType::Comment(comment) => {
            //ignore comments
        }
        MalType::UnbalancedListEnd => {
            print!("unbalanced list")
        }
        MalType::Deref(_) => {
            print!("deref ")
        }
        MalType::Quote => {
            print!("quote")
        }
        MalType::QuasiQuote => {
            print!("quasiquote")
        }
        MalType::UnQuote => {
            print!("unquote")
        }
        MalType::UnbalancedString(s) => {
            print!("unbalanced string")
        }
    }
}
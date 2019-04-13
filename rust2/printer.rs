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
    }
}

fn print_atom(atom: &MalType) {
    match atom {
        MalType::String(s) => {
            print!("{}", s);
        }
    }
}
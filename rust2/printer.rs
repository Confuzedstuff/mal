use crate::types::{MalToken, MalSimpleAST, MalType};

pub fn pr_str(ast: &Option<MalSimpleAST>){
    if let Some(ast) = ast{
        match ast {
            MalSimpleAST::MalAtom(atom) => {
                match atom {
                    MalType::String(s) => {
                        print!("{}", s);
                    },
                }
            },
            MalSimpleAST::MalList(_) => {
                print!("nothing");
            },
        }
    }
    println!("");

}
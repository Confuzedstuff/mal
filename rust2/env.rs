use crate::types::{ReplEnv, EnvFunc, MalType};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Env {
    outer: Option<Box<Env>>,
    data: ReplEnv,
}


impl Env {
    pub fn set(&mut self, key: &MalType, mal_type: MalType) {
        *self.data.entry(key.clone()).or_insert(mal_type) = mal_type.clone();
    }

    pub fn find(&self, key: &MalType) -> Option<&MalType> {
        self.data.get(key)
            .or_else(|| {
                match &self.outer {
                    None => {
                        None
                    },
                    Some(outer) => {
                        outer.find(key)
                    },
                }
            })
    }

    pub fn new(outer: Option<Box<Env>>) -> Env{
        Env {
            outer,
            data: HashMap::new()
        }
    }

}

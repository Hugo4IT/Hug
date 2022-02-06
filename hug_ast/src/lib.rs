use std::fmt::Display;

use hug_lexer::{parser::TokenPair, tokenizer::Ident};
use parser::HugTreeParser;

pub mod parser;

#[derive(Debug, Clone)]
pub enum HugTreeEntry {
    Noop,
    ModuleDefinition {
        module: Ident,
    },
    ExternalModuleDefinition {
        module: Ident,
        location: String,
    },
    VariableDefinition { variable: Ident },
}

pub struct HugTree {
    pub entries: Vec<HugTreeEntry>,
}

impl HugTree {
    pub fn new() -> HugTree {
        HugTree {
            entries: Vec::new()
        }
    }

    pub fn merge_with(&mut self, other: HugTree) {
        self.entries.extend(other.entries.into_iter());
    }

    pub fn from_token_pairs(pairs: Vec<TokenPair>) -> HugTree {
        HugTreeParser::new(pairs).parse()
    }
}

impl Display for HugTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();
        for entry in self.entries.iter() {
            buffer.push_str(&format!("    {:?},\n", entry));
        }
        write!(f, "[\n{}]", buffer)
    }
}

pub struct HugType {}

use std::{collections::HashMap, vec::IntoIter};

use hug_lexer::{
    parser::TokenPair,
    tokenizer::{AnnotationKind, KeywordKind, LiteralKind, TokenKind},
    FilterUseless,
};
use hug_lib::{value::HugValue, Ident};

use crate::{HugTree, HugTreeEntry, HugTreeFunctionCallArg};

#[derive(Debug)]
pub struct HugTreeAnnotationState {
    is_extern: bool,
    extern_location: String,
    custom: HashMap<Ident, HashMap<String, (LiteralKind, String)>>,
}

impl HugTreeAnnotationState {
    pub fn new() -> HugTreeAnnotationState {
        HugTreeAnnotationState {
            is_extern: false,
            extern_location: String::new(),
            custom: HashMap::new(),
        }
    }

    pub fn reset(&mut self) {
        self.is_extern = false;
        self.extern_location.clear();
        self.custom.clear();
    }

    #[inline]
    pub fn push_custom(&mut self, key: Ident, value: HashMap<String, (LiteralKind, String)>) {
        self.custom.insert(key, value).unwrap();
    }

    #[inline]
    pub fn get_custom(&mut self, key: Ident) -> Option<&HashMap<String, (LiteralKind, String)>> {
        self.custom.get(&key)
    }

    pub fn set_extern(&mut self, location: String) {
        self.is_extern = true;
        self.extern_location = location;
    }

    pub fn get_extern(&self) -> Option<String> {
        if self.is_extern {
            if !self.extern_location.is_empty() {
                Some(self.extern_location.clone())
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct HugTreeParser {
    tree: HugTree,
    pairs: IntoIter<TokenPair>,
    annotation_state: HugTreeAnnotationState,
}

impl HugTreeParser {
    pub fn new(pairs: Vec<TokenPair>) -> HugTreeParser {
        HugTreeParser {
            annotation_state: HugTreeAnnotationState::new(),
            pairs: pairs.filter_useless().into_iter(),
            tree: HugTree {
                entries: Vec::new(),
            },
        }
    }

    pub fn next(&mut self) -> Option<TokenPair> {
        self.pairs.next()
    }

    pub fn peek_next(&mut self) -> Option<TokenPair> {
        self.pairs.clone().next()
    }

    pub fn annotation(&mut self, kind: AnnotationKind) -> Option<HugTreeEntry> {
        let mut vars: HashMap<String, (LiteralKind, String)> = HashMap::new();

        if self.peek_next().unwrap().token.kind == TokenKind::OpenParenthesis {
            self.next(); // (

            loop {
                self.peek_next().unwrap().token.kind.expect_ident().unwrap();
                let name = self.next().unwrap().text;

                self.next()
                    .unwrap()
                    .token
                    .kind
                    .expect_kind(TokenKind::Assign)
                    .unwrap();

                let value_pair = self.next().unwrap();
                let value_kind = value_pair.token.kind.expect_literal().unwrap();
                let value = value_pair.text;
                let value = value[1..value.len() - 1].to_string();

                vars.insert(name, (value_kind, value));

                if self.next().unwrap().token.kind == TokenKind::CloseParenthesis {
                    break;
                }
            }
        }

        if vars.keys().len() > 0 {
            match kind {
                AnnotationKind::Extern => self
                    .annotation_state
                    .set_extern(vars.remove("location").unwrap().1),
                AnnotationKind::Other(id) => self.annotation_state.push_custom(id, vars),
            }
        } else {
            match kind {
                AnnotationKind::Extern => self.annotation_state.set_extern("".to_string()),
                AnnotationKind::Other(id) => self.annotation_state.push_custom(id, vars),
            }
        }

        self.next_entry() // An annotation isn't an AST entry by itself, it supports the following entry
    }

    pub fn keyword(&mut self, kind: KeywordKind) -> Option<HugTreeEntry> {
        match kind {
            // KeywordKind::Enum => todo!(),
            KeywordKind::Function => {
                if self.annotation_state.is_extern {
                    Some(HugTreeEntry::ExternalFunctionDefinition {
                        function: self.next().unwrap().token.kind.expect_ident().unwrap(),
                    })
                } else {
                    todo!()
                }
            }
            KeywordKind::Let => Some(self.variable_definition()),
            KeywordKind::Module => {
                if let Some(location) = self.annotation_state.get_extern() {
                    Some(HugTreeEntry::ExternalModuleDefinition {
                        location,
                        module: self.next().unwrap().token.kind.expect_ident().unwrap(),
                    })
                } else {
                    todo!() // TODO: Non-@extern modules not implemented yet.
                }
            }
            // TODO: KeywordKind::Private => todo!(),
            // TODO: KeywordKind::Public => todo!(),
            KeywordKind::Type => {
                if self.annotation_state.is_extern {
                    Some(HugTreeEntry::ExternalTypeDefinition {
                        _type: self.next().unwrap().token.kind.expect_ident().unwrap(),
                    })
                } else {
                    todo!() // TODO: Write non-extern type
                }
            }
            // KeywordKind::Use => todo!(),
            _ => None,
        }
    }

    pub fn identifier(&mut self, id: Ident) -> HugTreeEntry {
        let next = self.next().unwrap();
        match next.token.kind {
            TokenKind::Dot => {
                // TODO: Accessing fields
                todo!()
            }
            TokenKind::OpenParenthesis => {
                // TODO: Calling functions
                let mut args = Vec::new();
                loop {
                    let _next = self.next().unwrap();
                    if let Some(value) = _next.parse_literal() {
                        args.push(HugTreeFunctionCallArg::Value(value));
                    } else if let Some(value) = _next.token.kind.expect_ident() {
                        args.push(HugTreeFunctionCallArg::Variable(value));
                    } else if let TokenKind::CloseParenthesis = _next.token.kind {
                        break;
                    }
                }

                HugTreeEntry::FunctionCall { function: id, args }
            }
            TokenKind::Assign => {
                // TODO: Assigning values to existing variables
                todo!()
            }
            _ => panic!("Unexpected token after identifier: {:?}", next),
        }
    }

    pub fn variable_definition(&mut self) -> HugTreeEntry {
        let name = self.next().unwrap();
        let name = name.token.kind.expect_ident().unwrap();

        let next = self.next().unwrap();
        match next.token.kind {
            TokenKind::Assign => {
                let value = self.next().unwrap();
                let value = value.parse_literal().unwrap();
                HugTreeEntry::VariableDefinition {
                    variable: name,
                    value,
                }
            }
            TokenKind::Colon => todo!(),
            _ => panic!("Unexpected token at variable definition: {:?}", next),
        }
    }

    pub fn next_entry(&mut self) -> Option<HugTreeEntry> {
        if let Some(pair) = self.next() {
            match pair.token.kind {
                // TokenKind::Literal(_) => todo!(),
                TokenKind::Keyword(kind) => self.keyword(kind),
                TokenKind::Identifier(id) => Some(self.identifier(id)),
                TokenKind::Annotation(kind) => self.annotation(kind),
                // TokenKind::Dot => todo!(),
                // TokenKind::OpenParenthesis => todo!(),
                // TokenKind::CloseParenthesis => todo!(),
                // TokenKind::OpenBrace => todo!(),
                // TokenKind::CloseBrace => todo!(),
                // TokenKind::OpenBracket => todo!(),
                // TokenKind::CloseBracket => todo!(),
                // TokenKind::Colon => todo!(),
                // TokenKind::Assign => todo!(),
                // TokenKind::Add => todo!(),
                // TokenKind::Subtract => todo!(),
                // TokenKind::Multiply => todo!(),
                // TokenKind::Divide => todo!(),
                // TokenKind::Modulus => todo!(),
                // TokenKind::AddAssign => todo!(),
                // TokenKind::SubtractAssign => todo!(),
                // TokenKind::MultiplyAssign => todo!(),
                // TokenKind::DivideAssign => todo!(),
                // TokenKind::ModulusAssign => todo!(),
                // TokenKind::Not => todo!(),
                // TokenKind::And => todo!(),
                // TokenKind::Or => todo!(),
                // TokenKind::IsEqualTo => todo!(),
                // TokenKind::IsNotEqualTo => todo!(),
                // TokenKind::LessThan => todo!(),
                // TokenKind::GreaterThan => todo!(),
                // TokenKind::LessThanOrEquals => todo!(),
                // TokenKind::GreaterThanOrEquals => todo!(),
                // TokenKind::BinaryAnd => todo!(),
                // TokenKind::BinaryOr => todo!(),
                // TokenKind::BinaryNot => todo!(),
                // TokenKind::BinaryXOr => todo!(),
                // TokenKind::BinaryAndAssign => todo!(),
                // TokenKind::BinaryOrAssign => todo!(),
                // TokenKind::BinaryNotAssign => todo!(),
                // TokenKind::BinaryXOrAssign => todo!(),
                // TokenKind::ShiftLeft => todo!(),
                // TokenKind::ShiftRight => todo!(),
                // TokenKind::ShiftLeftOverflow => todo!(),
                // TokenKind::ShiftRightOverflow => todo!(),
                TokenKind::Unknown => panic!("Unknown token: {}!", pair.text),
                _ => self.next_entry(),
                // _ => unreachable!(),
            }
        } else {
            self.next_entry()
        }
    }

    pub fn parse(mut self) -> HugTree {
        self.annotation_state.reset();
        while self.pairs.as_slice().len() > 0 {
            self.annotation_state.reset();
            if let Some(entry) = self.next_entry() {
                self.tree.entries.push(entry);
            } else {
                break;
            }
        }

        self.tree
    }
}

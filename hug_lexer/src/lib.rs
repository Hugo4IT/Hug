use parser::{generate_pairs, TokenPair};
use tokenizer::{Token, TokenKind, Tokenizer};

pub mod parser;
pub mod tokenizer;

pub trait FilterUseless {
    fn filter_useless(self) -> Self;
}

impl FilterUseless for Vec<TokenPair> {
    fn filter_useless(self) -> Self {
        let mut new_self = Vec::with_capacity(self.capacity());

        for pair in self {
            match pair.token.kind {
                TokenKind::LineComment | TokenKind::BlockComment | TokenKind::Whitespace => (),
                _ => new_self.push(TokenPair {
                    text: pair.text.clone(),
                    token: pair.token,
                }),
            }
        }

        new_self.shrink_to_fit();
        new_self
    }
}

pub fn lex(program: &str) -> Vec<TokenPair> {
    let tokens = tokenize(program);
    generate_pairs(program, tokens)
}

pub fn tokenize(program: &str) -> Vec<Token> {
    Tokenizer::new(program).tokenize()
}

pub fn run_test(program: &str, expected_result: &[(TokenKind, usize)]) {
    let tokens = tokenize(program);
    for (token, (expected_kind, expected_len)) in tokens.iter().zip(expected_result.iter()) {
        println!(
            "Token: {:?}, Expected: {:?}, len: {}",
            *token, expected_kind, expected_len
        );
        assert_eq!(token.kind, *expected_kind);
        assert_eq!(token.len, *expected_len);
    }
}

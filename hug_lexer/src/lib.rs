use tokenizer::{Tokenizer, TokenKind, Token};

pub mod tokenizer;

pub fn tokenize(program: &str) -> Vec<Token> {
    Tokenizer::new(program).tokenize()
}

pub fn run_test(program: &str, expected_result: &[(TokenKind, usize)]) {
    let tokens = tokenize(program);
    for (token, (expected_kind, expected_len)) in tokens.iter().zip(expected_result.iter()) {
        println!("Token: {:?}, Expected: {:?}, len: {}", *token, expected_kind, expected_len);
        assert_eq!(token.kind, *expected_kind);
        assert_eq!(token.len, *expected_len);
    }
}
use crate::tokenizer::Token;

#[derive(Debug, Clone)]
pub struct TokenPair {
    pub text: String,
    pub token: Token,
}

pub fn generate_pairs(program: &str, tokens: Vec<Token>) -> Vec<TokenPair> {
    let mut pairs = Vec::new();

    let mut chars = program.chars();
    for token in tokens {
        let mut buffer = String::new();
        for _i in 0..token.len {
            buffer.push(chars.next().unwrap());
        }

        pairs.push(TokenPair {
            text: buffer,
            token
        })
    }

    pairs
}
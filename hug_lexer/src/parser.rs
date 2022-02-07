use hug_lib::value::HugValue;

use crate::tokenizer::Token;

#[derive(Debug, Clone)]
pub struct TokenPair {
    pub text: String,
    pub token: Token,
}

impl TokenPair {
    pub fn parse_literal(&self) -> Option<HugValue> {
        if let Some(_) = self.token.kind.expect_literal() {
            if let Ok(int) = self.text.parse::<i32>() {
                Some(HugValue::from(int))
            } else if let Ok(float) = self.text.parse::<f32>() {
                Some(HugValue::from(float))
            } else if self.text.len() > 2 {
                Some(HugValue::from(String::from(
                    &self.text[1..self.text.len() - 1],
                )))
            } else {
                None
            }
        } else {
            None
        }
    }
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
            token,
        })
    }

    pairs
}

use hug_lexer::{
    lex,
    tokenizer::{KeywordKind, TokenKind}, FilterUseless,
};
use hug_lib::Script;

const UNEXPECTED_EOF: &str = "Unexpected end of file!";

#[derive(Debug, Clone)]
pub enum Variable {
    Integer(i32),
    BigInteger(i64),
    PositiveInteger(u32),
    BigPositiveInteger(u64),
    Float(f32),
    BigFloat(f64),
    String(String),
    Pointer(usize),
}

#[derive(Debug, Clone)]
pub enum Symbol {
    Variable { name: String, var: Variable },
}

pub fn transpile(program: String) -> Script {
    let mut script = Script::empty();
    let token_pairs = lex(&program).filter_useless();
    let mut iterator = token_pairs.iter();

    while let Some(pair) = iterator.next() {
        match pair.token.kind {
            TokenKind::Keyword(word) => match word {
                KeywordKind::Var => {
                    let name = iterator.next().expect(UNEXPECTED_EOF);
                    let name = if let TokenKind::Identifier = name.token.kind {
                        name.text.clone()
                    } else {
                        eprintln!("Invalid variable name: {}", name.text);
                        "ERROR_INVALID".into()
                    };

                    let next = iterator.next().expect(UNEXPECTED_EOF);
                    let vtype = match next.token.kind {
                        TokenKind::Assign => "auto".into(),
                        TokenKind::Colon => {
                            let vtype = iterator.next().expect(UNEXPECTED_EOF);
                            if let TokenKind::Identifier = vtype.token.kind {
                                vtype.text.clone()
                            } else {
                                eprintln!("Invalid variable type: {}", vtype.text);
                                "ERROR_INVALID".into()
                            }
                        },
                        _ => {
                            eprintln!("Unexpected token: {}", next.text);
                            "ERROR_INVALID".into()
                        },
                    };

                    println!("Creating variable with name {} and type {}", name, vtype);
                }
            },
            // TokenKind::Literal(_) => todo!(),
            // TokenKind::Identifier => todo!(),
            // TokenKind::SemiColon => todo!(),
            // TokenKind::Comma => todo!(),
            // TokenKind::Dot => todo!(),
            // TokenKind::OpenParenthesis => todo!(),
            // TokenKind::CloseParenthesis => todo!(),
            // TokenKind::OpenBrace => todo!(),
            // TokenKind::CloseBrace => todo!(),
            // TokenKind::OpenBracket => todo!(),
            // TokenKind::CloseBracket => todo!(),
            // TokenKind::Colon => todo!(),
            // TokenKind::Annotation => todo!(),
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
            TokenKind::Unknown => eprintln!("Unknown token {}!", pair.text),
            _ => println!("Unhandled token: {}", pair.text),
        }
    }

    script
}

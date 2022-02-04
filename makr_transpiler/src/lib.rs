use std::str::Chars;

use makr_lib::Script;

const DELIMITERS: [char; 8] = ['+', '-', '*', '/', '=', '\'', '"', ';'];

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Keyword {
    Var,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Assign,
}

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
    Variable {
        name: String,
        var: Variable
    },
}

pub fn get_next_label(chars: &mut Chars) -> String {
    let mut buffer = String::new();
    while let Some(ch) = chars.next() {
        if !ch.is_alphabetic() {
            return buffer
        } else {
            buffer.push(ch);
        }
    }
    
    panic!("Unexpected end of file while parsing label!");
}

pub fn get_next_string(chars: &mut Chars) -> String {
    let mut buffer = String::new();
    while let Some(ch) = chars.next() {
        match ch {
            '\\' => buffer.push(chars.next().unwrap()),
            '"' => break,
            other => buffer.push(ch),
        }
    }
    buffer
}

pub fn get_next_variable(chars: &mut Chars) -> Variable {
    while let Some(ch) = chars.next() {
        if !ch.is_whitespace() {
            return match ch {
                '"' => Variable::String(get_next_string(chars)),
                '0'..='9' => {
                    let mut buffer = String::new();
                    buffer.push(ch);

                    while let Some(ch) = chars.next() {
                        if ch.is_numeric() {
                            buffer.push(ch);
                        } else {
                            if let Ok(parsed) = buffer.parse::<i32>() {
                                return Variable::Integer(parsed)
                            } else {
                                panic!("Invalid integer: {}", buffer);
                            }
                        }
                    }

                    panic!("Unexpected end of file while parsing variable!");
                },
                _ => panic!("Unrecognized variable staring with {}", ch),
            }
        }
    }

    panic!("Unexpected end of file while parsing variable!");
}

pub fn get_next_symbol(chars: &mut Chars) -> Option<Symbol> {
    let mut buffer = String::new();
    loop {
        if let Some(ch) = chars.next() {
            if !ch.is_alphabetic() {
                match buffer.as_str() {
                    "var" => {

                    }
                }
                buffer.clear();
            } else {
                buffer.push(ch);
            }
        }
    }
}

pub fn transpile(program: String) -> Script {
    let mut script = Script::empty();
    let mut chars = program.chars();

    while let Some(symbol) = get_next_symbol(&mut chars) {
        println!("Symbol: {:?}", symbol);
    }

    script
}
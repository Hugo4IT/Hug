use hug_lib::Script;
use hug_lexer::lex;

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


pub fn transpile(program: String) -> Script {
    let mut script = Script::empty();
    let token_pairs = lex(&program);

    for token in token_pairs.iter() {
        println!("{:?}", token);
    }

    script
}
use hug_lib::Script;
use hug_tokenizer::Tokenizer;

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
    let tokens = Tokenizer::new(&program).tokenize();

    for token in tokens.iter() {
        println!("{:?}", token);
    }

    script
}
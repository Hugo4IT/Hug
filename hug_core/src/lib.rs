use hug_lib::{hug_module, unwrap_args, value::HugValue};

pub const HUG_CORE_SCRIPT: &str = include_str!("../hug/core.hug");

hug_module!(init);
pub fn init(module: &mut HugModule) {
    module.register_function("add", add);
    module.register_function("print", print);
}

fn add(mut args: std::vec::IntoIter<HugValue>) -> Option<HugValue> {
    let (left, right) = unwrap_args!(args, i32, i32);

    println!("Added: {:?}", HugValue::from(left + right));

    Some(HugValue::from(left + right))
}

fn print(args: std::vec::IntoIter<HugValue>) -> Option<HugValue> {
    println!("{}", args.map(|v|v.to_string()).collect::<String>());

    None
}
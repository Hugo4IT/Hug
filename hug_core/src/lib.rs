use hug_lib::{hug_module, unwrap_args, value::HugValue};

pub const HUG_CORE_SCRIPT: &str = include_str!("../hug/core.hug");

pub fn init(module: &mut HugModule) {
    println!("Registering...");
    module.register_function("add", add);
    module.register_function("print", print);
    println!("HUG CORE LOADED!!!!");
}

pub fn deinit(module: &mut HugModule) {
    println!("rip hug");
}

fn add(mut args: std::vec::IntoIter<HugValue>) -> Option<HugValue> {
    let (left, right) = unwrap_args!(args, i32, i32);

    println!("Added: {:?}", HugValue::from(left + right));

    Some(HugValue::from(left + right))
}

fn print(mut args: std::vec::IntoIter<HugValue>) -> Option<HugValue> {
    let fmt = args
        .next()
        .expect("Nothing to print!")
        .assert::<String>()
        .expect("First argument of print must be a string!");

    let fmt_args = args.collect::<Vec<HugValue>>();

    None
}

hug_module!(init, deinit);

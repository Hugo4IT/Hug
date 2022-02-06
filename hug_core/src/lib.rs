use hug_lib::hug_module;

pub const HUG_CORE_SCRIPT: &str = include_str!("../hug/core.hug");

pub fn init(module: &mut HugModule) {
    println!("HUG CORE LOADED!!!!");
}

pub fn deinit(module: &mut HugModule) {
    println!("rip hug");
}

hug_module!(init, deinit);
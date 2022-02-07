pub use paste::paste;
use std::collections::HashMap;
use value::HugValue;

pub mod value;

#[macro_export]
macro_rules! hug_module {
    ($init:path, $deinit:path) => {
        use hug_lib::HugModule;

        #[no_mangle]
        extern "C" fn __HUG_MODULE_INIT(module: &mut HugModule) {
            $init(module);
        }

        #[no_mangle]
        extern "C" fn __HUG_MODULE_DEINIT(module: &mut HugModule) {
            $deinit(module);
        }
    };
}

#[macro_export]
macro_rules! unwrap_args {
    ($input:ident, $($args:ty),+) => {
        (
            $($input.next().expect(&format!("Not enought arguments for function {}!", stringify!($input))).assert::<$args>().unwrap()),+
        );
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ident(pub usize);

pub struct HugModule<'a> {
    pub functions: HashMap<Ident, fn(std::vec::IntoIter<HugValue>) -> Option<HugValue>>,
    idents: &'a mut HashMap<String, Ident>,
}

impl<'a> HugModule<'a> {
    pub fn new(idents: &mut HashMap<String, Ident>) -> HugModule {
        HugModule {
            functions: HashMap::new(),
            idents,
        }
    }

    pub fn register_function(
        &mut self,
        name: &str,
        func: fn(std::vec::IntoIter<HugValue>) -> Option<HugValue>,
    ) {
        if let Some(id) = self.idents.get(name) {
            self.functions.insert(*id, func);
        } else {
            println!(
                "The function \"{0}\" was registered before it was defined. \
                      Define it with \"@export function {0};\"",
                name
            );
        }
        assert!(
            self.idents.contains_key(name),
            "Define the function \"{}\" first with @export function before registering it!",
            name
        );
    }
}

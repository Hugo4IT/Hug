use std::{collections::HashMap, fs::OpenOptions, io::Read};

use hug_ast::HugTree;
use hug_core::HUG_CORE_SCRIPT;
use hug_lexer::{
    parser::generate_pairs,
    tokenizer::{Ident, Tokenizer},
};
use hug_lib::HugModule;

const INVALID_MODULE_ERROR: &str = "No function __HUG_MODULE_INIT was found on this module, add one with hug_module! or contact the module's developer.";

pub(crate) struct HugModuleHolder {
    library: libloading::Library,
    module: HugModule,
}

impl HugModuleHolder {
    pub(crate) unsafe fn new(path: String) -> HugModuleHolder {
        let library = libloading::Library::new(path).unwrap();
        let init_func: libloading::Symbol<
            unsafe extern "C" fn(&mut HugModule),
        > = library
            .get(b"__HUG_MODULE_INIT")
            .expect(INVALID_MODULE_ERROR);
        
        let mut module = HugModule::new();
        init_func(&mut module);

        HugModuleHolder {
            library,
            module
        }
    }
}

pub struct HugVM {
    paused: bool,
    pointer: usize,
    tree: HugTree,
    idents: HashMap<String, Ident>,
    external_modules: HashMap<Ident, HugModuleHolder>,
}

impl HugVM {
    pub fn new(file_path: &str) -> HugVM {
        let mut vm = HugVM {
            paused: false,
            pointer: 0,
            tree: HugTree::new(),
            idents: HashMap::new(),
            external_modules: HashMap::new(),
        };

        vm.load_script(HUG_CORE_SCRIPT);
        vm.load_file(file_path);
        vm
    }

    pub fn next(&mut self) {
        if !self.paused {
            self.pointer += 1;
        }
    }

    pub fn load_file(&mut self, file_path: &str) {
        let mut file = OpenOptions::new()
            .read(true)
            .open(file_path)
            .expect(format!("Could not open file {}!", file_path).as_str());

        let mut buffer = String::new();
        file.read_to_string(&mut buffer)
            .expect("Could not read file!");

        self.load_script(&buffer);
    }

    pub fn load_script(&mut self, program: &str) {
        let tokens = Tokenizer::with_idents(self.idents.clone(), program).tokenize();
        let pairs = generate_pairs(program, tokens);
        self.tree.merge_with(HugTree::from_token_pairs(pairs));
    }

    pub fn run(&mut self) {
        while self.pointer < self.tree.entries.len() {
            let instruction = self.tree.entries.get(self.pointer).unwrap();
            println!("Instruction: {:?}", instruction);
            match instruction {
                hug_ast::HugTreeEntry::Noop => todo!(),
                hug_ast::HugTreeEntry::ModuleDefinition { module } => todo!(),
                hug_ast::HugTreeEntry::ExternalModuleDefinition { module, location } => {
                    if !self.external_modules.contains_key(module) {
                        unsafe { self.external_modules.insert(*module, HugModuleHolder::new(location.clone())); }
                    }
                }
                hug_ast::HugTreeEntry::VariableDefinition { variable } => todo!(),
            }
            self.next();
        }
    }
}

use std::{collections::HashMap, fs::OpenOptions, io::Read};

use hug_ast::HugTree;
use hug_core::HUG_CORE_SCRIPT;
use hug_lexer::{parser::generate_pairs, tokenizer::Tokenizer};
use hug_lib::{
    value::{HugExternalFunction, HugValue},
    HugModule, Ident,
};

const INVALID_MODULE_ERROR: &str = "No function __HUG_MODULE_INIT was found on this module, add one with hug_module! or contact the module's developer.";

#[derive(Debug)]
pub struct HugVM {
    paused: bool,
    pointer: usize,
    tree: HugTree,
    idents: HashMap<String, Ident>,
    variables: Vec<Option<HugValue>>,
}

impl HugVM {
    pub fn new(file_path: &str) -> HugVM {
        let mut vm = HugVM {
            paused: false,
            pointer: 0,
            tree: HugTree::new(),
            idents: HashMap::new(),
            variables: Vec::new(),
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
        #[cfg(debug_assertions)]
        println!("Loading file: {}", file_path);

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
        #[cfg(debug_assertions)]
        println!("Loading script:\n> {}", program.replace("\n", "\n> "));

        let mut tokenizer = Tokenizer::with_idents(self.idents.clone(), program);
        let tokens = tokenizer.tokenize();
        self.idents = tokenizer.idents;

        let pairs = generate_pairs(program, tokens);
        let t = HugTree::from_token_pairs(pairs);
        self.tree.merge_with(t);
    }

    pub fn run(&mut self) {
        #[cfg(debug_assertions)]
        {
            println!("HugTree: {}", self.tree);
            println!("Identifiers: {}", {
                let mut buffer = String::new();
                for (key, value) in self.idents.iter() {
                    buffer.push_str(&format!("\n  {:?}: \"{}\",", value, key));
                }
                buffer
            });
            println!("Memory: {}", {
                let mut buffer = String::new();
                for (i, value) in self.variables.iter().enumerate() {
                    buffer.push_str(&format!("\n  {}: \"{:?}\",", i, value.clone()));
                }
                buffer
            })
        }

        while self.pointer < self.tree.entries.len() {
            let instruction = self.tree.entries.get(self.pointer).unwrap().clone();

            #[cfg(debug_assertions)]
            println!("Instruction: {:?}", instruction);

            match instruction {
                hug_ast::HugTreeEntry::ModuleDefinition { module } => todo!(),
                hug_ast::HugTreeEntry::ExternalModuleDefinition { module, location } => unsafe {
                    let library = libloading::Library::new(location).unwrap();
                    let init_func: libloading::Symbol<unsafe extern "C" fn(&mut HugModule)> =
                        library
                            .get(b"__HUG_MODULE_INIT")
                            .expect(INVALID_MODULE_ERROR);

                    let mut module = HugModule::new(&mut self.idents);
                    init_func(&mut module);

                    let HugModule { functions, .. } = module;

                    for (id, fun) in functions {
                        self.set_variable(id, HugValue::from(fun));
                    }
                },
                hug_ast::HugTreeEntry::VariableDefinition { variable, value } => {
                    self.set_variable(variable, value.clone());
                }
                hug_ast::HugTreeEntry::FunctionCall { function, args } => {
                    match self.get_variable(function).unwrap() {
                        HugValue::ExternalFunction(f) => {
                            f(args
                                .iter()
                                .map(|a| match a {
                                    hug_ast::HugTreeFunctionCallArg::Variable(v) => {
                                        self.get_variable(*v).unwrap().clone()
                                    }
                                    hug_ast::HugTreeFunctionCallArg::Value(v) => v.clone(),
                                })
                                .collect::<Vec<HugValue>>()
                                .into_iter());
                        }
                        HugValue::Function(l) => {
                            self.pointer = *l;
                        }
                        _ => panic!("Not a function! {:?}", function),
                    }
                }
                _ => (),
            }
            self.next();
        }
    }

    #[inline]
    pub fn enforce_variables_len(&mut self, size: usize) {
        if self.variables.len() < size + 1 {
            self.variables
                .extend((0..(size - self.variables.len() + 1)).map(|_| None));
        }
    }

    #[inline]
    pub fn get_variable(&self, at: Ident) -> Option<&HugValue> {
        self.variables.get(at.0).and_then(|h| h.as_ref())
    }

    #[inline]
    pub fn get_variable_mut(&mut self, at: Ident) -> Option<&mut HugValue> {
        self.enforce_variables_len(at.0);
        self.variables.get_mut(at.0).and_then(|h| h.as_mut())
    }

    #[inline]
    pub fn remove_variable(&mut self, at: Ident) -> Option<HugValue> {
        self.enforce_variables_len(at.0);
        self.variables.get_mut(at.0).unwrap().take()
    }

    #[inline]
    pub fn set_variable(&mut self, at: Ident, value: HugValue) {
        self.enforce_variables_len(at.0);
        let _ = self.variables.get_mut(at.0).unwrap().insert(value);
    }
}

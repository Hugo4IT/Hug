use hug_ast::HugTree;

pub mod vm;

pub fn interpret(ast: HugTree) {
    let mut current_instruction = 0;
    while current_instruction <= ast.entries.len() - 1 {
        match ast.entries.get(current_instruction).unwrap() {
            hug_ast::HugTreeEntry::Noop => (),
            hug_ast::HugTreeEntry::ModuleDefinition { module } => todo!(),
            hug_ast::HugTreeEntry::ExternalModuleDefinition { module, location } => {
                unsafe {
                    let lib = libloading::Library::new(location).unwrap();
                    let init_func: libloading::Symbol<unsafe extern fn()> = lib.get(b"__HUG_MODULE_INIT").unwrap();
                    init_func();
                }
            },
            hug_ast::HugTreeEntry::VariableDefinition { variable } => todo!(),
        }

        current_instruction += 1;
    }
}

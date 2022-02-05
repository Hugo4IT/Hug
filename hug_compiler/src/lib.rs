pub fn compile(program: String) {
    let elf_builder = asmpeach::assemble_code(include_str!("../../test/assembly/hello_world.asm").to_string(), asmpeach::Syntax::ATANDT).unwrap();
    elf_builder.generate_elf_file("bin/test.o", 0o644).unwrap();
}
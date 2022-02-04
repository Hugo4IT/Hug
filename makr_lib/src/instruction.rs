#[derive(Debug, Clone)]
pub enum Instruction {
    Noop,
    WriteStack { buffer: Vec<u8>, at: usize },
    ReadStack { from: usize, to: usize },
}
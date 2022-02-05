use crate::stack::StackAddress;

#[derive(Debug, Clone)]
pub enum Instruction {
    Noop,
    WriteStack { data: u32, location: StackAddress },
    CopyStack { from: StackAddress, to: StackAddress },
}
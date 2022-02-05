use instruction::Instruction;

pub mod instruction;
pub mod stack;

#[derive(Debug, Clone)]
pub struct Script {
    pub instructions: Vec<Instruction>,
    pub stack_size: usize,
}

impl Script {
    pub fn empty() -> Self {
        Self {
            instructions: Vec::new(),
            stack_size: 0,
        }
    }
}
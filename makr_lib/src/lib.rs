use instruction::Instruction;

pub mod instruction;

#[derive(Debug, Clone)]
pub struct Script {
    pub instructions: Vec<Instruction>
}

impl Script {
    pub fn empty() -> Self {
        Self {
            instructions: Vec::new()
        }
    }
}
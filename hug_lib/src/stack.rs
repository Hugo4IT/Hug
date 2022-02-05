#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct StackAddress {
    location: u32,
    data_size: u32,
}
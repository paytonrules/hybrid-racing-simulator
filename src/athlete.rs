#[derive(Default, Clone, PartialEq)]
pub struct Athlete {
    pub name: String,
    pub fatigue: i8,
    pub fitness: u8,
    pub pr: u16,
}

pub const MAX_FITNESS: u8 = 80;

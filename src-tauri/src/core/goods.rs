use shipyard::{self, Component};
use enum_derived::{self, Rand};

#[derive(Rand, Clone, Copy, PartialEq, Debug)]
pub enum Good {
    Apples,
    SulfuricAcid,
    Plastic,
    Tanks,
    Planks
}

impl std::fmt::Display for Good {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Good::Apples => write!(f, "Apples"),
            Good::Planks => write!(f, "Planks"),
            Good::SulfuricAcid => write!(f, "Sulfuric acid"),
            Good::Tanks => write!(f, "Tanks"),
            Good::Plastic => write!(f, "Plastic")
        }
    }
}

#[derive(Component, Debug)]
pub struct Goods(pub Good, pub u32);
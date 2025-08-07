use shipyard::Component;

use crate::core::{goods::Good, Currency, Goods};

pub enum AssetType {
    Headquarters,
    Factory
}

impl std::fmt::Display for AssetType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssetType::Factory => write!(f, "Factory"),
            AssetType::Headquarters => write!(f, "Headquater")
        }
    }
}

#[derive(Component)]
pub struct Headquarters {
    pub location: [f64; 2],
    pub value: Currency
}

#[derive(Component)]
pub struct Factory {
    pub location: [f64; 2],
    pub value: Currency
}

#[derive(Component)]
pub struct AssetBelongsTo(pub u64);


#[derive(Component)]
pub struct AssetId(pub u64);

#[derive(Component, Clone, Copy)]
pub struct Production {
    pub produces: Good,
    pub rate_per_hour: f32
}

#[derive(Component)]
pub struct Inventory(pub Vec<Goods>);

impl Inventory {
    pub fn add(&mut self, good_type: Good, amount: u32) {
        if let Some(existing) = self.0.iter_mut().find(|g| g.0 == good_type) {
            existing.1 += amount;
        } else {
            self.0.push(Goods(good_type, amount));
        }
    }
}
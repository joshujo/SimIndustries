use shipyard::Component;

use crate::core::Currency;

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
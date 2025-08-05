use shipyard::{Component, Unique};

use crate::core::{AssetId, Currency};

#[derive(Component)]
pub struct Company {
    pub name: String, 
    pub money: Currency,
}

#[derive(Component)]
pub struct CompanyId(pub u64);

#[derive(Component)]
pub struct Assets(pub Vec<u64>);

#[derive(Component, Unique)]
pub struct Player(pub u64);
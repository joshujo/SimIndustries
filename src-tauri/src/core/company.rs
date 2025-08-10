use shipyard::{Component, Unique};

use crate::core::{AssetId, CompanyId, Currency};

#[derive(Component)]
pub struct Company {
    pub name: String, 
    pub money: Currency,
}

#[derive(Component)]
pub struct Assets(pub Vec<AssetId>);

#[derive(Component, Unique)]
pub struct Player(pub CompanyId);

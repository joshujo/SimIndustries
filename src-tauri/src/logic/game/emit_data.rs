use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::Serialize;
use shipyard::{Get, View};
use tauri::{AppHandle, Emitter, Manager};

use crate::{core::{Assets, Identification, Inventory, Player, WorldData}, interface::retrieve_data::{RetrieveData, RetrieveType}};



#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlayerCompanyData {
    pub company_name: String,
    pub money: String,
    pub number_of_assets: u64,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DateTime {
    pub day: String,
    pub month: String,
    pub year: String,
    pub hour: String,
    pub minute: String,
    pub second: String,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    #[serde(rename = "type")]
    pub asset_type: String,
    pub value: String,
    pub production: Option<String>,
    pub has_inventory: bool,
    pub id: u64
}

pub fn emit_data(world_data: &WorldData, app: &AppHandle) {
    let state = app.state::<RetrieveData>().retrieve();

    if state.contains(&crate::interface::retrieve_data::RetrieveType::PlayerAssets) {
        let assets = retrieve_player_assets(world_data);
        app.emit("playerAssetData", assets).unwrap_or_default();
    }

    for item in &state {
        if let RetrieveType::Inventory(id) = *item {
            retrieve_inventory_data(world_data, id, app);
        }
    }
}

fn retrieve_player_assets(world_data: &WorldData) -> Vec<Asset> {
    let (_, assets, inventory) = world_data.world
        .borrow::<(View<Player>, View<Assets>, View<Inventory>)>()
        .unwrap();
    let player = world_data.world.get_unique::<&Player>().unwrap();
    let player_id = *world_data.companies.get(&player.0).unwrap();
    let player_assets = &assets.get(player_id).unwrap().0;

    let assets: Vec<_> = player_assets
        .par_iter()
        .map(|k| {
            let asset = world_data.assets.get(k).unwrap();
            let asset_type = asset.asset_type.to_string();
            let value = asset.value.as_string_dollars();
            let production = match asset.production {
                Some(ok) => Some(ok.to_string()),
                None => None,
            };
            let has_inventory = inventory.get(asset.entity_id).is_ok();

            Asset {
                asset_type,
                value,
                production,
                has_inventory,
                id: k.value()
            }
        })
        .collect();
    assets
}

#[derive(Serialize, Clone)]
struct InventoryItem {
    good: String,
    amount: u32
}

fn retrieve_inventory_data(world_data: &WorldData, id: u64, app: &AppHandle) {
    let inventory = world_data.world.borrow::<View<Inventory>>().unwrap();
    let entity_id = match world_data.assets.get(&id) {
        Some(ok) => ok.entity_id,
        None => {
            return ()
        }
    };
    let asset_inventory = match inventory.get(entity_id) {
        Ok(ok) => ok,
        Err(_) => {
            return ()
        }
    };

    let items: Vec<_> = asset_inventory.0.iter().map(|x| {
        let item_name = x.0.to_string();
        let amount = x.1;

        InventoryItem {
            good: item_name,
            amount
        }
    }).collect();

    let payload = items;
    
    app.emit(&format!("inventoryData{id}"), payload).unwrap_or_default();

}
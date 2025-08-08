use parking_lot::Mutex;

use rayon::iter::{ IntoParallelRefIterator, ParallelIterator };
use serde::Serialize;
use shipyard::{ world, EntityId, Get, IntoIter, View };
use tauri::{ AppHandle, Emitter, Manager };

use crate::{
    core::{ Assets, Company, Factory, Inventory, Player, WorldData },
    interface::retrieve_data::{RetrieveData, RetrieveType},
    logic::game::systems::produce_goods,
};

pub fn main(world_data: &mut WorldData, app: &AppHandle) {
    let player_company = update_player_data(world_data, app);
    emit_time(world_data, app);
    if world_data.tick % 10 == 0 {
        produce_goods(world_data);
    }
    emit_data(world_data, app);
}

fn update_player_data(world_data: &mut WorldData, app: &AppHandle) -> EntityId {
    let player = world_data.world.get_unique::<&Player>().unwrap().0;
    let company_id = *world_data.companies.get(&player).unwrap();

    let (companies, assets) = world_data.world.borrow::<(View<Company>, View<Assets>)>().unwrap();

    let player = companies.get(company_id).unwrap();
    let assets = assets.get(company_id).unwrap();

    let money = player.money.as_dollars();

    let data = PlayerCompanyData {
        company_name: player.name.clone(),
        money: format!("${:.2}", money),
        number_of_assets: assets.0.len() as u64,
    };

    app.emit("updatePlayerCompanyData", data).unwrap_or_default();

    company_id
}

fn emit_time(world_data: &mut WorldData, app: &AppHandle) {
    let date = world_data.time.date();
    let time = world_data.time.time();
    let (day, month, year, hour, minute, second) = {
        (
            format!("{:02}", date.day()),
            format!("{:02}", date.month() as usize),
            format!("{:02}", date.year()),
            format!("{:02}", time.hour()),
            format!("{:02}", time.minute()),
            format!("{:02}", time.second()),
        )
    };

    let payload = DateTime {
        day,
        month,
        year,
        hour,
        minute,
        second,
    };

    app.emit("timeAndDate", payload).unwrap_or_default();
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct PlayerCompanyData {
    company_name: String,
    money: String,
    number_of_assets: u64,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct DateTime {
    day: String,
    month: String,
    year: String,
    hour: String,
    minute: String,
    second: String,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct Asset {
    #[serde(rename = "type")]
    asset_type: String,
    value: String,
    production: Option<String>,
    has_inventory: bool
}

fn emit_data(world_data: &mut WorldData, app: &AppHandle) {
    let state = app.state::<Mutex<RetrieveData>>();
    let state = match state.try_lock() {
        Some(ok) => ok,
        None => {
            return;
        }
    };

    if state.data.contains(&crate::interface::retrieve_data::RetrieveType::PlayerAssets) {
        let (_, assets, inventory) = world_data.world
            .borrow::<(View<Player>, View<Assets>, View<Inventory>)>()
            .unwrap();
        let player = world_data.world.get_unique::<&Player>().unwrap();
        let player_id = *world_data.companies.get(&player.0).unwrap();
        let player_assets = &assets.get(player_id).unwrap().0;
        let has_inventory = match &inventory.get(player_id) {
            Ok(_) => true,
            Err(_) => false
        };

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
    
             

                Asset {
                    asset_type,
                    value,
                    production,
                    has_inventory
                }
            })
            .collect();
        app.emit("playerAssetData", assets).unwrap_or_default();
    }

    if let Some(inventory) = state.data.iter().find(|item| matches!(item, RetrieveType::Inventory(_))) {
        if let RetrieveType::Inventory(id) = inventory {

        }
    };
}

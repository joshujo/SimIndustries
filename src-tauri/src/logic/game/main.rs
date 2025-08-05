use serde::Serialize;
use shipyard::{EntityId, Get, View};
use tauri::{AppHandle, Emitter};

use crate::core::{Assets, Company, Player, WorldData};

pub fn main(world_data: &mut WorldData, app: &AppHandle) {
    let mut world = &mut world_data.world;
    let player_company = update_player_data(world_data, app);

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
        number_of_assets: assets.0.len() as u64
    };

    app.emit("updatePlayerCompanyData", data).unwrap_or_default();

    company_id
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct PlayerCompanyData {
    company_name: String,
    money: String,
    number_of_assets: u64,
}
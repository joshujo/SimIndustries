use serde::Serialize;
use shipyard::{EntityId, Get, View, World};
use tauri::{AppHandle, Emitter};

use crate::{core::{Company, Factory, Player}, logic::game::WorldData};

pub fn main(world_data: &mut WorldData, app: &AppHandle) {
    let mut world = &mut world_data.world;
    let player_company = update_player_data(world_data, app);

}

fn update_player_data(world_data: &mut WorldData, app: &AppHandle) -> EntityId {
    let player = world_data.world.get_unique::<&Player>().unwrap().0;
    let company_id = *world_data.companies.get(&player).unwrap();

    let (companies, factories) = world_data.world.borrow::<(View<Company>, View<Factory>)>().unwrap();

    let player = companies.get(company_id).unwrap();
    let money = player.money.as_dollars();

    app.emit("updatePlayerCompanyData", ());

    company_id
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PlayerCompanyData {
    company_name: String,
    money: f32,
    number_of_factories: u64,
}
use std::collections::HashMap;

use rand::{ fill, random_range };
use shipyard::{ EntityId, World };

use crate::new_game::RegisterData;
use crate::core::{
    AssetBelongsTo, AssetId, Assets, Company, CompanyId, Currency, Factory, Headquarters, Player, WorldData, ASSET_ID_GENERATOR, COMPANY_ID_GENERATOR
};

const COMPANIES: u64 = 1000;

pub fn generate_world(player_data: RegisterData) -> WorldData {
    let mut world = World::new();
    let mut assets: HashMap<u64, EntityId> = HashMap::new();
    let mut companies: HashMap<u64, EntityId> = HashMap::new();

    for _company in 0..=COMPANIES {
        generate_company(&mut world, &mut assets, &mut companies);
    }

    generate_player(&mut world, player_data, &mut assets, &mut companies);

    WorldData::new(world, assets, companies)
}

fn generate_company(
    world: &mut World,
    assets: &mut HashMap<u64, EntityId>,
    companies: &mut HashMap<u64, EntityId>
) {
    let id = COMPANY_ID_GENERATOR.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

    let company = world.add_entity((
        Company {
            name: random_range(1..=100000000).to_string(),
            money: Currency::from_dollars(random_range(100000..=25000000)),
        },
        CompanyId(id),
    ));

    companies.insert(id, company);

    let mut company_assets: Vec<u64> = Vec::new();

    let headquarters = {
        let mut location: [f64; 2] = [0f64; 2];

        fill(&mut location);

        let value = Currency::from_dollars(random_range(200000..=2000000));

        Headquarters {
            location,
            value,
        }
    };

    let headquarters_id = ASSET_ID_GENERATOR.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

    let entity_id = world.add_entity((headquarters, AssetId(headquarters_id), AssetBelongsTo(id)));

    company_assets.push(headquarters_id);
    assets.insert(headquarters_id, entity_id);

    let num_factories = random_range(1..=10);

    for _factory in 0..num_factories {
        let asset_id = ASSET_ID_GENERATOR.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let mut location = [0f64; 2];
        fill(&mut location);
        let value = Currency::from_dollars(random_range(250000..=5000000));

        let entity_id = world.add_entity((
            Factory {
                location,
                value,
            },
            AssetId(asset_id),
            AssetBelongsTo(id),
        ));
        company_assets.push(asset_id);
        assets.insert(asset_id, entity_id);
    }

    world.add_component(company, (Assets(company_assets),));
}

fn generate_player(
    world: &mut World,
    player_data: RegisterData,
    assets: &mut HashMap<u64, EntityId>,
    companies: &mut HashMap<u64, EntityId>
) {
    let id = COMPANY_ID_GENERATOR.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

    world.add_unique(Player(id));

    let company = world.add_entity((
        Company {
            name: player_data.company_name,
            money: Currency::from_dollars(random_range(100000..=25000000)),
        },
        CompanyId(id),
    ));

    companies.insert(id, company);

    let mut company_assets: Vec<u64> = Vec::new();

    let headquarters = {
        let mut location: [f64; 2] = [0f64; 2];

        fill(&mut location);

        let value = Currency::from_dollars(random_range(200000..=2000000));

        Headquarters {
            location,
            value,
        }
    };

    let headquarters_id = ASSET_ID_GENERATOR.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

    let entity_id = world.add_entity((headquarters, AssetId(headquarters_id), AssetBelongsTo(id)));

    company_assets.push(headquarters_id);
    assets.insert(headquarters_id, entity_id);

    let num_factories = random_range(1..=10);

    for _factory in 0..num_factories {
        let asset_id = ASSET_ID_GENERATOR.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let mut location = [0f64; 2];
        fill(&mut location);
        let value = Currency::from_dollars(random_range(250000..=5000000));

        let entity_id = world.add_entity((
            Factory {
                location,
                value,
            },
            AssetId(asset_id),
            AssetBelongsTo(id),
        ));
        company_assets.push(asset_id);
        assets.insert(asset_id, entity_id);
    }

    world.add_component(company, (Assets(company_assets),));
}

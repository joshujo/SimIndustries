use std::collections::HashMap;

use rand::{ fill, random_range };
use shipyard::{ EntityId, World };
use enum_derived::{self, Rand};


use crate::{core::{
    AssetBelongsTo, AssetData, AssetId, AssetType, Assets, Company, CompanyId, Currency, Factory, Good, Headquarters, Inventory, Player, Production, WorldData, ASSET_ID_GENERATOR, COMPANY_ID_GENERATOR
}, interface::register::RegisterData};

const COMPANIES: u64 = 1000;

pub fn generate_world(player_data: RegisterData) -> WorldData {
    let mut world = World::new();
    let mut assets: HashMap<u64, AssetData> = HashMap::new();
    let mut companies: HashMap<u64, EntityId> = HashMap::new();

    for _company in 0..COMPANIES {
        generate_company(&mut world, &mut assets, &mut companies);
    }

    generate_player(&mut world, player_data, &mut assets, &mut companies);

    WorldData::new(world, assets, companies)
}

fn generate_company(
    world: &mut World,
    assets: &mut HashMap<u64, AssetData>,
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

    let (headquarters, value) = {
        let mut location: [f64; 2] = [0f64; 2];

        fill(&mut location);

        let value = Currency::from_dollars(random_range(200000..=2000000));

        (Headquarters {
            location,
            value,
        }, value)
    };

    let headquarters_id = ASSET_ID_GENERATOR.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

    let entity_id = world.add_entity((headquarters, AssetId(headquarters_id), AssetBelongsTo(id)));

    let asset_data = AssetData {
        entity_id,
        asset_type: crate::core::AssetType::Headquarters,
        value,
        production: None
    };

    company_assets.push(headquarters_id);
    assets.insert(headquarters_id, asset_data);

    let num_factories = random_range(1..=10);

    for _factory in 0..num_factories {
        let asset_id = ASSET_ID_GENERATOR.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let mut location = [0f64; 2];
        fill(&mut location);
        let value = Currency::from_dollars(random_range(250000..=5000000));
        let good_production = Good::rand();

        let entity_id = world.add_entity((
            Factory {
                location,
                value,
            },
            AssetId(asset_id),
            AssetBelongsTo(id),
            Production {
                produces: good_production,
                rate_per_hour: random_range(60.0..=600.0),
                last_production_tick: 0,
            },
            Inventory(Vec::new())
        ));
        let asset_data = AssetData {
            entity_id,
            asset_type: crate::core::AssetType::Factory,
            value,
            production: Some(good_production)
        };
        company_assets.push(asset_id);
        assets.insert(asset_id, asset_data);
    }

    world.add_component(company, (Assets(company_assets),));
}

fn generate_player(
    world: &mut World,
    player_data: RegisterData,
    assets: &mut HashMap<u64, AssetData>,
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

    let (headquarters, value) = {
        let mut location: [f64; 2] = [0f64; 2];

        fill(&mut location);

        let value = Currency::from_dollars(random_range(200000..=2000000));

        (Headquarters {
            location,
            value,
        }, value)
    };

    let headquarters_id = ASSET_ID_GENERATOR.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

    let entity_id = world.add_entity((headquarters, AssetId(headquarters_id), AssetBelongsTo(id)));

    company_assets.push(headquarters_id);

    let asset_data = AssetData {
        entity_id,
        asset_type: AssetType::Headquarters,
        value,
        production: None
    };

    assets.insert(headquarters_id, asset_data);

    let num_factories = random_range(1..=10);

    for _factory in 0..num_factories {
        let asset_id = ASSET_ID_GENERATOR.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let mut location = [0f64; 2];
        fill(&mut location);
        let value = Currency::from_dollars(random_range(250000..=5000000));
        let production = Production {
            produces: Good::rand(),
            rate_per_hour: random_range(2000.0..=3000.0),
            last_production_tick: 0
        };

        let entity_id = world.add_entity((
            Factory {
                location,
                value,
            },
            AssetId(asset_id),
            AssetBelongsTo(id),
            production,
            Inventory(Vec::new())
        ));
        company_assets.push(asset_id);

        let asset_data = AssetData {
            entity_id,
            asset_type: AssetType::Factory,
            value,
            production: Some(production.produces)
        };

        assets.insert(asset_id, asset_data);
    }

    world.add_component(company, (Assets(company_assets),));
}

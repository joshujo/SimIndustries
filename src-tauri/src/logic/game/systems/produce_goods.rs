use shipyard::{IntoIter, View, ViewMut};
use rayon::prelude::*;

use crate::core::{AssetBelongsTo, Factory, Inventory, Production, WorldData};

pub fn produce_goods(world_data: &mut WorldData) {
    let world = &mut world_data.world;

    let (mut inventory, asset_belongs_to, factory, mut production) = world.borrow::<(ViewMut<Inventory>, View<AssetBelongsTo>, View<Factory>, ViewMut<Production>)>().unwrap();

    (&mut inventory, &asset_belongs_to, &factory, &mut production).par_iter()
        .for_each(|(inventory, asset_belongs_to, _, production)| {
            if asset_belongs_to.0 == 0 {
                return;
            }

            let production_type = production.produces;

            let tick_per_production = (60.0 / world_data.time_scale as f32 * 3600.0) / production.rate_per_hour;

            let ticks_since_last = world_data.tick - production.last_production_tick;

            let production_ticks = ticks_since_last as f32 / tick_per_production;

            production.production += production_ticks;

            inventory.add(production_type, extract_int(&mut production.production));

            production.last_production_tick = world_data.tick;
        });
    
}

fn extract_int(amount: &mut f32) -> u32 {
    let int = amount.trunc() as u32;
    *amount -= int as f32;
    int
}
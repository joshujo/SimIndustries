use shipyard::{IntoIter, View, ViewMut};
use rayon::prelude::*;

use crate::core::{Factory, Inventory, Production, WorldData};

pub fn produce_goods(world_data: &mut WorldData) {
    let world = &mut world_data.world;

    world.run(|mut inventory: ViewMut<Inventory>, factory: View<Factory>, production: View<Production>| {

        // factory.iter().with_id()
        //     .for_each(|(id, factory)| {
        //         let production = production.get(id).unwrap();
        //         let tick_per_production = ((60.0 / world_data.time_scale as f32 * 3600.0) / production.rate_per_hour).round() as u64;
        //         let product = production.produces;

        //         if world_data.tick % tick_per_production == 0 {
        //             let goods = Goods(product, 1);
        //             match good.get_or_insert(id, goods) {
        //                 Some(mut good) => {
        //                     good.1 += 1;
        //                 },
        //                 None => ()
        //             }
        //         }
        //     });

        (&mut inventory, &factory, &production);
    });

    let (mut inventory, factory, production) = world.borrow::<(ViewMut<Inventory>, View<Factory>, View<Production>)>().unwrap();

    (&mut inventory, &factory, &production).par_iter()
        .for_each(|(inventory, _, production)| {
            let production_type = production.produces;

            let tick_per_production = ((60.0 / world_data.time_scale as f32 * 3600.0) / production.rate_per_hour).round() as u64;

            if world_data.tick % tick_per_production == 0 {
                inventory.add(production_type, 1);
            }
        });
    
}
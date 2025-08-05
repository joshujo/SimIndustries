use std::collections::HashMap;

use shipyard::{EntityId, World};

pub struct WorldData {
    pub world: World,
    pub assets: HashMap<u64, EntityId>,
    pub companies: HashMap<u64, EntityId>,
    pub tick: u64,
    pub time_scale: u32
}

impl WorldData {
    pub fn new(world: World, assets: HashMap<u64, EntityId>, companies: HashMap<u64, EntityId>) -> Self {

        #[cfg(not(debug_assertions))]
        let time_scale: u32 = 1;

        #[cfg(debug_assertions)]
        let time_scale: u32 = 6;

        Self {
            world,
            assets,
            companies,
            tick: 0,
            time_scale
        }
    }

    pub fn tick(&mut self) {
        self.tick += 1
    }
}
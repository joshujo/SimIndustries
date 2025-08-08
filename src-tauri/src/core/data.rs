use std::collections::HashMap;
use time::{self, Date, Duration, PrimitiveDateTime, Time};

use shipyard::{EntityId, World};

use crate::core::{AssetType, Currency, Good};

pub struct WorldData {
    pub world: World,
    pub assets: HashMap<u64, AssetData>,
    pub companies: HashMap<u64, EntityId>,
    pub tick: u64,
    pub time_scale: u32,
    pub time: PrimitiveDateTime,
    pub unowned_assets: HashMap<u64, EntityId>
}

impl WorldData {
    pub fn new(world: World, assets: HashMap<u64, AssetData>, companies: HashMap<u64, EntityId>) -> Self {

        #[cfg(not(debug_assertions))]
        let time_scale: u32 = 1;

        #[cfg(debug_assertions)]
        let time_scale: u32 = 6;

        let date = Date::from_calendar_date(2000, time::Month::January, 1).unwrap();
        let time = Time::from_hms(12, 0, 0).unwrap();
        let time = PrimitiveDateTime::new(date, time);

        Self {
            world,
            assets,
            companies,
            tick: 0,
            time_scale,
            time,
            unowned_assets: HashMap::new()
        }
    }

    pub fn tick(&mut self) {
        self.tick += 1;
        let tick = Duration::seconds_f32(( 1.0 / 60.0) * self.time_scale as f32);
        self.time = self.time.saturating_add(tick);
    }
}

pub struct AssetData {
    pub entity_id: EntityId,
    pub asset_type: AssetType,
    pub value: Currency,
    pub production: Option<Good>
}
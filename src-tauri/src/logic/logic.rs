use std::{num::NonZeroU32, thread::sleep, time::{Duration, Instant}};
use tauri::{AppHandle, Manager};

use crate::{interface::register::{Register, RegisterData}, logic::game::{generate_world, main} };

#[cfg(debug_assertions)]
static UPDATES_PER_SECOND: NonZeroU32 = NonZeroU32::new(10).unwrap();

#[cfg(not(debug_assertions))]
static UPDATES_PER_SECOND: NonZeroU32 = NonZeroU32::new(60).unwrap();

struct TickSystem {
    start: Instant,
    interval: Duration,
    tick_count: u32
}

impl TickSystem {
    fn new(interval: NonZeroU32) -> Self {
        let interval = Duration::from_secs_f32(1.0 / interval.get() as f32);
        Self {
            start: Instant::now(),
            interval,
            tick_count: 0,
        }
    }

    fn tick(&mut self) {
        self.tick_count += 1;
        

        let next_tick = self.start + self.interval * self.tick_count;
        let now = Instant::now();

        if now < next_tick {
            sleep(next_tick - now);
        }
    }
}

pub fn logic(app: AppHandle) {
    loop {
        let player_register = await_register(&app);
        let mut world = generate_world(player_register);
        let mut tick = TickSystem::new(UPDATES_PER_SECOND);
        loop {
            world.tick();
            main(&mut world, &app);
            tick.tick();
        }
    }
}

fn await_register(app: &AppHandle) -> RegisterData {
    let register_handle = app.app_handle();
    let state = register_handle.state::<Register>();

    let mut receiver = state.receiver.lock().unwrap();
    let data = receiver.blocking_recv().unwrap();

    data
}
#[cfg(debug_assertions)]
use std::num::NonZeroU32;

use chron::Clock;
use tauri::{AppHandle, Manager};

use crate::{logic::game::{generate_world, main}, new_game::{Register, RegisterData}};

#[cfg(debug_assertions)]
static UPDATES_PER_SECOND: NonZeroU32 = NonZeroU32::new(10).unwrap();

#[cfg(not(debug_assertions))]
static UPDATES_PER_SECOND: NonZeroU32 = NonZeroU32::new(60).unwrap();

pub fn logic(app: AppHandle) {
    loop {
        let player_register = await_register(&app);
        let mut world = generate_world(player_register);
        let clock = Clock::new(UPDATES_PER_SECOND);
        for _ in clock {
            main(&mut world, &app);
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
mod company;
mod economy;
mod assets;
mod currency;
mod data;
mod goods;

use std::sync::atomic::AtomicU64;

pub use company::*;
pub use assets::*;
pub use currency::Currency;
pub use data::*;
pub use goods::*;



pub static COMPANY_ID_GENERATOR: AtomicU64 = AtomicU64::new(1);
pub static ASSET_ID_GENERATOR: AtomicU64 = AtomicU64::new(0);
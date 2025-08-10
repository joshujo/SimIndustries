use std::{ borrow::Borrow, sync::atomic::AtomicU64 };

use shipyard::Component;

pub static COMPANY_ID_GENERATOR: AtomicU64 = AtomicU64::new(1);
pub static ASSET_ID_GENERATOR: AtomicU64 = AtomicU64::new(0);

#[derive(Hash, PartialEq, Eq, Clone, Copy, Component)]
pub struct CompanyId(u64);
#[derive(Hash, PartialEq, Eq, Clone, Copy, Component)]
pub struct AssetId(u64);

pub trait Identification {
    fn new() -> Self;
    fn value(&self) -> u64;
}

impl Identification for CompanyId {
    fn new() -> Self {
        Self(COMPANY_ID_GENERATOR.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
    }

    fn value(&self) -> u64 {
        self.0
    }
}

impl Identification for AssetId {
    fn new() -> Self {
        Self(ASSET_ID_GENERATOR.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
    }

    fn value(&self) -> u64 {
        self.0
    }
}

pub fn reset_identification() {
    COMPANY_ID_GENERATOR.store(1, std::sync::atomic::Ordering::Relaxed);
    ASSET_ID_GENERATOR.store(1, std::sync::atomic::Ordering::Relaxed);
}

impl Borrow<u64> for CompanyId {
    fn borrow(&self) -> &u64 {
        &self.0
    }
}

impl Borrow<u64> for AssetId {
    fn borrow(&self) -> &u64 {
        &self.0
    }
}

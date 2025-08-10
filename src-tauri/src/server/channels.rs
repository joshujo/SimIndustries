use crossbeam::channel::{ unbounded, Receiver, Sender };
use once_cell::sync::Lazy;
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct ResponseData {
    pub payload: serde_json::Value
}

pub struct PurchaseAssetRequest {
    pub id: u64
}

pub static PURCHASEASSETS_CHANNEL: Lazy<(Sender<PurchaseAssetRequest>, Receiver<PurchaseAssetRequest>)> = Lazy::new(|| {
    unbounded::<PurchaseAssetRequest>()
});

pub fn server() {
    let _ = &*PURCHASEASSETS_CHANNEL;
}


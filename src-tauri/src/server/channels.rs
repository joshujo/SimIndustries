use crossbeam::channel::{ unbounded, Receiver, Sender };
use once_cell::sync::Lazy;

pub struct ResponseData {
    pub payload: serde_json::Value
}

pub struct PurchaseFactoryRequest {
    pub id: u64
}

pub static PURCHASEFACTORY_CHANNEL: Lazy<(Sender<PurchaseFactoryRequest>, Receiver<PurchaseFactoryRequest>)> = Lazy::new(|| {
    unbounded::<PurchaseFactoryRequest>()
});

pub fn server() {
    let _ = &*PURCHASEFACTORY_CHANNEL;
}


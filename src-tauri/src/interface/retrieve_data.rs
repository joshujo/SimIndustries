use std::sync::Arc;

use arc_swap::ArcSwap;

#[derive(PartialEq, Clone, Copy)]
pub enum RetrieveType {
    PlayerAssets,
    Inventory(u64)
}

pub struct RetrieveData {
    pub data: ArcSwap<Vec<RetrieveType>>
}

impl RetrieveData {
    pub fn default() -> Self {
        RetrieveData { 
            data:  ArcSwap::from_pointee(Vec::new())
        }
    }

    fn add(&self, retrieve_type: RetrieveType) {
        let arc = self.data.load();

        let mut clone = (**arc).clone();
        clone.push(retrieve_type);

        self.data.store(Arc::new(clone));
    } 

    fn remove(&self, retrieve_type: RetrieveType) {
        let arc = self.data.load();

        let mut clone = (**arc).clone();
        clone.retain(|&x| x != retrieve_type);

        self.data.store(Arc::new(clone));
    }

    pub fn retrieve(&self) -> Vec<RetrieveType> {
        self.data.load_full().to_vec()
    }
}

#[tauri::command]
pub fn retrieve_player_assets(state: tauri::State<'_, RetrieveData>) {
    state.add(RetrieveType::PlayerAssets);
}

#[tauri::command]
pub fn unsub_retrieve_player_assets(state: tauri::State<'_, RetrieveData>) {
    state.remove(RetrieveType::PlayerAssets);
}

#[tauri::command]
pub fn retrieve_inventory_data(state: tauri::State<'_, RetrieveData>, id: u64) {
    state.add(RetrieveType::Inventory(id));
}

#[tauri::command]
pub fn unsub_retrieve_inventory_data(state: tauri::State<'_, RetrieveData>, id: u64) {
    state.remove(RetrieveType::Inventory(id));
}
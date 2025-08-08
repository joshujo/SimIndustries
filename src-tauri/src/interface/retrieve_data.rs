use parking_lot::Mutex;

#[derive(PartialEq, Clone, Copy)]
pub enum RetrieveType {
    PlayerAssets,
    Inventory(u64)
}

#[derive(Default)]
pub struct RetrieveData {
    pub data: Vec<RetrieveType>
}

#[tauri::command]
pub fn retrieve_player_assets(state: tauri::State<'_, Mutex<RetrieveData>>) {
    let mut state = state.lock();
    state.data.push(RetrieveType::PlayerAssets);
}

#[tauri::command]
pub fn unsub_retrieve_player_assets(state: tauri::State<'_, Mutex<RetrieveData>>) {
    let mut state = state.lock();
    state.data.retain(|&x| x != RetrieveType::PlayerAssets);
}

#[tauri::command]
pub fn retrieve_inventory_data(state: tauri::State<'_, Mutex<RetrieveData>>, id: u64) {
    let mut state = state.lock();
    state.data.push(RetrieveType::Inventory(id));
}

#[tauri::command]
pub fn unsub_retrieve_inventory_data(state: tauri::State<'_, Mutex<RetrieveData>>, id: u64) {
    let mut state = state.lock();
    state.data.retain(|&x| x != RetrieveType::Inventory(id));
}
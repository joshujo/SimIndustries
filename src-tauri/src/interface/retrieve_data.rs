use parking_lot::Mutex;

#[derive(PartialEq, Clone, Copy)]
pub enum RetrieveType {
    PlayerAssets
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
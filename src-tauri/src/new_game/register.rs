use serde::{self, Serialize};

#[derive(Serialize)]
pub struct RegisterResult {
    success: bool,
    message: Option<String>
}


#[tauri::command(rename_all = "snake_case")]
pub fn register(game_name: String, company_name: String) -> RegisterResult {
    if game_name.len() == 0 || company_name.len() == 0 {
        return RegisterResult {
            success: false,
            message: Some(String::from("Fill in everything"))
        }
    }

    RegisterResult { success: true, message: None }
}
use std::sync::Mutex;

use serde::{self, Serialize};
use tauri::async_runtime::{channel, Receiver, Sender};

#[derive(Serialize)]
pub struct RegisterResult {
    success: bool,
    message: Option<String>
}

pub struct RegisterData {
    pub game_name: String,
    pub company_name: String
}

pub struct Register{
    sender: Sender<RegisterData>,
    pub receiver: Mutex<Receiver<RegisterData>>
}

impl Register {
    pub fn new() -> Register {
        let (sender, receiver) = channel::<RegisterData>(128);
        let receiver = Mutex::new(receiver);
        Register { sender, receiver }
    }
}



#[tauri::command(rename_all = "snake_case")]
pub async fn register(state: tauri::State<'_, Register>, game_name: String, company_name: String) -> Result<RegisterResult, ()> {
    if game_name.len() == 0 || company_name.len() == 0 {
        return Ok(RegisterResult {
            success: false,
            message: Some(String::from("Fill in everything"))
        })
    }

    let register_data = RegisterData {
        game_name,
        company_name
    };

    state.sender.send(register_data).await.unwrap();

    Ok(RegisterResult { success: true, message: None })
}


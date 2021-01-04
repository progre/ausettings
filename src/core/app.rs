use super::{
    auprocess::{AUProcess, AUProcessError},
    storage::{GameSettingsListItem, Storage},
};

fn error_msg(err: AUProcessError) -> String {
    match err {
        AUProcessError::ProcessNotFound => "Error: Process not found.".into(),
        AUProcessError::ReqwestError(err) => format!("Error: ReqwestError {}", err),
        AUProcessError::Json5Error(err) => format!("Error: Json5Error {}", err),
        AUProcessError::OffsetsNotFound => "Error: OffsetsNotFound.".into(),
        AUProcessError::DllNotFound(_) => "Error: DllNotFound.".into(),
    }
}

#[derive(Clone, Debug)]
pub struct App {}

impl App {
    pub fn new() -> Self {
        Self {}
    }

    pub fn game_settings_list(&self) -> Vec<GameSettingsListItem> {
        Storage::load().game_settings_list
    }

    pub fn set_game_settings_name(&self, idx: usize, name: String) -> bool {
        let mut storage = Storage::load();
        storage.game_settings_list[idx].name = name;
        match storage.save() {
            Err(err) => {
                eprintln!("Error: file output failed. {}", err);
                return false;
            }
            Ok(_) => {}
        };
        true
    }

    pub fn save_memory_to_file(&self, idx: usize) -> bool {
        let game_settings = match AUProcess::new().game_settings() {
            Err(err) => {
                eprintln!("{}", error_msg(err));
                return false;
            }
            Ok(x) => x,
        };
        let mut storage = Storage::load();
        storage.game_settings_list[idx].game_settings = Some(game_settings);
        match storage.save() {
            Err(_err) => {
                eprintln!("Error: file output failed.");
                return false;
            }
            Ok(_) => {}
        };
        true
    }

    pub fn load_memory_from_file(&self, idx: usize) -> bool {
        let mut storage = Storage::load();
        let game_settings = match storage.game_settings_list.remove(idx).game_settings {
            None => {
                eprintln!("Error: No data.");
                return false;
            }
            Some(x) => x,
        };
        match AUProcess::new().set_game_settings(game_settings) {
            Err(err) => {
                eprintln!("{}", error_msg(err));
                return false;
            }
            Ok(_) => {}
        }
        true
    }
}

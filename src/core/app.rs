use super::{
    aucaptureoffsets::{AUCaptureOffsets, AUCaptureOffsetsError},
    auprocess::{AUProcess, AUProcessError},
    auprocessreadwrite::AUProcessReadWrite,
    storage::{GameSettingsListItem, Storage},
};

fn error_msg(err: AUProcessError) -> String {
    match err {
        AUProcessError::ProcessNotFound => "Error: Process not found.".into(),
        AUProcessError::DllNotFound(_) => "Error: DllNotFound.".into(),
    }
}

pub enum AppError {
    FetchFailed(reqwest::Error),
    ParseFailed(json5::Error),
    ProcessNotFound,
    DllNotFound(std::io::Error),
}

pub struct App {
    au_capture_offsets: Option<AUCaptureOffsets>,
    au_process: Option<AUProcess>,
}

impl App {
    pub fn new() -> Self {
        Self {
            au_capture_offsets: None,
            au_process: None,
        }
    }

    pub fn init(&mut self) -> Result<Vec<GameSettingsListItem>, AppError> {
        self.au_capture_offsets = Some(AUCaptureOffsets::fetch().map_err(|err| match err {
            AUCaptureOffsetsError::FetchFailed(err) => AppError::FetchFailed(err),
            AUCaptureOffsetsError::ParseFailed(err) => AppError::ParseFailed(err),
        })?);
        self.au_process = Some(AUProcess::new().map_err(|err| match err {
            AUProcessError::ProcessNotFound => AppError::ProcessNotFound,
            AUProcessError::DllNotFound(err) => AppError::DllNotFound(err),
        })?);
        Ok(Storage::load().game_settings_list)
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

    pub fn save_memory_to_file(&self, idx: usize) -> Option<()> {
        let au_capture_offsets = self.au_capture_offsets.as_ref()?;
        let au_process = self.au_process.as_ref()?;
        let game_settings =
            AUProcessReadWrite::new(au_capture_offsets, au_process)?.game_settings();
        let mut storage = Storage::load();
        storage.game_settings_list[idx].game_settings = Some(game_settings);
        match storage.save() {
            Err(_err) => {
                eprintln!("Error: file output failed.");
                return None;
            }
            Ok(_) => {}
        };
        Some(())
    }

    pub fn load_memory_from_file(&self, idx: usize) -> Option<()> {
        let au_capture_offsets = self.au_capture_offsets.as_ref()?;
        let au_process = self.au_process.as_ref()?;
        let mut storage = Storage::load();
        let game_settings = match storage.game_settings_list.remove(idx).game_settings {
            None => {
                eprintln!("Error: No data.");
                return None;
            }
            Some(x) => x,
        };
        AUProcessReadWrite::new(au_capture_offsets, au_process)?.set_game_settings(game_settings);
        Some(())
    }
}

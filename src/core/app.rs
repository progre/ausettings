use std::{sync::Arc, time::Duration};

use tokio::{spawn, sync::RwLock, task::JoinHandle, time::interval};

use super::{
    aucaptureoffsets::{AUCaptureOffsets, AUCaptureOffsetsError},
    auprocess::{AUProcess, AUProcessError},
    auprocessreadwrite::AUProcessReadWrite,
    storage::{GameSettingsListItem, Storage},
};

async fn fetch_offsets(au_capture_offsets_lock: &RwLock<Option<AUCaptureOffsets>>) {
    match AUCaptureOffsets::fetch() {
        Err(err) => {
            let msg = match err {
                AUCaptureOffsetsError::FetchFailed(err) => format!("Fetch failed ({})", err),
                AUCaptureOffsetsError::ParseFailed(err) => format!("Parse failed ({})", err),
            };
            eprintln!("Fetch failed: {}", msg);
        }
        Ok(au_capture_offsets) => {
            *(au_capture_offsets_lock.write().await) = Some(au_capture_offsets);
        }
    }
}

async fn capture_process(au_process_lock: &RwLock<Option<AUProcess>>) {
    let mut interval = interval(Duration::from_secs(1));
    loop {
        interval.tick().await;
        if let Some(au_process) = au_process_lock.read().await.as_ref() {
            if au_process.process().is_active() {
                continue;
            }
        }
        println!("Capturing au process...");
        match AUProcess::new() {
            Err(err) => {
                let msg = match err {
                    AUProcessError::ProcessNotFound => "Process not found".into(),
                    AUProcessError::DllNotFound(err) => format!("DLL not found({})", err),
                };
                eprintln!("Capture failed: {}", msg);
            }
            Ok(au_process) => {
                println!("Captured.");
                *(au_process_lock.write().await) = Some(au_process);
            }
        }
    }
}

pub struct App {
    _au_capture_offsets_task: JoinHandle<()>,
    _au_process_task: JoinHandle<()>,
    au_capture_offsets: Arc<RwLock<Option<AUCaptureOffsets>>>,
    au_process: Arc<RwLock<Option<AUProcess>>>,
}

impl App {
    pub fn new() -> Self {
        let au_capture_offsets = Arc::new(RwLock::new(None));
        let au_process = Arc::new(RwLock::new(None));
        Self {
            _au_capture_offsets_task: spawn({
                let au_capture_offsets = au_capture_offsets.clone();
                async move { fetch_offsets(&au_capture_offsets).await }
            }),
            _au_process_task: spawn({
                let au_process = au_process.clone();
                async move { capture_process(&au_process).await }
            }),
            au_capture_offsets,
            au_process,
        }
    }

    pub async fn game_settings_list(&mut self) -> Vec<GameSettingsListItem> {
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

    pub async fn save_memory_to_file(&self, idx: usize) -> Option<()> {
        let game_settings = {
            let au_capture_offsets_guard = self.au_capture_offsets.read().await;
            let au_capture_offsets = au_capture_offsets_guard.as_ref()?;
            let au_process_guard = self.au_process.read().await;
            let au_process = au_process_guard.as_ref()?;
            AUProcessReadWrite::new(au_capture_offsets, au_process)?.game_settings()
        };
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

    pub async fn load_memory_from_file(&self, idx: usize) -> Option<()> {
        let au_capture_offsets_guard = self.au_capture_offsets.read().await;
        let au_capture_offsets = au_capture_offsets_guard.as_ref()?;
        let au_process_guard = self.au_process.read().await;
        let au_process = au_process_guard.as_ref()?;
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

#[test]
fn test_send_sync() {
    fn assert_send<T: Send>() {}
    assert_send::<App>();

    fn assert_sync<T: Sync>() {}
    assert_sync::<App>();
}

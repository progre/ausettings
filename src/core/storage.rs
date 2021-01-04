use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use directories_next::ProjectDirs;
use serde::{Deserialize, Serialize};

use super::game_settings::GameSettings;

fn data_path() -> PathBuf {
    let mut path = if let Some(project_dirs) = ProjectDirs::from("net", "prgrssv", "ausettings") {
        project_dirs.data_dir().into()
    } else {
        std::env::current_dir().unwrap_or(std::path::PathBuf::new())
    };
    path.push("ausettings.json");
    path
}

#[derive(Serialize, Deserialize)]
pub struct GameSettingsListItem {
    pub name: String,
    pub game_settings: Option<GameSettings>,
}

#[derive(Serialize, Deserialize)]
pub struct Storage {
    pub game_settings_list: Vec<GameSettingsListItem>,
}

impl Default for Storage {
    fn default() -> Self {
        Self {
            game_settings_list: (0..10)
                .map(|i| GameSettingsListItem {
                    name: format!("Settings {}", i + 1),
                    game_settings: None,
                })
                .collect(),
        }
    }
}

impl Storage {
    pub fn load() -> Self {
        let data_str = fs::read_to_string(&data_path()).unwrap_or_default();
        serde_json::from_str(&data_str).unwrap_or_default()
    }

    pub fn save(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        let data_path = data_path();
        fs::create_dir_all(&data_path.parent().unwrap())?;
        Ok(fs::write(&data_path, &json)?)
    }
}

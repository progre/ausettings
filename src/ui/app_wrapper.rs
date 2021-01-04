use serde_json::Value;
use web_view::{WVResult, WebView};

use crate::core::app::App;

pub struct AppWrapper {
    app: App,
}

impl AppWrapper {
    pub fn new() -> Self {
        Self { app: App::new() }
    }

    pub fn invoke_handler<T>(&self, web_view: &mut WebView<T>, arg: &str) -> WVResult {
        println!("{}", arg);
        let json: Value = serde_json::from_str(arg).unwrap();
        match json["type"].as_str().unwrap() {
            "game_settings_list" => {
                let list = self.app.game_settings_list();
                let eval = format!(
                    "{}(null, {})",
                    json["callback"].as_str().unwrap(),
                    serde_json::to_string(&list).unwrap()
                );
                println!("{}", eval);
                web_view.eval(&eval)?;
            }
            "set_game_settings_name" => {
                let payload = &json["payload"];
                let idx = payload["index"].as_u64().unwrap() as usize;
                let name = payload["name"].as_str().unwrap();
                self.app.set_game_settings_name(idx, name.into());
                web_view.eval(&format!("{}(null)", json["callback"].as_str().unwrap()))?;
            }
            "save_memory_to_file" => {
                let payload = &json["payload"];
                let idx = payload["index"].as_u64().unwrap() as usize;
                self.app.save_memory_to_file(idx);
                web_view.eval(&format!("{}(null)", json["callback"].as_str().unwrap()))?;
            }
            "load_memory_from_file" => {
                let payload = &json["payload"];
                let idx = payload["index"].as_u64().unwrap() as usize;
                self.app.load_memory_from_file(idx);
                web_view.eval(&format!("{}(null)", json["callback"].as_str().unwrap()))?;
            }
            _ => unreachable!(),
        }
        Ok(())
    }
}

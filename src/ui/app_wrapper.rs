use serde_json::{json, Value};
use web_view::{WVResult, WebView};

use crate::core::app::{App, AppError};

pub struct AppWrapper {
    app: App,
}

impl AppWrapper {
    pub fn new() -> Self {
        Self { app: App::new() }
    }

    pub fn invoke_handler<T>(&mut self, web_view: &mut WebView<T>, arg: &str) -> WVResult {
        println!("--> {}", arg);
        let json: Value = serde_json::from_str(arg).unwrap();
        match json["type"].as_str().unwrap() {
            "init" => {
                let eval = match self.app.init() {
                    Ok(list) => {
                        format!(
                            "{}(null, {})",
                            json["callback"].as_str().unwrap(),
                            serde_json::to_string(&list).unwrap()
                        )
                    }
                    Err(err) => {
                        let name = match err {
                            AppError::FetchFailed(_) => "FetchError",
                            AppError::ParseFailed(_) => "ParseError",
                            AppError::ProcessNotFound => "ProcessNotFoundError",
                            AppError::DllNotFound(_) => "DllNotFoundError",
                        };
                        format!(
                            "{}({})",
                            json["callback"].as_str().unwrap(),
                            json!({ "name": name }).to_string()
                        )
                    }
                };
                println!("<-- {}", eval);
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

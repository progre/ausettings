use std::sync::Arc;

use serde_json::{json, Value};
use tokio::{spawn, sync::Mutex};
use web_view::{Handle, WVResult, WebView};

use crate::core::app::App;

async fn invoke_handler<T>(handle: Handle<T>, arg: &str, app_mutex: &Mutex<App>) {
    let json: Value = serde_json::from_str(arg).unwrap();
    let mut app = app_mutex.lock().await;
    let (err, result) = match json["type"].as_str().unwrap() {
        "game_settings_list" => (
            "null".to_owned(),
            serde_json::to_string(&app.game_settings_list().await).unwrap(),
        ),
        "set_game_settings_name" => {
            let payload = &json["payload"];
            let idx = payload["index"].as_u64().unwrap() as usize;
            let name = payload["name"].as_str().unwrap();
            app.set_game_settings_name(idx, name.into());
            ("null".into(), "null".into())
        }
        "save_memory_to_file" => {
            let payload = &json["payload"];
            let idx = payload["index"].as_u64().unwrap() as usize;
            match app.save_memory_to_file(idx).await {
                Some(_) => ("null".into(), "null".into()),
                None => (json!({ "name": "Error" }).to_string(), "null".into()),
            }
        }
        "load_memory_from_file" => {
            let payload = &json["payload"];
            let idx = payload["index"].as_u64().unwrap() as usize;
            match app.load_memory_from_file(idx).await {
                Some(_) => ("null".into(), "null".into()),
                None => (json!({ "name": "Error" }).to_string(), "null".into()),
            }
        }
        _ => unreachable!(),
    };
    let eval = format!(
        "{}({}, {})",
        json["callback"].as_str().unwrap(),
        err,
        result
    );
    println!("<-- {}", eval);
    handle
        .dispatch(move |web_view| web_view.eval(&eval))
        .unwrap_or_else(|err| eprintln!("{}", err));
}

pub struct AppWrapper {
    app: Arc<Mutex<App>>,
}

impl AppWrapper {
    pub fn new() -> Self {
        Self {
            app: Arc::new(Mutex::new(App::new())),
        }
    }

    pub fn invoke_handler(&mut self, web_view: &mut WebView<()>, arg: &str) -> WVResult {
        let handle = web_view.handle();
        let arg = arg.to_owned();
        let app = self.app.clone();
        spawn(async move {
            println!("--> {}", arg);
            invoke_handler(handle, &arg, &app).await;
        });
        Ok(())
    }
}

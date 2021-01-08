use std::sync::Arc;

use serde_json::{json, Value};
use tokio::{
    spawn,
    sync::{mpsc, Mutex},
};
use web_view::{Handle, WVResult, WebView};

use crate::core::app::{App, ProcessStatus};

async fn invoke_handler<T>(handle: Handle<T>, arg: &str, app_mutex: &Mutex<App>) {
    let json: Value = serde_json::from_str(arg).unwrap();
    let app = app_mutex.lock().await;
    let payload = &json["payload"];
    let (err, result) = match json["type"].as_str().unwrap() {
        "init" => (
            "null".to_owned(),
            serde_json::to_string(&app.init().await).unwrap(),
        ),
        "open_browser" => {
            app.open_browser(payload["url"].as_str().unwrap());
            ("null".to_owned(), "null".to_owned())
        }
        "set_game_settings_name" => {
            let idx = payload["index"].as_u64().unwrap() as usize;
            let name = payload["name"].as_str().unwrap();
            app.set_game_settings_name(idx, name.into());
            ("null".into(), "null".into())
        }
        "save_memory_to_file" => {
            let idx = payload["index"].as_u64().unwrap() as usize;
            match app.save_memory_to_file(idx).await {
                Some(_) => ("null".into(), "null".into()),
                None => (json!({ "name": "Error" }).to_string(), "null".into()),
            }
        }
        "load_memory_from_file" => {
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
    pub fn new(handle: Handle<()>) -> Self {
        let (process_status_sender, mut rx) = mpsc::channel::<ProcessStatus>(16);
        spawn(async move {
            loop {
                let process_status = rx.recv().await.unwrap();
                let eval = format!(
                    "window.onChangeProcessStatus({})",
                    serde_json::to_string(&process_status).unwrap()
                );
                println!("<-- {}", eval);
                handle
                    .dispatch(move |web_view| web_view.eval(&eval))
                    .unwrap_or_else(|err| eprintln!("{}", err));
            }
        });
        Self {
            app: Arc::new(Mutex::new(App::new(process_status_sender))),
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

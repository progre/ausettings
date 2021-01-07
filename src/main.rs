#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod core;
mod ui;

use std::sync::{Arc, Mutex};

use web_view::*;

use crate::ui::app_wrapper::AppWrapper;

#[tokio::main]
async fn main() {
    let app_wrapper: Arc<Mutex<Option<AppWrapper>>> = Arc::new(Mutex::new(None));
    let web_view = web_view::builder()
        .title("ausettings")
        .content(Content::Html(include_str!("../dist/index.html")))
        .size(416, 720)
        .resizable(true)
        .debug(cfg!(debug_assertions))
        .user_data(())
        .invoke_handler({
            let app_wrapper = app_wrapper.clone();
            move |web_view, arg| {
                let mut guard = app_wrapper.lock().unwrap();
                guard.as_mut().unwrap().invoke_handler(web_view, arg)
            }
        })
        .build()
        .unwrap();
    *(app_wrapper.lock().as_deref_mut().unwrap()) = Some(AppWrapper::new(web_view.handle()));
    web_view.run().unwrap();
}

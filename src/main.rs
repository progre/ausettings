#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod core;
mod ui;

use web_view::*;

use crate::ui::app_wrapper::AppWrapper;

#[tokio::main]
async fn main() {
    let mut app_wrapper = AppWrapper::new();
    web_view::builder()
        .title("ausettings")
        .content(Content::Html(include_str!("../dist/index.html")))
        .size(370, 640)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|web_view, arg| app_wrapper.invoke_handler(web_view, arg))
        .run()
        .unwrap();
}

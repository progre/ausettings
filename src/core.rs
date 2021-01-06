pub mod app;
mod aucaptureoffsets;
mod auprocess;
mod auprocessreadwrite;
mod game_settings;
mod process;
#[cfg(windows)]
mod process_impl;
#[cfg(not(windows))]
mod process_mock;
mod storage;

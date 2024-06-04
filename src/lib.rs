use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

use std::{collections::HashMap, sync::Mutex};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Settings;
#[cfg(mobile)]
use mobile::Settings;

#[derive(Default)]
struct MyState(Mutex<HashMap<String, String>>);

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the settings APIs.
pub trait SettingsExt<R: Runtime> {
  fn settings(&self) -> &Settings<R>;
}

impl<R: Runtime, T: Manager<R>> crate::SettingsExt<R> for T {
  fn settings(&self) -> &Settings<R> {
    self.state::<Settings<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("settings")
    .invoke_handler(tauri::generate_handler![commands::execute])
    .setup(|app, api| {
      #[cfg(mobile)]
      let settings = mobile::init(app, api)?;
      #[cfg(desktop)]
      let settings = desktop::init(app, api)?;
      app.manage(settings);

      // manage state so it is accessible by the commands
      app.manage(MyState::default());
      Ok(())
    })
    .build()
}

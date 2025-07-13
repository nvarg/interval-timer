#![windows_subsystem = "windows"]

use eframe;

mod app;
mod internal;
mod ui;

fn main() -> Result<(), String> {
    let config_file = internal::dirs::default_config_path()?;
    let _ = internal::dirs::create_dirs_if_not_exists();
    let app_options = internal::settings_parser::AppSettings::new_from_file(&config_file)?;

    let app = app::App::new(app_options);
    let options = eframe::NativeOptions::default();
    eframe::run_native("Interval Timer", options, Box::new(|_cc| Box::new(app)))
        .map_err(|v| v.to_string())
}

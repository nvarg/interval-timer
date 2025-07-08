use eframe::egui::{self, Color32};
use std::env;

use crate::internal::clock::Clock;
use crate::internal::dirs::default_config_path;
use crate::internal::settings_parser::AppConfig;
use crate::internal::sound;
use crate::ui::countdown::{CountdownElement, CountdownSignal};
use crate::ui::settings::SettingsElement;

pub struct App {
    countdown_element: CountdownElement,
    settings_element: SettingsElement,
    tick_interval: u64,
    vol: f32,
    show_settings: bool,
}

impl App {
    pub fn new(options: AppConfig) -> Self {
        let countdown_element = CountdownElement::new(
            &options
                .timers
                .iter()
                .map(|v| (v.0, Color32::from_rgb(v.1.0, v.1.1, v.1.2)))
                .collect(),
            options.play_once,
        );

        let tick_interval = options.tick_interval;

        let vol = options.vol;
        let settings_element = SettingsElement::new(options);

        Self {
            countdown_element,
            settings_element,
            tick_interval: tick_interval,
            vol: vol,
            show_settings: false,
        }
    }

    fn toggle_settings(&mut self) -> () {
        self.show_settings = !self.show_settings;
    }
    fn update_options(&mut self, config: &AppConfig) {
        self.countdown_element = CountdownElement::new(
            &config
                .timers
                .iter()
                .map(|v| (v.0, Color32::from_rgb(v.1.0, v.1.1, v.1.2)))
                .collect(),
            config.play_once,
        );

        self.tick_interval = config.tick_interval;

        self.vol = config.vol;

        let path = default_config_path().unwrap_or(
            // TODO: Handle this error better
            env::temp_dir()
                .join("config.txt")
                .to_string_lossy()
                .to_string(),
        );

        // TODO: Handle error
        let _ = config.write_to_file(&path);

        self.toggle_settings();
    }

    pub fn handle_countdown_signal(&self, signal: CountdownSignal) {
        match signal {
            CountdownSignal::Finished => sound::play_sound(self.vol),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let repaint_clock = Clock::new(self.tick_interval);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.spacing_mut().item_spacing.y = 8.0;
            ui.heading("Interval Timer");

            if self.show_settings {
                if let Some(opts) = self.settings_element.draw(ui) {
                    self.update_options(&opts);
                }
            } else {
                if ui.button("Settings").clicked() {
                    self.toggle_settings();
                }

                if let Some(signal) = self.countdown_element.draw(ui) {
                    self.handle_countdown_signal(signal);
                }
            }
        });

        ctx.request_repaint_after(repaint_clock.remaining());
    }
}

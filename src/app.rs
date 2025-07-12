use eframe::egui::{self, Color32};
use std::env;
use std::path::Path;

use crate::internal::clock::Clock;
use crate::internal::dirs::default_config_path;
use crate::internal::settings_parser::AppSettings;
use crate::internal::sound::{self, SoundFile};
use crate::ui::countdown::{CountdownElement, CountdownSignal};
use crate::ui::settings::{self, SettingsSignal};

pub struct App {
    countdown_element: CountdownElement,
    settings: AppSettings,
    show_settings: bool,
    custom_sound: sound::SoundFile,
}

const DEFAULT_TIMER_DURATION: u64 = 5000;
const DEFAULT_TIMER_COLOR: (u8, u8, u8) = (
    Color32::RED.to_array()[0],
    Color32::RED.to_array()[1],
    Color32::RED.to_array()[2],
);

impl App {
    pub fn new(settings: AppSettings) -> Self {
        let countdown_element = CountdownElement::new(
            &settings
                .timers
                .iter()
                .map(|v| (v.0, Color32::from_rgb(v.1.0, v.1.1, v.1.2)))
                .collect(),
            settings.play_once,
        );

        let mut custom_sound = SoundFile::new();
        if settings.use_custom_sound {
            custom_sound
                .load_file(&settings.custom_sound_location)
                .unwrap_or_else(|_| {
                    // TODO: Handle error with notification. For now:
                    // let's handle doing nothing
                });
        }

        Self {
            countdown_element,
            settings,
            show_settings: false,
            custom_sound,
        }
    }

    fn toggle_settings(&mut self) -> () {
        self.show_settings = !self.show_settings;
    }

    fn change_countdown_element(&mut self) {
        self.countdown_element = CountdownElement::new(
            &self
                .settings
                .timers
                .iter()
                .map(|v| (v.0, Color32::from_rgb(v.1.0, v.1.1, v.1.2)))
                .collect(),
            self.settings.play_once,
        );
    }

    fn save_settings(&self) {
        let path = default_config_path().unwrap_or(
            // TODO: Handle this error better
            env::temp_dir()
                .join("config.txt")
                .to_string_lossy()
                .to_string(),
        );

        // TODO: Handle error
        let _ = self.settings.write_to_file(&path);
    }

    fn load_sound(&mut self) {
        if Path::new(&self.settings.custom_sound_location).exists() {
            self.custom_sound
                .load_file(&self.settings.custom_sound_location)
                .unwrap_or_else(|_| {});
        }
    }

    pub fn handle_settings_signal(&mut self, signal: SettingsSignal) -> Result<(), String> {
        match signal {
            SettingsSignal::UpdateTimer((index, timer)) => {
                self.settings.timers[index] = timer;
                self.change_countdown_element();
            }
            SettingsSignal::AddTimer => {
                self.settings
                    .timers
                    .push((DEFAULT_TIMER_DURATION, DEFAULT_TIMER_COLOR));
                self.change_countdown_element();
            }
            SettingsSignal::UpdatePlayOnce(play_once) => {
                self.settings.play_once = play_once;
                self.change_countdown_element();
            }
            SettingsSignal::UpdateVolume(volume) => {
                self.settings.volume = volume;
            }
            SettingsSignal::UpdateUseCustomSound(use_custom_sound) => {
                self.settings.use_custom_sound = use_custom_sound;
            }
            SettingsSignal::UpdateCustomSoundLocation(location) => {
                self.settings.custom_sound_location = location;
                self.load_sound();
            }
            SettingsSignal::SaveSettings => {
                self.save_settings();
                self.show_settings = false;
            }
        };
        Ok(())
    }

    pub fn handle_countdown_signal(&self, signal: CountdownSignal) {
        match signal {
            CountdownSignal::Finished => {
                if self.settings.use_custom_sound && self.custom_sound.is_ready() {
                    self.custom_sound.play(self.settings.volume);
                } else {
                    sound::play_sound(self.settings.volume);
                }
            }
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let repaint_clock = Clock::new(self.settings.tick_interval);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.spacing_mut().item_spacing.y = 8.0;
                ui.horizontal(|ui| {
                    ui.heading("Interval Timer");

                    if ui.button("Settings").clicked() {
                        self.toggle_settings();
                    }
                });
                ui.add_space(16.0);

                if self.show_settings {
                    if let Some(signal) = settings::draw(
                        ui,
                        &self.settings.timers,
                        self.settings.play_once,
                        self.settings.volume,
                        self.settings.use_custom_sound,
                        &self.settings.custom_sound_location,
                    ) {
                        self.handle_settings_signal(signal).unwrap_or_else(|_| {});
                    }
                } else {
                    if let Some(signal) = self.countdown_element.draw(ui) {
                        self.handle_countdown_signal(signal);
                    }
                }
            });
        });

        ctx.request_repaint_after(repaint_clock.remaining());
    }
}

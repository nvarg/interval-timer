use crate::internal::settings_parser::AppConfig;
use eframe::egui;

pub struct SettingsElement {
    app_config: AppConfig,
}

impl SettingsElement {
    pub fn new(app_config: AppConfig) -> Self {
        Self { app_config }
    }

    pub fn draw(&mut self, ui: &mut egui::Ui) -> Option<AppConfig> {
        ui.group(|ui| {
            ui.heading("Timers");
            for mut timer in &mut self.app_config.timers {
                draw_edit_timer(ui, &mut timer);
            }
            if ui.button("Add").clicked() {
                self.app_config.timers.push((5000, (255, 95, 128)));
            }
        });

        ui.add(egui::Slider::new(&mut self.app_config.vol, 0.0..=1.0).text("Volume"));

        if ui.button("Confirm").clicked() {
            Some(self.app_config.clone())
        } else {
            None
        }
    }
}

fn draw_edit_timer(ui: &mut egui::Ui, timer: &mut (u64, (u8, u8, u8))) -> () {
    let mut color = timer.1.into();
    let mut time: String = timer.0.to_string();

    ui.horizontal(|ui| {
        let response = ui.color_edit_button_srgb(&mut color);
        if response.changed() {
            timer.1 = color.into();
        }

        let response = ui.text_edit_singleline(&mut time);
        if response.changed() {
            timer.0 = time.parse::<u64>().unwrap_or(timer.0);
        }
    });
}

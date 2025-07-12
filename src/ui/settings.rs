use eframe::egui;

pub enum SettingsSignal {
    UpdateTimer((usize, (u64, (u8, u8, u8)))),
    AddTimer,
    UpdatePlayOnce(bool),
    UpdateVolume(f32),
    UpdateUseCustomSound(bool),
    UpdateCustomSoundLocation(String),
    SaveSettings,
}

pub fn draw(
    ui: &mut egui::Ui,
    timers: &Vec<(u64, (u8, u8, u8))>,
    play_once: bool,
    volume: f32,
    use_custom_sound: bool,
    custom_sound_location: &String,
) -> Option<SettingsSignal> {
    let mut update = None;

    ui.vertical_centered_justified(|ui| {
        ui.group(|ui| {
            ui.heading("Timers");
            for (i, timer) in timers.iter().enumerate() {
                if let Some(change) = draw_edit_timer(ui, *timer) {
                    update = Some(SettingsSignal::UpdateTimer((i, change)));
                }
            }

            ui.vertical(|ui| {
                if ui.button("Add").clicked() {
                    update = Some(SettingsSignal::AddTimer)
                }

                let mut play_once = play_once;
                if ui.toggle_value(&mut play_once, "Play once?").changed() {
                    update = Some(SettingsSignal::UpdatePlayOnce(play_once))
                }
            });
        });

        ui.group(|ui| {
            ui.heading("Audio");
            let mut volume = volume;
            let slider = egui::Slider::new(&mut volume, 0.0..=1.0).text("Volume");
            if ui.add(slider).changed() {
                update = Some(SettingsSignal::UpdateVolume(volume));
            }

            ui.vertical(|ui| {
                let mut use_custom_sound = use_custom_sound;
                if ui
                    .checkbox(&mut use_custom_sound, "Use custom sound")
                    .changed()
                {
                    update = Some(SettingsSignal::UpdateUseCustomSound(use_custom_sound));
                }

                if use_custom_sound {
                    let mut custom_sound_location = custom_sound_location.clone();
                    ui.horizontal(|ui| {
                        ui.label("Custom sound location");
                        if ui
                            .text_edit_singleline(&mut custom_sound_location)
                            .changed()
                        {
                            update = Some(SettingsSignal::UpdateCustomSoundLocation(
                                custom_sound_location,
                            ));
                        }
                    });
                }
            });
        });

        ui.add_space(16.0);
        let save_button = egui::Button::new(egui::RichText::new("Save").size(20.0));
        if ui.add(save_button).clicked() {
            update = Some(SettingsSignal::SaveSettings)
        }
    });

    update
}

fn draw_edit_timer(ui: &mut egui::Ui, timer: (u64, (u8, u8, u8))) -> Option<(u64, (u8, u8, u8))> {
    let mut color = timer.1.into();
    let (mut hrs, mut mins, mut secs, mut ms) = millis_to_time(timer.0);
    let mut changed = false;

    ui.horizontal(|ui| {
        if ui.color_edit_button_srgb(&mut color).changed() {
            changed = true;
        }

        let prefix = zpad_prefix(hrs, 2);
        let hrs_input = egui::DragValue::new(&mut hrs).suffix(" h").prefix(prefix);

        let prefix = zpad_prefix(mins, 2);
        let mins_input = egui::DragValue::new(&mut mins)
            .clamp_range(0..=59)
            .suffix(" m")
            .prefix(prefix);

        let prefix = zpad_prefix(secs, 2);
        let secs_input = egui::DragValue::new(&mut secs)
            .clamp_range(0..=59)
            .suffix(" s")
            .prefix(prefix);

        let prefix = zpad_prefix(ms, 3);
        let ms_input = egui::DragValue::new(&mut ms)
            .clamp_range(0..=999)
            .suffix(" ms")
            .prefix(prefix);

        if ui.add(hrs_input).changed() {
            changed = true;
        }
        if ui.add(mins_input).changed() {
            changed = true;
        }
        if ui.add(secs_input).changed() {
            changed = true;
        }
        if ui.add(ms_input).changed() {
            changed = true;
        }
    });

    if changed {
        let new_time = time_to_millis(hrs, mins, secs, ms);
        Some((new_time, color.into()))
    } else {
        None
    }
}

fn time_to_millis(h: u64, m: u64, s: u64, ms: u64) -> u64 {
    ((h * 3600 + m * 60 + s) * 1000) + ms
}

fn millis_to_time(ms_total: u64) -> (u64, u64, u64, u64) {
    let total_secs = ms_total / 1000;
    let h = total_secs / 3600;
    let m = (total_secs / 60) % 60;
    let s = total_secs % 60;
    let ms = ms_total % 1000;
    (h, m, s, ms)
}

fn zpad_prefix(num: u64, width: usize) -> String {
    let num_str = num.to_string();
    let pad_len = width.saturating_sub(num_str.len());
    "0".repeat(pad_len)
}

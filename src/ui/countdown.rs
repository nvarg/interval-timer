use crate::ui::circular_progress_bar;
use eframe::egui;
use egui::Color32;

use crate::internal::clock::{Clock, State};
use crate::internal::queue::Queue;

pub struct CountdownElement {
    timers: Queue<(Clock, Color32)>,
    play_once: bool,
}

#[derive(Debug)]
pub enum CountdownSignal {
    // Types of events that can be returned
    Finished,
}

impl CountdownElement {
    pub fn new(timers: &Vec<(u64, Color32)>, play_once: bool) -> Self {
        let mut queue = Queue::new();
        queue.set(timers.iter().map(|v| (Clock::new(v.0), v.1)).collect());

        Self {
            timers: queue,
            play_once: play_once,
        }
    }

    fn get_clock(&self) -> Option<&Clock> {
        let timer = self.timers.get();

        if timer.is_none() {
            return None;
        }

        Some(&timer.unwrap().0)
    }

    fn get_clock_mut(&mut self) -> Option<&mut Clock> {
        let timer = self.timers.get_mut();

        if timer.is_none() {
            return None;
        }

        Some(&mut timer.unwrap().0)
    }

    fn get_color(&self) -> Color32 {
        let timer = self.timers.get();

        if timer.is_none() {
            return Color32::from_rgb(255, 255, 0);
        }

        timer.unwrap().1
    }

    fn get_state(&self) -> State {
        let timer = self.timers.get();

        if timer.is_none() {
            return State::Stopped;
        }

        timer.unwrap().0.get_state()
    }

    fn is_running(&self) -> bool {
        self.get_state() == State::Running
    }

    fn is_finished(&self) -> bool {
        self.get_state() == State::Finished
    }

    fn prev(&mut self) {
        self.timers.prev();
        if let Some(clock) = self.get_clock_mut() {
            clock.reset();
        }
    }

    fn next(&mut self) {
        let play_once_pause_condition = self.play_once && self.timers.is_last();
        self.timers.next();
        if let Some(clock) = self.get_clock_mut() {
            clock.reset();
            if play_once_pause_condition {
                clock.stop();
            }
        }
    }

    fn draw_progress(&self, ui: &mut egui::Ui, clock: &Clock, color: Color32) {
        let frac = clock.fraction();
        let timestamp = &clock.to_string();

        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            circular_progress_bar::draw(ui, 500.0, 500.0, color, frac, Some(timestamp));
        });
    }

    fn draw_placeholder(&self, ui: &mut egui::Ui, color: Color32) {
        circular_progress_bar::draw(ui, 500.0, 500.0, color, 0.0, Some("No Timers Set"));
    }

    fn draw_buttons(&mut self, ui: &mut egui::Ui) {
        let layout = egui::Layout::centered_and_justified(egui::Direction::LeftToRight);

        ui.allocate_ui_with_layout([240., 40.].into(), layout, |ui| {
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    let button = egui::Button::new("<").min_size([60., 40.].into());
                    if ui.add_sized([60., 40.], button).clicked() {
                        self.prev();
                        self.get_clock_mut().unwrap().stop();
                    }

                    if self.is_running() {
                        let button = egui::Button::new("Stop").min_size([120., 40.].into());
                        if ui.add(button).clicked() {
                            self.get_clock_mut().unwrap().stop();
                        }
                    } else {
                        let button = egui::Button::new("Start").min_size([120., 40.].into());
                        if ui.add(button).clicked() {
                            self.get_clock_mut().unwrap().start();
                        }
                    };

                    let button = egui::Button::new(">").min_size([60., 40.].into());
                    if ui.add_sized([60., 40.], button).clicked() {
                        self.next();
                        self.get_clock_mut().unwrap().stop();
                    }
                });
            });
        });
    }

    pub fn draw(&mut self, ui: &mut egui::Ui) -> Option<CountdownSignal> {
        // Checking before rendering, prevents
        // displayed state from flickering
        let signal = if self.is_finished() {
            self.next();
            Some(CountdownSignal::Finished)
        } else {
            None
        };

        ui.vertical_centered(|ui| {
            if let Some(clock) = self.get_clock() {
                self.draw_progress(ui, clock, self.get_color());
                ui.add_space(16.);
                self.draw_buttons(ui);
            } else {
                self.draw_placeholder(ui, self.get_color());
            }
        });

        signal
    }
}

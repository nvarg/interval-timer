use eframe::egui::{self, RichText, Vec2};
use egui::Color32;

use crate::internal::clock::{Clock, State};
use crate::internal::queue::Queue;
use crate::ui::circular_progress_bar;

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
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            let button_size = Vec2::new(120.0, 40.0);
            if self.is_running() {
                let stop_button =
                    egui::Button::new(RichText::new("Stop").size(20.0)).min_size(button_size);

                if ui.add(stop_button).clicked() {
                    self.get_clock_mut().unwrap().stop();
                }
            } else {
                let start_button =
                    egui::Button::new(RichText::new("Start").size(20.0)).min_size(button_size);

                if ui.add(start_button).clicked() {
                    self.get_clock_mut().unwrap().start();
                }
            }
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

        if let Some(clock) = self.get_clock() {
            self.draw_progress(ui, clock, self.get_color());
            self.draw_buttons(ui);
        } else {
            self.draw_placeholder(ui, self.get_color());
        }

        signal
    }
}

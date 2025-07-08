use std::fmt;
use std::time::{Duration, Instant};

#[derive(PartialEq, Eq)]
pub enum State {
    Running,
    Stopped,
    Finished,
}

#[derive(Debug)]
pub struct Clock {
    start: Instant,
    duration: Duration,
    running: bool,
}

impl Clock {
    pub fn new(millis: u64) -> Self {
        Self {
            start: Instant::now(),
            duration: Duration::from_millis(millis),
            running: true,
        }
    }

    pub fn elapsed(&self) -> Duration {
        if !self.running {
            return Duration::ZERO;
        }

        Instant::now() - self.start
    }

    pub fn fraction(&self) -> f32 {
        match self.duration {
            Duration::ZERO => 1.0,
            _ => self.elapsed().div_duration_f32(self.duration),
        }
        .clamp(0., 1.)
    }

    pub fn remaining(&self) -> Duration {
        if !self.running {
            return self.duration;
        }

        let elapsed = self.elapsed();

        match elapsed >= self.duration {
            true => Duration::from_secs(0),
            false => self.duration - elapsed,
        }
    }

    pub fn reset(&mut self) {
        self.start = Instant::now();
    }

    pub fn start(&mut self) {
        self.running = true;
        self.reset();
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn get_state(&self) -> State {
        match self.running {
            false => State::Stopped,
            true => {
                if self.elapsed() >= self.duration {
                    State::Finished
                } else {
                    State::Running
                }
            }
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let secs = self.remaining().as_secs();
        let seconds = self.remaining().as_secs_f32() % 60.0;
        let minutes = (secs / 60) % 60;
        let hours = (secs / 60) / 60;
        write!(f, "{}:{:0>2}:{}", hours, minutes, format_secs(seconds))
    }
}

fn format_secs(val: f32) -> String {
    let whole = val.trunc() as u32;
    let frac = ((val.fract() * 100.0).round()) as u32;
    format!("{:02}.{:02}", whole, frac)
}

use std::fs;
use std::io;

use crate::internal::keys_and_values::{KeysAndValues, ParseError};

#[derive(Default, Clone)]
pub struct AppConfig {
    pub timers: Vec<(u64, (u8, u8, u8))>,
    pub tick_interval: u64,
    pub play_once: bool,
    pub vol: f32,
}

static DEFAULT_TICK_INTERVAL: u64 = 50;
static DEFAULT_PLAY_ONCE: bool = false;
static DEFAULT_VOL: f32 = 0.5;

impl AppConfig {
    pub fn new_from_file(path: &str) -> Result<Self, String> {
        let file_content = match fs::read_to_string(path) {
            Ok(value) => value,
            Err(e) if e.kind() == io::ErrorKind::NotFound => String::new(),
            Err(e) => return Err(format!("Unhandled io error: {}", e)),
        };
        AppConfig::new_from_str(file_content)
    }

    pub fn new_from_str(data: String) -> Result<Self, String> {
        let kv = KeysAndValues::new_from_str(&data).map_err(|e| e.to_string())?;

        let timers = match kv.get("timers", |v| parse_timers(v)) {
            Ok(value) => value,
            Err(ParseError::MissingKey(_)) => vec![],
            Err(e) => return Err(e.to_string()),
        };

        let tick_interval = match kv.get("tick_interval", |v| v.parse::<u64>()) {
            Ok(value) => value,
            Err(ParseError::MissingKey(_)) => DEFAULT_TICK_INTERVAL,
            Err(e) => return Err(e.to_string()),
        };

        let play_once = match kv.get("play_once", |v| v.parse::<bool>()) {
            Ok(value) => value,
            Err(ParseError::MissingKey(_)) => DEFAULT_PLAY_ONCE,
            Err(e) => return Err(e.to_string()),
        };

        let vol = match kv.get("vol", |v| v.parse::<f32>()) {
            Ok(value) => value,
            Err(ParseError::MissingKey(_)) => DEFAULT_VOL,
            Err(e) => return Err(e.to_string()),
        };

        Ok(Self {
            timers: timers,
            tick_interval: tick_interval,
            play_once: play_once,
            vol: vol,
        })
    }

    pub fn write_to_file(&self, path: &str) -> Result<(), String> {
        let kv = self.to_kv()?;
        kv.write_to_file(path).map_err(|v| v.to_string())
    }

    fn to_kv(&self) -> Result<KeysAndValues, String> {
        let mut kv = KeysAndValues::new();

        kv.set("timers", &self.timers, |v| timers_to_string(v));
        kv.set("tick_interval", &self.tick_interval, |v| v.to_string());
        kv.set("play_once", &self.play_once, |v| v.to_string());

        Ok(kv)
    }
}

fn parse_timers(line: &str) -> Result<Vec<(u64, (u8, u8, u8))>, String> {
    line.split(",")
        .map(|entry| {
            let (millis, color) = entry
                .split_once("#")
                .ok_or_else(|| format!("Missing '#' in timer color '{}'", entry))?;

            let millis: u64 = millis
                .parse()
                .map_err(|e| format!("Invalid time provided in timer '{}': {}", millis, e))?;

            let color = parse_color(color)?;
            Ok((millis, color))
        })
        .collect()
}

fn parse_color(hex: &str) -> Result<(u8, u8, u8), String> {
    if hex.len() != 6 {
        return Err(format!("Colors should be 6 hex characters"));
    }

    let r = u8::from_str_radix(&hex[0..2], 16)
        .map_err(|v| format!("Invalid red component {} (00-ff)", v))?;
    let g = u8::from_str_radix(&hex[2..4], 16)
        .map_err(|v| format!("Invalid green component {} (00-ff)", v))?;
    let b = u8::from_str_radix(&hex[4..6], 16)
        .map_err(|v| format!("Invalid blue component {} (00-ff)", v))?;

    Ok((r, g, b))
}

fn timers_to_string(value: &Vec<(u64, (u8, u8, u8))>) -> String {
    value
        .iter()
        .map(|v| format!("{}#{:02x}{:02x}{:02x}", v.0, v.1.0, v.1.1, v.1.2))
        .collect::<Vec<_>>()
        .join(",")
}

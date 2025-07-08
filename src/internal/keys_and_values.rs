use std::collections::HashMap;
use std::fmt;
use std::fs;

#[derive(Debug)]
pub enum ParseError {
    MissingKey(&'static str),
    InvalidFormat {
        key: &'static str,
        value: String,
        expected: &'static str,
        error: String,
    },
}

pub struct KeysAndValues {
    map: HashMap<String, String>,
}

impl KeysAndValues {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn new_from_str(data: &str) -> Result<Self, ParseError> {
        Ok(Self {
            map: data
                .lines()
                .filter_map(|line| {
                    if line.is_empty() {
                        return None;
                    }

                    parse(line)
                })
                .collect(),
        })
    }

    pub fn to_string(&self) -> String {
        self.map
            .iter()
            .map(|entry| format!("{}={}", entry.0, entry.1))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn write_to_file(&self, file: &str) -> std::io::Result<()> {
        fs::write(file, self.to_string())
    }

    pub fn get<T, F, E>(&self, key: &'static str, conversion: F) -> Result<T, ParseError>
    where
        F: Fn(&str) -> Result<T, E>,
        E: std::fmt::Display,
    {
        if let Some(value) = self.map.get(key) {
            conversion(value).map_err(|e| ParseError::InvalidFormat {
                key,
                value: value.to_string(),
                expected: std::any::type_name::<T>(),
                error: e.to_string(),
            })
        } else {
            Err(ParseError::MissingKey(key))
        }
    }

    pub fn set<T, F>(&mut self, key: &str, value: &T, conversion: F)
    where
        F: Fn(&T) -> String,
    {
        self.map.insert(key.to_string(), conversion(value));
    }
}

fn parse(line: &str) -> Option<(String, String)> {
    if !line.contains("=") {
        return None;
    }
    let (key, value) = line.split_at(
        line.find("=")
            .expect("Expected '=' in config line Example: key=value"),
    );
    Some((key.to_string(), value[1..].to_string()))
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::MissingKey(key) => write!(f, "Missing required key: {}", key),
            ParseError::InvalidFormat {
                key,
                value,
                expected,
                error,
            } => {
                write!(
                    f,
                    "Invalid value '{}' for key '{}': expected {}, error: {}",
                    value, key, expected, error
                )
            }
        }
    }
}

impl std::error::Error for ParseError {}

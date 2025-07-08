use std::env;
use std::fs;
use std::path::PathBuf;

fn config_dir() -> Result<PathBuf, &'static str> {
    let os_config_dir = match env::consts::OS {
        "linux" => {
            if let Some(xdg) = env::var_os("XDG_CONFIG_HOME") {
                Ok(PathBuf::from(xdg))
            } else if let Some(home) = env::var_os("HOME") {
                Ok(PathBuf::from(home).join(".config"))
            } else {
                Err("env var $HOME is not set.")
            }
        }

        "windows" => match env::var_os("APPDATA") {
            Some(appdata) => Ok(PathBuf::from(appdata)),
            None => Err("env var %APPDATA% is not set."),
        },

        "macos" => match env::var_os("HOME") {
            Some(home) => Ok(PathBuf::from(home)
                .join("Library")
                .join("Application Support")),
            None => Err("env var $HOME is not set."),
        },

        _ => Err("OS not supported"),
    }?;

    Ok(os_config_dir.join("interval-timer"))
}

pub fn default_config_path() -> Result<String, &'static str> {
    let dir = config_dir()?;
    let path = dir.join("config.txt");
    Ok(path.to_string_lossy().to_string())
}

pub fn create_dirs_if_not_exists() -> Result<(), &'static str> {
    let dir = config_dir()?;
    fs::create_dir_all(dir).ok();
    Ok(())
}

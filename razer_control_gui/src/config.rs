use serde::{Deserialize, Serialize};
use std::{fs, fs::File, io, env};
use std::io::prelude::*;

const SETTINGS_FILE: &str = "/.local/share/razercontrol/daemon.json";
const EFFECTS_FILE: &str = "/.local/share/razercontrol/effects.json";

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct PowerConfig {
    pub power_mode: u8,
    pub cpu_boost: u8,
    pub gpu_boost: u8,
    pub fan_rpm: i32,
    pub brightness: u8,
    pub logo_state: u8,
    pub standard_effect: u8,
}

impl PowerConfig {
    pub fn new() -> PowerConfig {
        return PowerConfig{
            power_mode: 0,
            cpu_boost: 1,
            gpu_boost: 0,
            fan_rpm: 0,
            brightness: 128,
            logo_state: 0,
            standard_effect: 0, // off
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub power: [PowerConfig; 2],
    pub sync: bool, // sync light settings between ac and battery
    pub no_light: u8, // no light bellow this percentage of battery
    pub screensaver: bool, // turno of keyboard light if screen is blank
}

impl Configuration {
    pub fn new() -> Configuration {
        return Configuration {
            power: [PowerConfig::new(), PowerConfig::new()],
            sync: false,
            no_light: 0,
            screensaver: false,
        };
    }

    pub fn write_to_file(&mut self) -> io::Result<()> {
        let j: String = serde_json::to_string_pretty(&self)?;
        File::create(env::var("HOME").unwrap() + SETTINGS_FILE)?.write_all(j.as_bytes())?;
        Ok(())
    }

    pub fn read_from_config() -> io::Result<Configuration> {
        let str = fs::read_to_string(env::var("HOME").unwrap() + SETTINGS_FILE)?;
        let res: Configuration = serde_json::from_str(str.as_str())?;
        Ok(res)
    }

    pub fn write_effects_save(json: serde_json::Value) -> io::Result<()> {
        let j: String = serde_json::to_string_pretty(&json)?;
        File::create(env::var("HOME").unwrap() + EFFECTS_FILE)?.write_all(j.as_bytes())?;
        Ok(())
    }

    pub fn read_effects_file() -> io::Result<serde_json::Value> {
        let str = fs::read_to_string(env::var("HOME").unwrap() + EFFECTS_FILE)?;
        let res: serde_json::Value = serde_json::from_str(str.as_str())?;
        Ok(res)
    }
}

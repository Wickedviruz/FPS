use bevy::prelude::*;
use std::{collections::HashMap, fs, path::PathBuf};

#[derive(Resource, Debug, Clone)]
pub struct GameConfig {
    pub values: HashMap<String, String>,
}

impl Default for GameConfig {
    fn default() -> Self {
        let mut values = HashMap::new();
        values.insert("cl_fullscreen".into(), "1".into());
        values.insert("cl_resolution".into(), "1920x1080".into());
        values.insert("cl_vsync".into(), "0".into());
        Self { values }
    }
}

impl GameConfig {
    pub fn load() -> Self {
        let path = config_path();
        if let Ok(data) = fs::read_to_string(&path) {
            let mut values = HashMap::new();
            for line in data.lines() {
                if let Some((key, val)) = parse_line(line) {
                    values.insert(key.to_string(), val.to_string());
                }
            }
            return Self { values };
        }
        Self::default()
    }

    pub fn save(&self) {
        let mut out = String::new();
        for (key, val) in &self.values {
            out.push_str(&format!("{} \"{}\"\n", key, val));
        }
        let _ = fs::write(config_path(), out);
    }
}

fn parse_line(line: &str) -> Option<(&str, &str)> {
    let mut parts = line.splitn(2, ' ');
    let key = parts.next()?.trim();
    let val = parts.next()?.trim().trim_matches('"');
    Some((key, val))
}

fn config_path() -> PathBuf {
    let mut dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    dir.push("rust_fps_net");
    fs::create_dir_all(&dir).ok();
    dir.push("config.cfg");
    dir
}

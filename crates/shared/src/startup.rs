use bevy::prelude::*;
use bevy::window::{Window, WindowMode};
use crate::config::GameConfig;

pub struct StartupConfigPlugin;

impl Plugin for StartupConfigPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameConfig::load())
           .add_systems(Startup, apply_config_on_startup);
    }
}

fn apply_config_on_startup(
    mut windows: Query<&mut Window>,
    config: Res<GameConfig>,
) {
    if let Ok(mut window) = windows.get_single_mut() {
        // Fullscreen
        let fullscreen = config.values.get("cl_fullscreen").map(|s| s == "1").unwrap_or(false);
        window.mode = if fullscreen {
            WindowMode::Fullscreen
        } else {
            WindowMode::Windowed
        };

        // Resolution
        if let Some(res) = config.values.get("cl_resolution") {
            if let Some((w, h)) = res.split_once('x') {
                if let (Ok(w), Ok(h)) = (w.parse::<f32>(), h.parse::<f32>()) {
                    window.resolution.set(w, h);
                }
            }
        }
    }
}

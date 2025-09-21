use bevy::prelude::*;
use shared::AppState;
use core::CorePlugin;
use map::MapPlugin;
use physics::PhysicsPlugin;
use ui::UiPlugin;
use audio::AudioPlugin;
use shared::config::GameConfig;
use shared::startup::StartupConfigPlugin;

fn main() {
    App::new()
        .insert_resource(GameConfig::load())
        .add_plugins(DefaultPlugins)
        .add_plugins((
            StartupConfigPlugin,
            CorePlugin,
            PhysicsPlugin,
            UiPlugin,
            AudioPlugin,
            MapPlugin,
        ))
        .init_state::<AppState>()
        .add_systems(Startup, |mut next_state: ResMut<NextState<AppState>>| {
            next_state.set(AppState::MainMenu);
        })
        .run();
}

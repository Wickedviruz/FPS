use bevy::prelude::*;
use core::CorePlugin;
use physics::PhysicsPlugin;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins) // headless server
        .add_plugins((
            CorePlugin,
            PhysicsPlugin,
        ))
        .run();
}

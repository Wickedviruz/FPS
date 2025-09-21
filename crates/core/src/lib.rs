use bevy::prelude::*;

pub mod player;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, hello_system);
    }
}

fn hello_system() {
    // placeholder – här kommer game logic
    // println!("Core system tick");
}

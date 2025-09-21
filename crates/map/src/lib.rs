pub mod dummy_world;
pub mod targets;

use bevy::prelude::*;
use dummy_world::DummyWorldPlugin;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DummyWorldPlugin);
    }
}

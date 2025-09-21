use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MapId {
    Tutorial,
    BoxArena,
}

#[derive(Resource, Debug, Clone, Copy)]
pub struct SelectedMap(pub MapId);

/// Global application states
#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum AppState {
    #[default]
    Loading,
    MainMenu,
    PlayMenu,
    InventoryMenu,
    OptionsMenu(OptionsSubState),
    InGame,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Default, States)]
pub enum OptionsSubState {
    #[default]
    Root,           // listan med alla kategorier
    KeyboardMouse,
    GameSettings,
    VideoSettings,
    AudioSettings,
    Credits,
}
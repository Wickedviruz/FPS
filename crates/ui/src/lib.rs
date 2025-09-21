use bevy::prelude::*;
use bevy_egui::EguiPlugin;

pub mod navbar;
pub mod home_menu;
pub mod play_menu;
pub mod inventory_menu;
pub mod options_menu;
pub mod background;
pub mod playerbox;
pub mod friendlist;
pub mod loading_screen;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
           .add_plugins((
               background::BackgroundPlugin,
               navbar::NavbarPlugin,
               home_menu::HomeMenuPlugin,
               play_menu::PlayMenuPlugin,
               inventory_menu::InventoryMenuPlugin,
               options_menu::OptionsMenuPlugin,
               loading_screen::LoadingScreenPlugin,
           ));
    }
}

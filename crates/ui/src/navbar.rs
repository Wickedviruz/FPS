//! This module defines a navigation bar UI component for a Bevy application.
//!
//! # Overview
//! The navigation bar provides buttons for navigating between different application states,
//! such as Home, Play, Inventory, Options, and Exit. It visually highlights the active and hovered
//! buttons and handles user interactions to trigger state changes or exit the application.
//!
//! # Components
//! - `NavItem`: Enum representing each navigation button and its associated label and target state.
//! - `UiFont`: Resource for storing the UI font handle.
//! - `NavRoot`: Marker component for the root node of the navigation bar.
//! - `NavButton`: Component for navigation buttons, storing the associated `NavItem`.
//!
//! # Systems
//! - `load_font`: Loads the UI font and inserts it as a resource.
//! - `spawn_navbar`: Spawns the navigation bar UI, including buttons and separators.
//! - `spawn_nav_button`: Helper function to spawn individual navigation buttons.
//! - `nav_button_interactions`: Handles button press interactions, triggering state changes or exiting.
//! - `update_active_highlight`: Updates button appearance based on hover and active state.
//!
//! # Plugin
//! - `NavbarPlugin`: Registers all systems required for the navigation bar.
//!
//! # Usage
//! Add `NavbarPlugin` to your Bevy app to enable the navigation bar UI.
//!
//! # Dependencies
//! - Requires the `shared` crate for `AppState` and `OptionsSubState`.
//! - Expects a font asset at `"fonts/Inter-Bold.ttf"`.
//!
//! # UI Details
//! - The navigation bar is styled with a background color, border, and horizontal layout.
//! - Buttons visually indicate hover and active states with color and underline effects.
//! - A vertical separator is placed between buttons except after the last one.
use bevy::app::AppExit;
use bevy::prelude::*;
use shared::AppState;
use shared::types::OptionsSubState;

// vilka knappar vi har och vilken state de leder till
#[derive(Clone, Copy, PartialEq, Eq)]
enum NavItem {
    Home,
    Play,
    Inventory,
    Options,
    Exit,
}

impl NavItem {
    fn label(self) -> &'static str {
        match self {
            NavItem::Home => "HOME",
            NavItem::Play => "PLAY",
            NavItem::Inventory => "INVENTORY",
            NavItem::Options => "OPTIONS",
            NavItem::Exit => "EXIT",
        }
    }

    fn target_state(self) -> Option<AppState> {
        match self {
            NavItem::Home => Some(AppState::MainMenu),
            NavItem::Play => Some(AppState::PlayMenu),
            NavItem::Inventory => Some(AppState::InventoryMenu),
            NavItem::Options => Some(AppState::OptionsMenu(OptionsSubState::Root)),
            NavItem::Exit => None,
        }
    }
}

#[derive(Resource, Clone)]
struct UiFont(Handle<Font>);

#[derive(Component)]
struct NavRoot;

#[derive(Component)]
struct NavButton {
    item: NavItem,
}

pub struct NavbarPlugin;

impl Plugin for NavbarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_font)
            .add_systems(PostStartup, spawn_navbar)
            .add_systems(Update, (nav_button_interactions, update_active_highlight));
    }
}

fn load_font(mut commands: Commands, assets: Res<AssetServer>) {
    let handle: Handle<Font> = assets.load("fonts/Inter-Bold.ttf");
    commands.insert_resource(UiFont(handle));
}

fn spawn_navbar(mut commands: Commands, font: Res<UiFont>) {
    let bg = Color::srgb(0.10, 0.12, 0.16);
    let items = [
        NavItem::Home,
        NavItem::Play,
        NavItem::Inventory,
        NavItem::Options,
        NavItem::Exit,
    ];

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(48.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::bottom(Val::Px(1.0)),
                    ..default()
                },
                background_color: BackgroundColor(bg),
                border_color: BorderColor(Color::srgba(1.0, 1.0, 1.0, 0.1)),
                z_index: ZIndex::Global(10),
                ..default()
            },
            NavRoot,
        ))
        .with_children(|root| {
            for (i, item) in items.iter().enumerate() {
                spawn_nav_button(root, *item, &font);

                // Lägg in en separator om det inte är sista knappen
                if i < items.len() - 1 {
                    root.spawn(NodeBundle {
                        style: Style {
                            width: Val::Px(1.0),
                            height: Val::Percent(50.0), // halva navbarens höjd
                            margin: UiRect::horizontal(Val::Px(8.0)),
                            ..default()
                        },
                        background_color: Color::srgba(1.0, 1.0, 1.0, 0.2).into(),
                        ..default()
                    });
                }
            }
        });
}


fn spawn_nav_button(parent: &mut ChildBuilder, item: NavItem, font: &UiFont) {
    let idle_text = Color::srgb(0.80, 0.82, 0.86);
    let idle_bg = Color::NONE;

    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    padding: UiRect::axes(Val::Px(10.0), Val::Px(4.0)),
                    align_items: AlignItems::Center,
                    border: UiRect::bottom(Val::Px(2.0)),
                    ..default()
                },
                background_color: BackgroundColor(idle_bg),
                border_color: BorderColor(Color::srgba(1.0, 1.0, 1.0, 0.0)),
                ..default()
            },
            NavButton { item },
        ))
        .with_children(|btn| {
            let color =idle_text;
            btn.spawn(TextBundle::from_section(
                item.label(),
                TextStyle {
                    font: font.0.clone(),
                    font_size: 18.0,
                    color,
                },
            ));
        });
}

fn nav_button_interactions(
    mut q: Query<(&Interaction, &NavButton), (Changed<Interaction>, With<Button>)>,
    mut next_state: ResMut<NextState<AppState>>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, nav) in &mut q {
        if *interaction == Interaction::Pressed {
            if let Some(target) = nav.item.target_state() {
                next_state.set(target);
            } else if nav.item == NavItem::Exit {
                exit.send(AppExit::Success);
            }
        }
    }
}

// färger för hover/aktiv + underline
fn update_active_highlight(
    state: Res<State<AppState>>,
    mut q: Query<(&NavButton, &mut BorderColor, &mut BackgroundColor, &Children), With<Button>>,
    mut texts: Query<&mut Text>,
    interactions: Query<&Interaction, With<Button>>,
) {
    let active = state.get().clone();

    for (nav, mut border, mut bg, children) in &mut q {
        if let Some(&child) = children.first() {
            if let Ok(mut text) = texts.get_mut(child) {
                let hovered = interactions
                    .get(child)
                    .map(|i| matches!(*i, Interaction::Hovered))
                    .unwrap_or(false);

                let is_active = nav.item.target_state().is_some_and(|s| s == active);

                border.0 = if is_active {
                    Color::srgb(1.0, 1.0, 1.0)
                } else {
                    Color::srgba(1.0, 1.0, 1.0, 0.0)
                };

                if is_active {
                    text.sections[0].style.color = Color::srgb(1.0, 1.0, 1.0);
                    bg.0 = Color::NONE;
                } else if hovered {
                    text.sections[0].style.color = Color::srgb(0.92, 0.94, 0.98);
                    bg.0 = Color::srgba(1.0, 1.0, 1.0, 0.04);
                } else {
                    text.sections[0].style.color = Color::srgb(0.80, 0.82, 0.86);
                    bg.0 = Color::NONE;
                }
            }
        }
    }
}

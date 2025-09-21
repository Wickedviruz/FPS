use bevy::prelude::*;
use shared::AppState;

// återanvändbara komponenter
use crate::playerbox::spawn_playerbox;
use crate::friendlist::spawn_friendlist;

pub struct PlayMenuPlugin;

impl Plugin for PlayMenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectedMap(MapId::Tutorial)) // default
            .add_systems(OnEnter(AppState::PlayMenu), spawn_play_menu)
            .add_systems(OnExit(AppState::PlayMenu), cleanup_play_menu)
            .add_systems(Update, (
                map_button_interactions,
                play_button_interactions,
            ).run_if(in_state(AppState::PlayMenu)));
    }
}

#[derive(Component)]
struct PlayMenuRoot;

#[derive(Component)]
struct MapButton(MapId);

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapId {
    Tutorial,
    BoxArena,
}

#[derive(Resource, Debug, Clone, Copy)]
pub struct SelectedMap(pub MapId);

fn spawn_play_menu(mut commands: Commands, asset_server: Res<AssetServer>, selected: Res<SelectedMap>) {
    let font = asset_server.load("fonts/Inter-Bold.ttf");

    // Root
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                padding: UiRect::all(Val::Px(16.0)),
                margin: UiRect::top(Val::Px(48.0)), // lämna plats för navbar
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        },
        PlayMenuRoot,
    ))
    .with_children(|root| {
        // -----------------------------
        // Left: Profile + Friends + Settings
        // -----------------------------
        root.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(30.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(12.0),
                ..default()
            },
            background_color: Color::srgba(0.05, 0.05, 0.07, 0.9).into(),
            ..default()
        })
        .with_children(|left| {
            spawn_playerbox(left, &font, &font);
            spawn_friendlist(left, &font, &font);
            spawn_play_button(left, &font);

            left.spawn(TextBundle::from_section(
                "Game Settings",
                TextStyle {
                    font: font.clone(),
                    font_size: 20.0,
                    color: Color::srgb(0.7, 0.85, 1.0),
                },
            ));

            spawn_game_setting(left, &font, "Permissions", "Invited Friends Only");
            spawn_game_setting(left, &font, "Lobby Leader", "Uber- (Global Elite)");

            // Kartväljare
            left.spawn(TextBundle::from_section(
                "Select Map",
                TextStyle {
                    font: font.clone(),
                    font_size: 18.0,
                    color: Color::srgb(0.7, 0.85, 1.0),
                },
            ));

            for map in [MapId::Tutorial, MapId::BoxArena] {
                spawn_map_button(left, &font, map, map == selected.0);
            }
        });

        // -----------------------------
        // Right: Lobby + chat
        // -----------------------------
        root.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(70.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: Color::srgba(0.07, 0.07, 0.1, 0.9).into(),
            ..default()
        })
        .with_children(|right| {
            // Lobby player list
            right.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::FlexStart,
                    column_gap: Val::Px(12.0),
                    padding: UiRect::all(Val::Px(8.0)),
                    ..default()
                },
                ..default()
            })
            .with_children(|players| {
                for name in ["Uber", "DOXA", "CapuRoss", "Jeffty", "Professor Manly"] {
                    spawn_lobby_player(players, &font, name);
                }
            });

            // Chat box
            right.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    flex_grow: 1.0,
                    margin: UiRect::all(Val::Px(8.0)),
                    ..default()
                },
                background_color: Color::srgba(0.02, 0.02, 0.03, 0.9).into(),
                ..default()
            })
            .with_children(|chat| {
                chat.spawn(TextBundle::from_section(
                    "Connected to lobby...\nUber: Hello team!\nJeffty: Ready!",
                    TextStyle {
                        font: font.clone(),
                        font_size: 14.0,
                        color: Color::srgb(0.9, 0.9, 0.9),
                    },
                ));
            });

            // Bottom buttons
            right.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    padding: UiRect::all(Val::Px(12.0)),
                    ..default()
                },
                ..default()
            })
            .with_children(|buttons| {
                spawn_button(buttons, &font, "INVITE FRIENDS");
                spawn_button(buttons, &font, "INVENTORY");
            });
        });
    });
}

/// ==== Helpers ====

fn spawn_map_button(parent: &mut ChildBuilder, font: &Handle<Font>, map: MapId, active: bool) {
    let label = match map {
        MapId::Tutorial => "Tutorial Map",
        MapId::BoxArena => "Box Arena",
    };

    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    padding: UiRect::axes(Val::Px(10.0), Val::Px(6.0)),
                    margin: UiRect::bottom(Val::Px(4.0)),
                    ..default()
                },
                background_color: if active {
                    Color::srgb(0.2, 0.4, 0.7).into()
                } else {
                    Color::srgba(0.1, 0.1, 0.1, 0.8).into()
                },
                ..default()
            },
            MapButton(map),
        ))
        .with_children(|btn| {
            btn.spawn(TextBundle::from_section(
                label,
                TextStyle {
                    font: font.clone(),
                    font_size: 14.0,
                    color: Color::WHITE,
                },
            ));
        });
}

fn map_button_interactions(
    mut q: Query<(&Interaction, &MapButton, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
    mut selected: ResMut<SelectedMap>,
) {
    for (interaction, btn, mut bg) in &mut q {
        if *interaction == Interaction::Pressed {
            selected.0 = btn.0;
            *bg = Color::srgb(0.2, 0.4, 0.7).into();
            info!("Selected map: {:?}", selected.0);
        }
    }
}

fn spawn_game_setting(parent: &mut ChildBuilder, font: &Handle<Font>, label: &str, value: &str) {
    parent.spawn(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(8.0)),
            ..default()
        },
        background_color: Color::srgba(0.1, 0.1, 0.15, 0.6).into(),
        ..default()
    })
    .with_children(|row| {
        row.spawn(TextBundle::from_section(
            label,
            TextStyle {
                font: font.clone(),
                font_size: 16.0,
                color: Color::srgb(0.8, 0.8, 0.9),
            },
        ));
        row.spawn(TextBundle::from_section(
            value,
            TextStyle {
                font: font.clone(),
                font_size: 14.0,
                color: Color::srgb(0.9, 0.9, 0.9),
            },
        ));
    });
}

fn spawn_lobby_player(parent: &mut ChildBuilder, font: &Handle<Font>, name: &str) {
    parent.spawn(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(6.0)),
            ..default()
        },
        background_color: Color::srgba(0.12, 0.12, 0.18, 0.8).into(),
        ..default()
    })
    .with_children(|p| {
        p.spawn(NodeBundle {
            style: Style {
                width: Val::Px(48.0),
                height: Val::Px(48.0),
                ..default()
            },
            background_color: Color::srgb(0.3, 0.3, 0.4).into(),
            ..default()
        });
        p.spawn(TextBundle::from_section(
            name,
            TextStyle {
                font: font.clone(),
                font_size: 14.0,
                color: Color::srgb(0.9, 0.9, 0.9),
            },
        ));
    });
}

fn spawn_button(parent: &mut ChildBuilder, font: &Handle<Font>, label: &str) {
    parent
        .spawn(ButtonBundle {
            style: Style {
                padding: UiRect::axes(Val::Px(14.0), Val::Px(6.0)),
                ..default()
            },
            background_color: Color::srgba(0.1, 0.1, 0.1, 0.8).into(),
            ..default()
        })
        .with_children(|btn| {
            btn.spawn(TextBundle::from_section(
                label,
                TextStyle {
                    font: font.clone(),
                    font_size: 14.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
                },
            ));
        });
}

fn spawn_play_button(parent: &mut ChildBuilder, font: &Handle<Font>) {
    parent.spawn((
        ButtonBundle {
            style: Style {
                padding: UiRect::axes(Val::Px(20.0), Val::Px(10.0)),
                margin: UiRect::top(Val::Px(10.0)),
                ..default()
            },
            background_color: Color::srgba(0.2, 0.6, 0.2, 0.9).into(),
            ..default()
        },
        PlayButton,
    ))
    .with_children(|btn| {
        btn.spawn(TextBundle::from_section(
            "PLAY",
            TextStyle {
                font: font.clone(),
                font_size: 18.0,
                color: Color::WHITE,
            },
        ));
    });
}

#[derive(Component)]
struct PlayButton;

fn play_button_interactions(
    mut q: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<PlayButton>)>,
    mut next_state: ResMut<NextState<AppState>>,
    selected: Res<SelectedMap>,
) {
    for (interaction, mut bg) in &mut q {
        match *interaction {
            Interaction::Pressed => {
                *bg = Color::srgb(0.1, 0.4, 0.1).into();
                // Gå till Loading screen och ta med vald karta
                next_state.set(AppState::Loading);
                info!("Starting game with map {:?}", selected.0);
            }
            Interaction::Hovered => {
                *bg = Color::srgb(0.3, 0.7, 0.3).into();
            }
            Interaction::None => {
                *bg = Color::srgba(0.2, 0.6, 0.2, 0.9).into();
            }
        }
    }
}

fn cleanup_play_menu(mut commands: Commands, q: Query<Entity, With<PlayMenuRoot>>) {
    for e in &q {
        commands.entity(e).despawn_recursive();
    }
}

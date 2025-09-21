use bevy::prelude::*;
use shared::config::GameConfig;
use bevy::window::{Window, WindowMode, PresentMode};
use shared::AppState;
use shared::types::OptionsSubState;

pub struct OptionsMenuPlugin;

impl Plugin for OptionsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::OptionsMenu(OptionsSubState::Root)), spawn_options_menu)
            .add_systems(OnEnter(AppState::OptionsMenu(OptionsSubState::KeyboardMouse)), spawn_options_menu)
            .add_systems(OnEnter(AppState::OptionsMenu(OptionsSubState::GameSettings)), spawn_options_menu)
            .add_systems(OnEnter(AppState::OptionsMenu(OptionsSubState::VideoSettings)), spawn_options_menu)
            .add_systems(OnEnter(AppState::OptionsMenu(OptionsSubState::AudioSettings)), spawn_options_menu)
            .add_systems(OnEnter(AppState::OptionsMenu(OptionsSubState::Credits)), spawn_options_menu)
            .add_systems(OnExit(AppState::OptionsMenu(OptionsSubState::Root)), cleanup_options_menu)
            .add_systems(OnExit(AppState::OptionsMenu(OptionsSubState::KeyboardMouse)), cleanup_options_menu)
            .add_systems(OnExit(AppState::OptionsMenu(OptionsSubState::GameSettings)), cleanup_options_menu)
            .add_systems(OnExit(AppState::OptionsMenu(OptionsSubState::VideoSettings)), cleanup_options_menu)
            .add_systems(OnExit(AppState::OptionsMenu(OptionsSubState::AudioSettings)), cleanup_options_menu)
            .add_systems(OnExit(AppState::OptionsMenu(OptionsSubState::Credits)), cleanup_options_menu)
            .add_systems(
                Update,
                (
                    subnav_button_interactions,
                    update_subnav_highlight,
                    apply_window_settings, // nytt system
                ),
            );
    }
}

#[derive(Component)]
struct OptionsMenuRoot;

#[derive(Component)]
struct SubNavRoot;

#[derive(Component)]
struct SubNavButton(OptionsSubState);

#[derive(Component)]
struct ApplyButton;

fn spawn_options_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<State<AppState>>,
) {
    let font = asset_server.load("fonts/Inter-Bold.ttf");

    let substate = match state.get() {
        AppState::OptionsMenu(sub) => sub.clone(),
        _ => return,
    };

    // Root container
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    margin: UiRect::top(Val::Px(48.0)), // lämna plats för huvud-navbar
                    ..default()
                },
                background_color: Color::NONE.into(),
                ..default()
            },
            OptionsMenuRoot,
        ))
        .with_children(|root| {
            // === Sekundär navbar ===
            let bg = Color::srgb(0.10, 0.12, 0.16);
            let subs = [
                OptionsSubState::KeyboardMouse,
                OptionsSubState::GameSettings,
                OptionsSubState::VideoSettings,
                OptionsSubState::AudioSettings,
                OptionsSubState::Credits,
            ];

            root.spawn((
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
                    ..default()
                },
                SubNavRoot,
            ))
            .with_children(|bar| {
                for (i, sub) in subs.iter().enumerate() {
                    spawn_subnav_button(bar, sub.clone(), &font, sub == &substate);

                    if i < subs.len() - 1 {
                        bar.spawn(NodeBundle {
                            style: Style {
                                width: Val::Px(1.0),
                                height: Val::Percent(50.0),
                                margin: UiRect::horizontal(Val::Px(8.0)),
                                ..default()
                            },
                            background_color: Color::srgba(1.0, 1.0, 1.0, 0.2).into(),
                            ..default()
                        });
                    }
                }
            });

            // === Innehållssektionen ===
            root.spawn(NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    padding: UiRect::all(Val::Px(16.0)),
                    row_gap: Val::Px(8.0),
                    ..default()
                },
                background_color: Color::NONE.into(),
                ..default()
            })
            .with_children(|content| {
                match substate {
                    OptionsSubState::VideoSettings => spawn_video_settings(content, &font),
                    OptionsSubState::GameSettings => {
                        spawn_dummy_section(content, &font, "GAME SETTINGS", &[
                            ("Difficulty", "Normal"),
                            ("HUD", "Enabled"),
                            ("Crosshair", "Default"),
                        ]);
                    }
                    OptionsSubState::KeyboardMouse => {
                        spawn_dummy_section(content, &font, "KEYBOARD / MOUSE", &[
                            ("Sensitivity", "1.2"),
                            ("Invert Mouse", "Off"),
                            ("Key Bindings", "[Open Menu]"),
                        ]);
                    }
                    OptionsSubState::AudioSettings => {
                        spawn_dummy_section(content, &font, "AUDIO SETTINGS", &[
                            ("Master Volume", "80%"),
                            ("Music Volume", "50%"),
                            ("Voice Chat", "Push-To-Talk"),
                        ]);
                    }
                    OptionsSubState::Credits => {
                        spawn_dummy_section(content, &font, "CREDITS", &[
                            ("Game Design", "Your Name"),
                            ("Programming", "Rust Devs"),
                            ("Engine", "Bevy"),
                        ]);
                    }
                    OptionsSubState::Root => {
                        content.spawn(TextBundle::from_section(
                            "Select a settings category above",
                            TextStyle {
                                font: font.clone(),
                                font_size: 18.0,
                                color: Color::srgb(0.8, 0.8, 0.8),
                            },
                        ));
                    }
                }
            });
        });
}

fn spawn_subnav_button(
    parent: &mut ChildBuilder,
    sub: OptionsSubState,
    font: &Handle<Font>,
    active: bool,
) {
    let idle_text = Color::srgb(0.80, 0.82, 0.86);

    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    padding: UiRect::axes(Val::Px(10.0), Val::Px(4.0)),
                    align_items: AlignItems::Center,
                    border: UiRect::bottom(Val::Px(2.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::NONE),
                border_color: BorderColor(if active {
                    Color::WHITE
                } else {
                    Color::srgba(1.0, 1.0, 1.0, 0.0)
                }),
                ..default()
            },
            SubNavButton(sub.clone()),
        ))
        .with_children(|btn| {
            let color = if active { Color::WHITE } else { idle_text };
            btn.spawn(TextBundle::from_section(
                format!("{:?}", sub).replace('_', " ").to_uppercase(),
                TextStyle {
                    font: font.clone(),
                    font_size: 18.0,
                    color,
                },
            ));
        });
}

fn subnav_button_interactions(
    mut q: Query<(&Interaction, &SubNavButton), (Changed<Interaction>, With<Button>)>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, btn) in &mut q {
        if *interaction == Interaction::Pressed {
            next_state.set(AppState::OptionsMenu(btn.0.clone()));
        }
    }
}

fn update_subnav_highlight(
    state: Res<State<AppState>>,
    mut q: Query<(&SubNavButton, &mut BorderColor, &mut BackgroundColor, &Children), With<Button>>,
    mut texts: Query<&mut Text>,
    interactions: Query<&Interaction, With<Button>>,
) {
    let active = state.get().clone();

    for (btn, mut border, mut bg, children) in &mut q {
        if let Some(&child) = children.first() {
            if let Ok(mut text) = texts.get_mut(child) {
                let hovered = interactions
                    .get(child)
                    .map(|i| matches!(*i, Interaction::Hovered))
                    .unwrap_or(false);

                let is_active = matches!(active, AppState::OptionsMenu(ref sub) if *sub == btn.0);

                border.0 = if is_active {
                    Color::WHITE
                } else {
                    Color::srgba(1.0, 1.0, 1.0, 0.0)
                };

                if is_active {
                    text.sections[0].style.color = Color::WHITE;
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

fn spawn_video_settings(root: &mut ChildBuilder, font: &Handle<Font>) {
    spawn_dummy_section(root, font, "VIDEO SETTINGS", &[
        ("Brightness", "Slider [ ]"),
        ("Resolution", "1920x1080"),
        ("Display Mode", "Fullscreen"),
    ]);
    spawn_dummy_section(root, font, "ADVANCED VIDEO OPTIONS", &[
        ("Shadow Quality", "Low"),
        ("Texture Detail", "Medium"),
        ("Anti-Aliasing", "None"),
    ]);
}

fn spawn_dummy_section(root: &mut ChildBuilder, font: &Handle<Font>, title: &str, rows: &[(&str, &str)]) {
    root.spawn(TextBundle::from_section(
        title,
        TextStyle {
            font: font.clone(),
            font_size: 20.0,
            color: Color::srgb(0.5, 0.7, 0.9),
        },
    ));

    for (label, value) in rows {
        spawn_setting_row(root, font, label, value);
    }

    if title.contains("VIDEO") {
        root.spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                justify_content: JustifyContent::SpaceBetween,
                margin: UiRect::top(Val::Px(20.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|buttons| {
            spawn_button(buttons, font, "BACK");
            spawn_button(buttons, font, "RESTORE DEFAULTS");

            // Apply-knapp markerad
            buttons
                .spawn((
                    ButtonBundle {
                        style: Style {
                            padding: UiRect::axes(Val::Px(12.0), Val::Px(6.0)),
                            ..default()
                        },
                        background_color: Color::srgba(0.1, 0.1, 0.1, 0.8).into(),
                        ..default()
                    },
                    ApplyButton,
                ))
                .with_children(|btn| {
                    btn.spawn(TextBundle::from_section(
                        "APPLY",
                        TextStyle {
                            font: font.clone(),
                            font_size: 16.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
    }
}

fn spawn_setting_row(parent: &mut ChildBuilder, font: &Handle<Font>, label: &str, value: &str) {
    parent
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                justify_content: JustifyContent::SpaceBetween,
                width: Val::Percent(100.0),
                padding: UiRect::axes(Val::Px(8.0), Val::Px(4.0)),
                ..default()
            },
            background_color: Color::srgba(0.05, 0.05, 0.05, 0.6).into(),
            ..default()
        })
        .with_children(|row| {
            row.spawn(TextBundle::from_section(
                label,
                TextStyle {
                    font: font.clone(),
                    font_size: 16.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
                },
            ));
            row.spawn(TextBundle::from_section(
                value,
                TextStyle {
                    font: font.clone(),
                    font_size: 16.0,
                    color: Color::srgb(0.8, 0.8, 0.8),
                },
            ));
        });
}

fn spawn_button(parent: &mut ChildBuilder, font: &Handle<Font>, label: &str) {
    parent
        .spawn(ButtonBundle {
            style: Style {
                padding: UiRect::axes(Val::Px(12.0), Val::Px(6.0)),
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
                    font_size: 16.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
                },
            ));
        });
}

fn apply_window_settings(
    mut windows: Query<&mut Window>,
    config: ResMut<GameConfig>,
    interaction_q: Query<&Interaction, With<ApplyButton>>,
) {
    for interaction in &interaction_q {
        if *interaction == Interaction::Pressed {
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
                    if let Some((w,h)) = res.split_once('x') {
                        if let (Ok(w), Ok(h)) = (w.parse::<f32>(), h.parse::<f32>()) {
                            window.resolution.set(w,h);
                        }
                    }
                }

                // VSync
                let vsync = config.values.get("cl_vsync").map(|s| s == "1").unwrap_or(true);
                window.present_mode = if vsync {
                    PresentMode::AutoVsync
                } else {
                    PresentMode::AutoNoVsync
                };
    
            }

            config.save();
            info!("Applied & saved config.cfg");
        }
    }
}
fn cleanup_options_menu(mut commands: Commands, q: Query<Entity, With<OptionsMenuRoot>>) {
    for e in &q {
        commands.entity(e).despawn_recursive();
    }
}

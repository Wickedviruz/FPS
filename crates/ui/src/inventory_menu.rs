use bevy::prelude::*;
use shared::AppState;

// återanvändbara komponenter
use crate::playerbox::spawn_playerbox;
use crate::friendlist::spawn_friendlist;

pub struct InventoryMenuPlugin;

impl Plugin for InventoryMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InventoryMenu), spawn_inventory_menu)
           .add_systems(OnExit(AppState::InventoryMenu), cleanup_inventory_menu);
    }
}

#[derive(Component)]
struct InventoryMenuRoot;

fn spawn_inventory_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/Inter-Bold.ttf");

    // Root
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                margin: UiRect::top(Val::Px(48.0)), // navbar space
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        },
        InventoryMenuRoot,
    ))
    .with_children(|root| {
        // -----------------------------
        // Left panel: profile + friends
        // -----------------------------
        root.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(25.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(8.0)),
                row_gap: Val::Px(8.0),
                ..default()
            },
            background_color: Color::srgba(0.05, 0.05, 0.07, 0.9).into(),
            ..default()
        })
        .with_children(|left| {
            spawn_playerbox(left, &font, &font);
            spawn_friendlist(left, &font, &font);
        });

        // -----------------------------
        // Right panel: inventory grid
        // -----------------------------
        root.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(75.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(8.0)),
                row_gap: Val::Px(8.0),
                ..default()
            },
            background_color: Color::srgba(0.07, 0.07, 0.1, 0.9).into(),
            ..default()
        })
        .with_children(|right| {
            // Top bar
            right.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    padding: UiRect::all(Val::Px(6.0)),
                    ..default()
                },
                background_color: Color::srgba(0.1, 0.1, 0.15, 0.9).into(),
                ..default()
            })
            .with_children(|bar| {
                for tab in ["Inventory", "Loadout", "Marketplace"] {
                    bar.spawn(TextBundle::from_section(
                        tab,
                        TextStyle {
                            font: font.clone(),
                            font_size: 16.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                        },
                    ));
                }
            });

            // Grid
            right.spawn(NodeBundle {
                style: Style {
                    display: Display::Grid,
                    grid_template_columns: RepeatedGridTrack::flex(6, 1.0),
                    grid_template_rows: RepeatedGridTrack::flex(5, 1.0),
                    row_gap: Val::Px(6.0),
                    column_gap: Val::Px(6.0),
                    ..default()
                },
                ..default()
            })
            .with_children(|grid| {
                for i in 0..30 {
                    grid.spawn(NodeBundle {
                        style: Style {
                            width: Val::Px(100.0),
                            height: Val::Px(100.0),
                            ..default()
                        },
                        background_color: if i % 2 == 0 {
                            Color::srgb(0.6, 0.6, 0.2).into() // gul "låda"
                        } else {
                            Color::srgb(0.3, 0.3, 0.3).into() // placeholder vapen
                        },
                        ..default()
                    })
                    .with_children(|cell| {
                        cell.spawn(TextBundle::from_section(
                            if i % 2 == 0 { "Case" } else { "AK-47" },
                            TextStyle {
                                font: font.clone(),
                                font_size: 12.0,
                                color: Color::srgb(0.9, 0.9, 0.9),
                            },
                        ));
                    });
                }
            });
        });
    });
}

fn cleanup_inventory_menu(mut commands: Commands, q: Query<Entity, With<InventoryMenuRoot>>) {
    for e in &q {
        commands.entity(e).despawn_recursive();
    }
}

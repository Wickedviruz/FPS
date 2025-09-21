use bevy::prelude::*;
use shared::AppState;

use crate::playerbox::spawn_playerbox;
use crate::friendlist::spawn_friendlist;

pub struct HomeMenuPlugin;

impl Plugin for HomeMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn_home_menu)
           .add_systems(OnExit(AppState::MainMenu), cleanup_home_menu);
    }
}

#[derive(Component)]
struct HomeMenuRoot;

fn spawn_home_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_bold = asset_server.load("fonts/Inter-Bold.ttf");
    let font_regular = asset_server.load("fonts/Inter-Bold.ttf");

    commands
    .spawn((
        NodeBundle {
            style: Style {
                display: Display::Flex,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(12.0)),
                column_gap: Val::Px(12.0),
                margin: UiRect {
                    top: Val::Px(48.0),
                    left: Val::Px(0.0),
                    right: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                },
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        },
        HomeMenuRoot,
    ))
    .with_children(|root| {
        // === Vänster kolumn ===
        root.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                width: Val::Percent(25.0),
                row_gap: Val::Px(12.0),
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        })
        .with_children(|left| {
            spawn_playerbox(left, &font_bold, &font_regular);
            spawn_friendlist(left, &font_bold, &font_regular);
        });

        // === Mitten kolumn ===
        root.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                flex_grow: 1.0,
                row_gap: Val::Px(12.0),
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        })
        .with_children(|mid| {
            // fortfarande kvar som paneler tills vi gör moduler av dessa
            spawn_panel(
                mid,
                &font_bold,
                &font_regular,
                "Featured",
                "[Image Banner Placeholder]\nOperation Wildfire\nThug Life Sticker",
            );

            spawn_panel(
                mid,
                &font_bold,
                &font_regular,
                "News",
                "ESL One Cologne 2016\nCS:GO’s Next Major\nJuly 5 – 10 in Cologne, Germany\n...",
            );
        });

        // === Höger kolumn ===
        root.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                width: Val::Percent(25.0),
                row_gap: Val::Px(12.0),
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        })
        .with_children(|right| {
            spawn_panel(
                right,
                &font_bold,
                &font_regular,
                "Statistics",
                "Unique Players Last Month:\n10,451,389",
            );

            spawn_panel(
                right,
                &font_bold,
                &font_regular,
                "Workshop Queue",
                "Vote for community maps!\n[Queue Placeholder]",
            );

            spawn_panel(
                right,
                &font_bold,
                &font_regular,
                "Live Streams",
                "Virtus.pro vs NiP – 123k viewers\nDreamHack – 59k viewers",
            );
        });
    });
}

/// Hjälper för vanliga paneler (tills vi bryter ut dessa också)
fn spawn_panel(
    parent: &mut ChildBuilder,
    font_bold: &Handle<Font>,
    font_regular: &Handle<Font>,
    title: &str,
    body: &str,
) {
    parent
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(8.0)),
                row_gap: Val::Px(6.0),
                ..default()
            },
            background_color: Color::srgba(0.05, 0.05, 0.05, 0.7).into(),
            ..default()
        })
        .with_children(|panel| {
            panel.spawn(TextBundle::from_section(
                title,
                TextStyle {
                    font: font_bold.clone(),
                    font_size: 18.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
                },
            ));
            panel.spawn(TextBundle::from_section(
                body,
                TextStyle {
                    font: font_regular.clone(),
                    font_size: 14.0,
                    color: Color::srgb(0.8, 0.8, 0.8),
                },
            ));
        });
}

fn cleanup_home_menu(mut commands: Commands, q: Query<Entity, With<HomeMenuRoot>>) {
    for e in &q {
        commands.entity(e).despawn_recursive();
    }
}

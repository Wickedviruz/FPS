use bevy::prelude::*;
use shared::AppState;
use shared::maps::MapId;

pub struct LoadingScreenPlugin;

impl Plugin for LoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Loading), spawn_loading_screen)
            .add_systems(OnExit(AppState::Loading), cleanup_loading_screen)
            .add_systems(Update, check_if_ready.run_if(in_state(AppState::Loading)));
    }
}

#[derive(Component)]
struct LoadingScreenRoot;

fn spawn_loading_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/Inter-Bold.ttf");

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::BLACK.into(),
                ..default()
            },
            LoadingScreenRoot,
        ))
        .with_children(|root| {
            root.spawn(TextBundle::from_section(
                "Loading...",
                TextStyle {
                    font: font.clone(),
                    font_size: 32.0,
                    color: Color::WHITE,
                },
            ));
        });
}

fn cleanup_loading_screen(mut commands: Commands, q: Query<Entity, With<LoadingScreenRoot>>) {
    for e in &q {
        commands.entity(e).despawn_recursive();
    }
}

// Senare kan vi vänta på att kartans assets laddas
fn check_if_ready(mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::InGame);
}

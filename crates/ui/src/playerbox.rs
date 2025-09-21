use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerBoxRoot;

pub fn spawn_playerbox(
    parent: &mut ChildBuilder,
    font_bold: &Handle<Font>,
    _font_regular: &Handle<Font>,
) {
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(8.0)),
                    row_gap: Val::Px(8.0),
                    ..default()
                },
                background_color: Color::srgba(0.05, 0.05, 0.05, 0.7).into(),
                ..default()
            },
            PlayerBoxRoot,
        ))
        .with_children(|row| {
            // Avatar placeholder
            row.spawn(NodeBundle {
                style: Style {
                    width: Val::Px(48.0),
                    height: Val::Px(48.0),
                    margin: UiRect::right(Val::Px(8.0)),
                    ..default()
                },
                background_color: Color::srgba(0.3, 0.3, 0.3, 1.0).into(),
                ..default()
            });

            // Text info
            row.spawn(TextBundle::from_section(
                "PlayerName\nRank: Gold Nova 1",
                TextStyle {
                    font: font_bold.clone(),
                    font_size: 14.0,
                    color: Color::WHITE,
                },
            ));
        });
}

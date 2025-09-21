use bevy::prelude::*;

#[derive(Component)]
pub struct FriendListRoot;

pub fn spawn_friendlist(
    parent: &mut ChildBuilder,
    font_bold: &Handle<Font>,
    font_regular: &Handle<Font>,
) {
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(8.0)),
                    row_gap: Val::Px(4.0),
                    ..default()
                },
                background_color: Color::srgba(0.05, 0.05, 0.05, 0.7).into(),
                ..default()
            },
            FriendListRoot,
        ))
        .with_children(|col| {
            col.spawn(TextBundle::from_section(
                "Friends Online (3)",
                TextStyle {
                    font: font_bold.clone(),
                    font_size: 16.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
                },
            ));

            for friend in ["Alice", "Bob", "Charlie"] {
                col.spawn(TextBundle::from_section(
                    friend,
                    TextStyle {
                        font: font_regular.clone(),
                        font_size: 14.0,
                        color: Color::srgb(0.8, 0.8, 0.8),
                    },
                ));
            }
        });
}

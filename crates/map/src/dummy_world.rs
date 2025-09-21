use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use shared::AppState;
use shared::components::Shootable;

pub struct DummyWorldPlugin;

impl Plugin for DummyWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_world)
           .add_systems(OnExit(AppState::InGame), cleanup_world);
    }
}

#[derive(Component)]
struct WorldRoot;

fn spawn_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Root
    let root = commands
        .spawn((SpatialBundle::default(), WorldRoot))
        .id();

    commands.entity(root).with_children(|parent| {
        let level_material = materials.add(StandardMaterial {
            base_color: Color::WHITE,
            ..default()
        });

        // Ground
        parent.spawn((
            Collider::cuboid(1000., 0.1, 1000.),
            PbrBundle {
                material: level_material.clone(),
                transform: Transform::IDENTITY,
                mesh: meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(1000.))),
                ..default()
            },
            Shootable,
        ));

        // A big box
        parent.spawn((
            Collider::cuboid(30., 30., 30.),
            PbrBundle {
                material: level_material.clone(),
                transform: Transform::from_xyz(0., 30., -100.),
                mesh: meshes.add(Cuboid::from_length(60.)),
                ..default()
            },
            Shootable,
        ));

        // Light
        parent.spawn(DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: light_consts::lux::OVERCAST_DAY,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(100., 200., 100.)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        });
    });
}

fn cleanup_world(mut commands: Commands, q: Query<Entity, With<WorldRoot>>) {
    for e in &q {
        commands.entity(e).despawn_recursive();
    }
}

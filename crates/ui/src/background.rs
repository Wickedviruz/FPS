use bevy::prelude::*;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::{
    Extent3d, TextureDimension, TextureFormat,
};

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_background);
        app.add_systems(Update, resize_background);
    }
}

#[derive(Component)]
struct BackgroundTag;

fn setup_background(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {
    let size = 512u32;
    let mut data = Vec::with_capacity((size * size * 4) as usize);

    let center = (size as f32 / 2.0, size as f32 / 2.0);
    let max_dist = (center.0.powi(2) + center.1.powi(2)).sqrt();

    for y in 0..size {
        for x in 0..size {
            let dx = x as f32 - center.0;
            let dy = y as f32 - center.1;
            let dist = (dx * dx + dy * dy).sqrt() / max_dist;

            // Vit i mitten -> blå i kanten
            let r = (1.0 - dist) * 1.0 + dist * 0.0;
            let g = (1.0 - dist) * 1.0 + dist * 0.0;
            let b = (1.0 - dist) * 1.0 + dist * 1.0;

            data.push((r * 255.0) as u8);
            data.push((g * 255.0) as u8);
            data.push((b * 255.0) as u8);
            data.push(255);
        }
    }

    let image = Image::new_fill(
        Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    );

    let handle = images.add(image);

    // Kamera
    commands.spawn(Camera2dBundle::default());

    // Sprite med komponent för auto-scaling
    commands.spawn((
        SpriteBundle {
            texture: handle,
            transform: Transform::from_scale(Vec3::splat(1.0)),
            ..default()
        },
        BackgroundTag,
    ));
}

/// Gör att bakgrunden alltid fyller hela fönstret
fn resize_background(
    windows: Query<&Window>,
    mut bg_query: Query<&mut Transform, With<BackgroundTag>>,
) {
    let window = windows.single();

    if let Ok(mut transform) = bg_query.get_single_mut() {
        // Skala bakgrunden så att den fyller hela fönstret
        let width = window.width();
        let height = window.height();
        transform.scale = Vec3::new(width / 512.0, height / 512.0, 1.0);
    }
}

use crate::{
    domino,
    floor::Floor,
    pusher::{self, Pusher},
};
use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use bevy_rapier3d::prelude::*;

const LIGHT_DISTANCE: f32 = 100.;
pub const FLOOR_LENGTH: f32 = 40.;
pub const FLOOR_HEIGHT: f32 = 1.;
pub const FLOOR_SIZE: Vec3 = vec3(FLOOR_LENGTH, FLOOR_HEIGHT, FLOOR_LENGTH);
pub const FLOOR_HALF_SIZE: Vec3 = vec3(FLOOR_SIZE.x * 0.5, FLOOR_SIZE.y * 0.5, FLOOR_SIZE.z * 0.5);
pub const PUSHER_OFFSET: f32 = 0.875;
pub const PUSHER_START_POS: Vec3 = vec3(
    FLOOR_HALF_SIZE.x * PUSHER_OFFSET,
    domino::DOMINO_Y_POS + domino::DOMINO_HALF_SIZE.y * 0.5,
    -FLOOR_HALF_SIZE.z * PUSHER_OFFSET,
);

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_environment);
    }
}

fn setup_environment(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    commands.spawn((
        Name::new("Light"),
        DirectionalLight {
            illuminance: 2500.,
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_xyz(0., LIGHT_DISTANCE * 0.5, -LIGHT_DISTANCE).looking_at(
            Vec3 {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            Dir3::Y,
        ),
    ));

    commands.spawn((
        Name::new("Floor"),
        Floor,
        RigidBody::Fixed,
        Ccd::enabled(),
        Collider::cuboid(FLOOR_HALF_SIZE.x, FLOOR_HALF_SIZE.y, FLOOR_HALF_SIZE.z),
        Mesh3d(meshes.add(Cuboid::new(FLOOR_SIZE.x, FLOOR_SIZE.y, FLOOR_SIZE.z))),
        MeshMaterial3d(debug_material.clone()),
        Transform::from_xyz(0., 0., 0.),
        Friction::new(1.),
        Restitution::new(0.),
    ));

    commands.spawn((
        Name::new("Pusher"),
        Pusher,
        RigidBody::Fixed,
        Collider::ball(pusher::RADIUS),
        Mesh3d(meshes.add(Sphere {
            radius: pusher::RADIUS,
        })),
        MeshMaterial3d(materials.add(pusher::PUSHER_COLOR)),
        Transform::from_translation(PUSHER_START_POS),
    ));
}

fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}

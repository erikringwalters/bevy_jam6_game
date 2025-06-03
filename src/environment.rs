use crate::floor::Floor;
use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use bevy_rapier3d::prelude::*;
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
    let light_distance = 100.;
    let camera_distance = 25.;
    let floor_length = 100.;
    let floor_height = 1.;
    let floor_size = vec3(floor_length, floor_height, floor_length);
    let floor_half_size = floor_size * 0.5;

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
        Transform::from_xyz(-light_distance, light_distance * 0.5, light_distance).looking_at(
            Vec3 {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            Dir3::Y,
        ),
    ));

    commands.spawn((
        Name::new("Camera"),
        Camera3d::default(),
        Transform::from_xyz(-camera_distance, camera_distance * 0.5, -camera_distance).looking_at(
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
        Collider::cuboid(floor_half_size.x, floor_half_size.y, floor_half_size.z),
        Mesh3d(meshes.add(Cuboid::new(floor_size.x, floor_size.y, floor_size.z))),
        MeshMaterial3d(debug_material.clone()),
        Transform::from_xyz(0., 0., 0.),
        Friction::new(1.),
        Restitution::new(0.1),
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

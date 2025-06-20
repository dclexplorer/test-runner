use bevy::prelude::*;
use bevy::window::{WindowPlugin, PresentMode, WindowMode};

const IMAGE_SIZE: (u32, u32) = (1920, 1080);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (IMAGE_SIZE.0 as f32, IMAGE_SIZE.1 as f32).into(),
                present_mode: PresentMode::AutoNoVsync,
                mode: WindowMode::Windowed,
                visible: false, // Headless
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create a cube mesh
    let cube_mesh = meshes.add(Mesh::from(Cuboid::new(2.0, 2.0, 2.0)));

    // Create a bright red material for the cube
    let cube_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.9, 0.1, 0.1), // Bright red
        metallic: 0.1,
        perceptual_roughness: 0.3,
        ..default()
    });

    // Spawn the cube at the center
    commands.spawn(PbrBundle {
        mesh: cube_mesh,
        material: cube_material,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    // Add a bright directional light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 10000.0, // Bright light
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(2.0, 4.0, 2.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::PI / 4.0),
            ..default()
        },
        ..default()
    });

    // Add ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 500.0, // Bright ambient light
    });

    // Create a camera looking at the cube
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        camera: Camera {
            clear_color: bevy::render::camera::ClearColorConfig::Custom(Color::srgb(0.2, 0.4, 0.8)),
            ..default()
        },
        ..default()
    });

    println!("3D cube scene setup complete!");
}

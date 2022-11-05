use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

mod tower;

pub use tower::*;

pub const HEIGHT: f32 = 800.;
pub const WIDTH: f32 = 1920.;
pub const GAME_NAME: &str = "Rusty Defender";

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(WindowDescriptor {
            width: WIDTH,
            height: HEIGHT,
            title: GAME_NAME.to_string(),
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(TowerPlugin)
        .add_startup_system(setup_scene)
        .add_startup_system(spawn_camera)
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    const PLANE_COLOR: Color = Color::rgb(0.6, 0.0, 0.75);
    const PLANE_SIZE: f32 = 5.;

    // Setup plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: PLANE_SIZE })),
        material: materials.add(PLANE_COLOR.into()),
        ..default()
    });
    // Light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera3dBundle {
        projection: OrthographicProjection {
            scale: 3.0,
            scaling_mode: ScalingMode::FixedVertical(1.4),
            ..default()
        }
            .into(),
        transform: Transform::from_xyz(4.0, 4.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

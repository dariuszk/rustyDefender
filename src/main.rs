use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

mod tower;
mod enemy;

pub use tower::*;
pub use enemy::*;

pub const HEIGHT: f32 = 800.;
pub const WIDTH: f32 = 1920.;
pub const GAME_NAME: &str = "Rusty Defender";

pub struct AssetsHolder{
    tower_t1: Handle<Scene>,
    enemy_e1: Handle<Scene>,
}


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
        .add_plugin(EnemyPlugin)
        .add_startup_system(setup_scene)
        .add_startup_system(spawn_camera)
        .add_startup_system_to_stage(StartupStage::PreStartup, preload_assets)
        .run();
}

fn preload_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(AssetsHolder {
        tower_t1: asset_server.load("TowerT1.glb#Scene0"),
        enemy_e1: asset_server.load("EnemyE1.glb#Scene0")
    });
}


fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    const PLANE_COLOR: Color = Color::rgb(0.6, 0.0, 0.75);
    const PLANE_SIZE: f32 = 10.;

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
            scale: 2.0,
            scaling_mode: ScalingMode::FixedVertical(4.),
            ..default()
        }
            .into(),
        transform: Transform::from_xyz(4.0, 4.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

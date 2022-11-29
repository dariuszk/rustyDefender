use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_inspector_egui::{WorldInspectorPlugin, Inspectable, RegisterInspectable};

mod tower;
mod enemy;
mod board;

pub use tower::*;
pub use enemy::*;
pub use board::*;

pub const HEIGHT: f32 = 800.;
pub const WIDTH: f32 = 1920.;
pub const GAME_NAME: &str = "Rusty Defender";

#[derive(Resource)]
pub struct AssetsHolder{
    tower_t1: Handle<Scene>,
    enemy_e1: Handle<Scene>,
}


#[derive( Resource)]
struct Game {
    nav_point_vecs:   [Vec3; 4]
}

impl FromWorld for Game {

    fn from_world(_world: &mut World) -> Self {
        Game {
            nav_point_vecs: [Vec3 { x: 5.0, y: 0.0, z: 0.0 }
                , Vec3 { x: 0.0, y: 0.0, z: -5.0 }
                , Vec3 { x: -5.0, y: 0.0, z: 0.0 }
                , Vec3 { x: 0.0, y: 0.0, z: 5.0 }]
        }
    }


}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {

    Gameplay,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .init_resource::<Game>(

        )
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window:  WindowDescriptor {
            width: WIDTH,
            height: HEIGHT,
            title: GAME_NAME.to_string(),
            resizable: false,
            ..default()
        },
        ..default()}))
        .add_state(GameState::Gameplay)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(TowerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(BoardPlugin)
        .add_startup_system(spawn_camera)

        .add_system_set(SystemSet::on_enter(GameState::Gameplay).with_system(setup_scene))
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
) {

    // Light
    commands.spawn(PointLightBundle {
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
    commands.spawn(Camera3dBundle {
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

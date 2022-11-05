use bevy::prelude::*;

#[derive(  Component)]
pub enum TowerType{
    T1,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Tower {

}

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Tower>().add_system(spawn_tower);
    }
}

fn spawn_tower(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>
)
{
    let tower_model = asset_server.load("TowerT1.glb#Scene0");
    commands
        .spawn_bundle(SceneBundle  {
             scene: tower_model,
             transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(TowerType::T1)
        .insert(Tower {

        } )
        .insert(Name::new("Tower"));
}
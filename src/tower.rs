use bevy::prelude::*;
use crate::AssetsHolder;


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
    game_assets: Res<AssetsHolder>,
)
{
    commands
        .spawn_bundle(SceneBundle  {
             scene: game_assets.tower_t1.clone(),
             transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(TowerType::T1)
        .insert(Tower {} )
        .insert(Name::new("Tower"));
}
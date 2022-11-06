use bevy::prelude::*;


//TODO: More fancy names needed
#[derive(Component)]
pub enum EnemyType{
    E1,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Enemy{

}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Enemy>()
            .add_system(spawn_enemy);
    }
}

fn spawn_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,

)
{
    let enemy_model =  asset_server.load("EnemyE1.glb#Scene0");

    commands.spawn_bundle( SceneBundle{
        scene: enemy_model,
        transform: Transform::from_xyz(1.0, 0.0, 0.0),
        ..default()
    })
    .insert(EnemyType::E1)
    .insert(Enemy { } )
    .insert(Name::new("EnemyT1_1"));
}

use bevy::prelude::*;
use rand::Rng;

use crate::{AssetsHolder, GameState};


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
            .add_system_set(SystemSet::on_enter(GameState::Gameplay).with_system(spawn_enemy))
            .add_system(enemy_movement_system);
    }
}

fn spawn_enemy(
    mut commands: Commands,
    game_assets: Res<AssetsHolder>,

)
{
    commands.spawn_bundle( SceneBundle{
        scene: game_assets.enemy_e1.clone(),
        transform: Transform::from_xyz(-4.0, 0.0, 0.0),
        ..default()
    })
    .insert(EnemyType::E1)
    .insert(Enemy { } )
    .insert(Name::new("EnemyT1_1"));
}

fn enemy_movement_system(mut enemies: Query<(&Enemy, &mut Transform)>, time: Res<Time>) {

    for (_enemy, mut transform) in &mut enemies {


        let x_ori = transform.translation.x;
        let z_ori = transform.translation.z;




        let mut rng = rand::thread_rng();

        let mut x  =  x_ori + rng.gen_range(-0.5..0.5)  ;
        let mut z = z_ori + rng.gen_range(-0.5..0.5) ;

        if !(-5_f32..=5_f32).contains(&x) {
             x = x_ori - rng.gen_range(0.0..0.5);
         }
        if !(-5_f32..=5_f32).contains(&z){
           z = z_ori - rng.gen_range(0.0..0.5);
        }

        transform.translation.x = x;
        transform.translation.z = z;




        // let mut translation = &transform.translation;
        // let mut rng = rand::thread_rng();
        //
        // let x  = rng.gen_range(0.0..10.0);
        // let y =  rng.gen_range(0.0..10.0);
        // let sig = rng.gen_range(-1..1);
        //
        //
        // trans.translation.x = trans.translation.x.clamp(-320.0, 320.0);



    }
}
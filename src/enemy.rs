use bevy::prelude::*;
use rand::Rng; // 0.8.5


use crate::*;


//TODO: More fancy names needed
#[derive(Component)]
pub enum EnemyType{
    E1,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Enemy{
    move_target_index: usize,
    visiting_nav: Option<usize>
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Enemy>()
            .add_system_set(SystemSet::on_enter(GameState::Gameplay).with_system(spawn_enemy))
            .add_system(enemy_movement_system)
            .add_system(enemy_reach_nav_point);
    }
}

fn spawn_enemy(
    mut commands: Commands,
    game_assets: Res<AssetsHolder>,
    nav_point: Query<(&NavPoint, &mut Transform)>,
)
{
    let target_navpt = nav_point
        .iter()
        .min_by_key(|nav|{
            nav.0.index
        });

    if let Some(_target_navpt) = target_navpt {

    commands.spawn( SceneBundle{
        scene: game_assets.enemy_e1.clone(),
        transform: Transform::from_xyz(-4.0, 0.0, 0.0),
        ..default()
    })


    .insert(EnemyType::E1)
    .insert(Enemy { move_target_index: 0, visiting_nav: None } )
    .insert(Name::new("EnemyT1_1"));
    }
}

fn enemy_movement_system(mut enemies: Query<(&Enemy, &mut Transform), Without<NavPoint>>
                        , _nav_point: Query<(&NavPoint, &mut Transform)>
                        , time: Res<Time>
                        , game: ResMut<Game>
                         ) {


    for (_enemy,  mut _enemy_transform) in &mut enemies {

        let target_move_vec =  game.nav_point_vecs[_enemy.move_target_index];
        let direction = - target_move_vec - _enemy_transform.translation ;
        _enemy_transform.translation += (direction).normalize() * 1.2 * time.delta_seconds();

    }
}

fn enemy_reach_nav_point(
    mut _commands: Commands,
    mut enemies: Query<(&mut Enemy, &mut Transform), Without<NavPoint>>,
    nav_point: Query<(&NavPoint, &mut Transform)>,
    game: ResMut<Game>
)
{
    for (nav_point, nav_point_transform) in &nav_point {
        for (mut _enemy, enemy_transform) in &mut enemies {


            game.nav_point_vecs.iter().enumerate().for_each(|(_ix, &_vec)| {
                if Vec3::distance(enemy_transform.translation, _vec) > 2. {

                }
            });


            if Vec3::distance(nav_point_transform.translation, enemy_transform.translation) < 0.2   {

                if _enemy.visiting_nav.is_some()
                {
                    if _enemy.visiting_nav.unwrap() != nav_point.index
                    {
                        _enemy.visiting_nav = Some(nav_point.index);


                        if game.nav_point_vecs.len() > _enemy.move_target_index + 1 {


                                match  rand::thread_rng().gen_range(0..=1) {
                                    0 => {
                                        if (_enemy.move_target_index as i32 - 1) as i32 >= 0{
                                            _enemy.move_target_index -= 1;
                                        }
                                        else {
                                            _enemy.move_target_index += 1;
                                        }
                                    },
                                    1 => {
                                        if  _enemy.move_target_index  + 1 < game.nav_point_vecs.len() {
                                            _enemy.move_target_index += 1;
                                        }
                                        else {
                                            _enemy.move_target_index -= 1;
                                        }
                                    },
                                    _ => {}
                                }

                        } else {
                            _enemy.move_target_index = 0;
                        }

                    }
                }
                else {
                    _enemy.visiting_nav = Some(nav_point.index + 1);

                }

            }

        }

        }
    }


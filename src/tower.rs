use bevy::prelude::*;
use bevy::utils::FloatOrd;
use bevy_inspector_egui::{Inspectable};
use crate::{AssetsHolder, Enemy, Lifetime, Missile};
use crate::GameState;


#[derive(Inspectable, Component, Clone, Copy, Debug)]
pub enum TowerType{
    T1,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Tower {
    pub shooting_interval: Timer,
    pub bullet_offset: Vec3,
}

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {

        app.register_type::<Tower>().add_startup_system(spawn_tower)

            .add_system_set(
            SystemSet::on_update(GameState::Gameplay)   .with_system(shooting_tower) ) ;
    }
}

fn spawn_tower(
    mut commands: Commands,
    game_assets: Res<AssetsHolder>,
)
{



    commands
        .spawn(SceneBundle  {
             scene: game_assets.tower_t1.clone(),
             transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        })
        .with_children(|parent| {

            parent.spawn(SceneBundle  {
                scene: game_assets.tower_base.clone(),
                transform: Transform::from_xyz(0.0, -1.0, 0.0),
                ..default()
            });


        })
        .with_children(|parent| {
            parent.spawn(SceneBundle {
                scene: game_assets.tower_bottom.clone(),
                transform: Transform::from_xyz(0.0, -0.8, 0.0),
                ..default()
            });
        })

        .insert(TowerType::T1)
        .insert(Tower {
            shooting_interval: Timer::from_seconds(0.3, TimerMode::Repeating),
            bullet_offset: Vec3::new(0.0, 0.2, 0.0)

        } )
        .insert(Name::new("Tower"));
}

fn shooting_tower(mut _commands: Commands,
                  mut towers: Query<(Entity, &mut Tower, &TowerType, &GlobalTransform)>,
                  targets: Query<&GlobalTransform, With<Enemy>>,
                  mut meshes: ResMut<Assets<Mesh>>,
                  mut materials: ResMut<Assets<StandardMaterial>>,
                  time: Res<Time>,)
{
    for (_tower_ent, mut tower, _tower_type, transform) in &mut towers {
        tower.shooting_interval.tick(time.delta());

        if ! tower.shooting_interval.just_finished()
        {
            continue
        }

        let bullet_spawn = transform.translation() + tower.bullet_offset;

        let _direction = targets
            .iter()
            .min_by_key(|target_transform| {
                FloatOrd(Vec3::distance(target_transform.translation(), bullet_spawn))
            })
            .map(|closest_target| closest_target.translation() - bullet_spawn);

            if let Some(direction) = _direction {

                _commands.entity(_tower_ent).with_children(|commands| {
                commands
                    .spawn(PbrBundle  {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
                        material: materials.add(Color::rgb(1.0, 1.0, 0.0).into()),
                        transform: Transform::from_translation(tower.bullet_offset),
                        ..Default::default()
                    })
                    .insert(Lifetime {
                        timer: Timer::from_seconds(5.0, TimerMode::Once),
                    })
                    .insert(Missile {
                        direction,
                        speed: 9.5,
                    })
                    .insert(Name::new("Bullet"));
            });
        }



    }
}
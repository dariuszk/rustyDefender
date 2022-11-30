use bevy::prelude::*;
use crate::GameState;

pub struct MissilePlugin;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Missile {
    pub speed: f32,
    pub direction: Vec3

}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Lifetime {
    pub timer: Timer,
}


impl Plugin for MissilePlugin {
    fn build(&self, app: &mut App) {

        app.register_type::<Missile>()
            .register_type::<Lifetime>()
            .add_system_set(
            SystemSet::on_update(GameState::Gameplay)
                                .with_system(move_missile) );
    }
}


fn move_missile(mut missiles: Query<(&Missile, &mut Transform)>, time: Res<Time>) {
    for (missile, mut transform) in &mut missiles {

        transform.translation += missile.direction.normalize() * missile.speed * time.delta_seconds();
    }
}

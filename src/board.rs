use bevy::prelude::*;
use crate::Game;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Board {
    size: f32,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct NavPoint {
    point: Vec3,
    pub index: usize

}

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Board>()
            .register_type::<NavPoint>()
            .add_startup_system(spawn_board)
            .add_startup_system(spawn_nav_points);
    }
}

fn spawn_nav_points(    mut commands: Commands,
                        mut meshes: ResMut<Assets<Mesh>>,
                        mut materials: ResMut<Assets<StandardMaterial>>,
                        game: ResMut<Game>)
{
    const COLOR: Color = Color::rgb(0.8, 0.0, 0.0);



    game.nav_point_vecs.iter().enumerate().for_each(|(ix, &_vec)| {

        commands
            .spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
                material: materials.add(COLOR.into()),
                transform: Transform {
                    translation: _vec.clone(),
                    ..Default::default()
                },
                ..Default::default()
            }).insert(NavPoint{
            point: _vec.clone(),
            index:ix
        })

            .insert(Name::new( format!("NavPoint_{ix}")));
    });

}



fn spawn_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
)
{
    const PLANE_COLOR: Color = Color::rgb(0.6, 0.0, 0.75);
    const PLANE_SIZE: f32 = 10_f32;

    let board = Board { size: PLANE_SIZE };
    let _physical_board_size = board.size * 40.;


    // Setup plane
    commands
        .spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: PLANE_SIZE })),
        material: materials.add(PLANE_COLOR.into()),
            ..Default::default()

    });


}
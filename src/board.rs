use bevy::prelude::*;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
struct Board {
    size: f32,
}

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Board>()
            .add_startup_system(spawn_board);
    }
}


fn spawn_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
)
{
    const PLANE_COLOR: Color = Color::rgb(0.6, 0.0, 0.75);
    const PLANE_SIZE: f32 = 10 as f32; ;

    let board = Board { size: PLANE_SIZE };
    let physical_board_size = f32::from(board.size as f32) * 40.;


    // Setup plane
    commands
        .spawn_bundle(PbrBundle {


        mesh: meshes.add(Mesh::from(shape::Plane { size: PLANE_SIZE })),
        material: materials.add(PLANE_COLOR.into()),
            ..Default::default()

    });


}
use bevy::prelude::*;

pub const HEIGHT: f32 = 800.;
pub const WIDTH: f32 = 1920.;
pub const GAME_NAME: &str =  "Rusty Defender" ;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(WindowDescriptor {
            width: WIDTH,
            height: HEIGHT,
            title: GAME_NAME.to_string(),
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .run();
}
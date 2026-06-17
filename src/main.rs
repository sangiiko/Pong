use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins).add_systems(Startup,setup).run();
}

fn setup(mut commands: Commands){
   commands.spawn(Camera2d);

   commands.spawn(Sprite {
       custom_size: Some(Vec2::new(100.0,100.0)),
       color: Color::srgb(1.0,1.0,1.0),
       ..default()
   });
}

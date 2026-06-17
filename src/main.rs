use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins).add_systems(Startup,setup).add_systems(Update,move_player).run();
}

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands){
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::new(100.0,100.0)),
            color: Color::srgb(1.0,1.0,1.0),
            ..default()
        },
        Player,
    ));

}

fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
    ){
        let mut direction = Vec3::ZERO;
       
        if input.pressed(KeyCode::KeyW){
            direction.y += 1.0;
        }
        if input.pressed(KeyCode::KeyS){
            direction.y -= 1.0;
        }
        if input.pressed(KeyCode::KeyA){
            direction.x -= 1.0;
        }
        if input.pressed(KeyCode::KeyD){
            direction.x += 1.0;
        }

        let speed = 300.0;

        for mut transform in &mut query{
            transform.translation += direction*speed*time.delta_secs();
        }
    }

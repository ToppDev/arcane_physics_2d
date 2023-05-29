use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

const PLAYER_LINEAR_SPEED: f32 = 10.0; // [m/s]
const PLAYER_ROTATION_SPEED: f32 = 80.0; // [deg/s]

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    mut last_time: Local<f32>,
    time: Res<Time>,
) {
    let dt = time.raw_elapsed_seconds() - *last_time;
    *last_time = time.raw_elapsed_seconds();

    for mut player_transform in &mut query {
        let dir_x = match (
            keyboard_input.pressed(KeyCode::L) || keyboard_input.pressed(KeyCode::D),
            keyboard_input.pressed(KeyCode::H) || keyboard_input.pressed(KeyCode::A),
        ) {
            (true, false) => 1.0,
            (false, true) => -1.0,
            _ => 0.0,
        };
        let dir_y = match (
            keyboard_input.pressed(KeyCode::K) || keyboard_input.pressed(KeyCode::W),
            keyboard_input.pressed(KeyCode::J) || keyboard_input.pressed(KeyCode::S),
        ) {
            (true, false) => 1.0,
            (false, true) => -1.0,
            _ => 0.0,
        };
        player_transform.translation += dt * PLAYER_LINEAR_SPEED * Vec3::new(dir_x, dir_y, 0.0);

        let rot_dir = match (
            keyboard_input.pressed(KeyCode::Q),
            keyboard_input.pressed(KeyCode::E),
        ) {
            (true, false) => 1.0,
            (false, true) => -1.0,
            _ => 0.0,
        };
        player_transform.rotation *=
            Quat::from_rotation_z(dt * PLAYER_ROTATION_SPEED.to_radians() * rot_dir)
        // *objects.first_mut().unwrap().rotation_velocity_mut() = PLAYER_ROTATION_SPEED * rot_dir;
    }
}

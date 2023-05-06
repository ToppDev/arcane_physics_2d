use ::rand::{thread_rng, Rng};
use body::{Body, BodyType, Drawable, Movable, Position};
use macroquad::prelude::*;
use math::Vec2f;

mod body;
mod draw;
mod math;

pub type GameResult = std::result::Result<(), GameError>;
#[derive(Debug)]
pub enum GameError {
    FileError(macroquad::file::FileError),
}
impl From<macroquad::file::FileError> for GameError {
    fn from(error: macroquad::file::FileError) -> GameError {
        GameError::FileError(error)
    }
}

pub const SHAPE_BORDER_WIDTH: f32 = 0.2; // [m]

const PLAYER_SPEED: f32 = 10.0; // [m/s]
const COLORS: [Color; 20] = [
    BEIGE, BLUE, BROWN, DARKBLUE, DARKBROWN, DARKGRAY, DARKGREEN, DARKPURPLE, GOLD, GRAY, GREEN,
    LIGHTGRAY, LIME, MAGENTA, MAROON, PINK, PURPLE, SKYBLUE, VIOLET, YELLOW,
];
const SPAWN_SIZE: (f32, f32) = (70.0, 40.0);
const OBJECT_RADIUS: f32 = 1.0; // [m]

const MIN_BODY_SIZE: f32 = 0.01 * 0.01; // [m^2]
const MAX_BODY_SIZE: f32 = 64.0 * 64.0; // [m^2]

const MIN_DENSITY: f32 = 0.2; // [g/cm^3]
const MAX_DENSITY: f32 = 21.4; // [g/cm^3] (density of platinum)

fn spawn_shapes(objects: &mut Vec<Body>, player: &Body) {
    let mut rng = thread_rng();
    for color in COLORS {
        let pos = loop {
            let pos = Vec2f::new(
                rng.gen_range(-SPAWN_SIZE.0 / 2.0..=SPAWN_SIZE.0 / 2.0),
                rng.gen_range(-SPAWN_SIZE.1 / 2.0..=SPAWN_SIZE.1 / 2.0),
            );
            if (player.position() - pos).norm() < 4.0 * OBJECT_RADIUS {
                continue;
            }

            if objects
                .iter()
                .all(|obj| (obj.position() - pos).norm() > 3.0 * OBJECT_RADIUS)
            {
                break pos;
            }
        };

        objects.push(
            Body::new_circle(
                pos,
                OBJECT_RADIUS,
                color,
                Some(WHITE),
                1.0,
                0.0,
                BodyType::Dynamic,
            )
            .unwrap(),
        );
    }
}

pub async fn entry_point() -> GameResult {
    let mut player = Body::new_circle(
        Vec2f::new(0.0, 0.0),
        OBJECT_RADIUS,
        ORANGE,
        Some(WHITE),
        1.0,
        0.0,
        BodyType::Dynamic,
    )
    .unwrap();

    let mut objects: Vec<Body> = Vec::new();
    spawn_shapes(&mut objects, &player);

    let mut zoom = 1.0 / 40.0;
    let mut target = (0., 0.);
    let mut last_update = get_time();

    loop {
        let dt = (get_time() - last_update) as f32;
        last_update = get_time();

        // Camera movement ########################################################################
        match mouse_wheel() {
            (_x, y) if y != 0.0 => {
                zoom *= 1.1f32.powf(y);
            }
            _ => (),
        }
        match (is_key_down(KeyCode::W), is_key_down(KeyCode::S)) {
            (true, false) => target.1 -= 10.0,
            (false, true) => target.1 += 10.0,
            _ => (),
        }
        match (is_key_down(KeyCode::A), is_key_down(KeyCode::D)) {
            (true, false) => target.0 += 10.0,
            (false, true) => target.0 -= 10.0,
            _ => (),
        }
        set_camera(&Camera2D {
            target: vec2(target.0, target.1),
            zoom: macroquad::prelude::vec2(zoom, zoom * screen_width() / screen_height()),
            ..Default::default()
        });

        // Player movement ########################################################################
        let dir_x = match (
            is_key_down(KeyCode::L) || is_key_down(KeyCode::Right),
            is_key_down(KeyCode::H) || is_key_down(KeyCode::Left),
        ) {
            (true, false) => 1.0,
            (false, true) => -1.0,
            _ => 0.0,
        };
        let dir_y = match (
            is_key_down(KeyCode::K) || is_key_down(KeyCode::Up),
            is_key_down(KeyCode::J) || is_key_down(KeyCode::Down),
        ) {
            (true, false) => 1.0,
            (false, true) => -1.0,
            _ => 0.0,
        };
        let dir = Vec2f::new(dir_x, dir_y)
            .try_normalize(0.1)
            .unwrap_or(Vec2f::zeros());
        player.offset(PLAYER_SPEED * dt * dir);
        if let Body::Dynamic(player) = &mut player {
            *player.linear_velocity_mut() = PLAYER_SPEED * dir * dt;
        }

        // Drawing ################################################################################
        clear_background(BLACK);
        // draw_rectangle(
        //     -SPAWN_SIZE.0 / 2.0,
        //     -SPAWN_SIZE.1 / 2.0,
        //     SPAWN_SIZE.0,
        //     SPAWN_SIZE.1,
        //     GRAY,
        // );
        player.draw();
        for object in &objects {
            object.draw();
        }

        next_frame().await
    }
}

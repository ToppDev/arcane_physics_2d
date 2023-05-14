use ::rand::{thread_rng, Rng};
use body::{
    circle::Circle,
    components::{Movable, Positionable, Rotatable},
    polygon::Polygon,
    Drawable, Dynamic, Updatable,
};
use draw::draw_rectangle;
use macroquad::{
    color::colors::*,
    prelude::{is_key_down, set_camera, vec2, Camera2D, Color, KeyCode},
    time::get_time,
    window::{clear_background, next_frame, screen_height, screen_width},
};
use math::Vec2f;

use crate::body::components::BodyColor;

mod body;
mod draw;
mod math;
mod physics;
pub mod world;

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

trait GameObject: Drawable + Positionable + Movable + Rotatable + Updatable {}
impl<T> GameObject for T where T: Drawable + Positionable + Movable + Rotatable + Updatable {}

const DEBUG_DRAW_SPAWN_AREA: bool = false;

const INITAL_ZOOM: f32 = 1.0 / 30.0;
const OBJECT_SIZE: f32 = 2.0; // [m]
const SHAPE_BORDER_WIDTH: f32 = 0.15; // [m]
const SPAWN_SIZE: (f32, f32) = (50.0, 30.0);

const PLAYER_LINEAR_SPEED: f32 = 10.0; // [m/s]
const PLAYER_ROTATION_SPEED: f32 = 80.0; // [deg/s]

fn spawn_shapes(objects: &mut Vec<Box<dyn GameObject>>) {
    const COLORS: [Color; 20] = [
        BEIGE, BLUE, BROWN, DARKBLUE, DARKBROWN, DARKGRAY, DARKGREEN, DARKPURPLE, GOLD, GRAY,
        GREEN, LIGHTGRAY, LIME, MAGENTA, MAROON, PINK, PURPLE, SKYBLUE, VIOLET, YELLOW,
    ];

    let mut rng = thread_rng();
    for color in COLORS {
        let pos = loop {
            let pos = Vec2f::new(
                rng.gen_range(-SPAWN_SIZE.0 / 2.0..=SPAWN_SIZE.0 / 2.0),
                rng.gen_range(-SPAWN_SIZE.1 / 2.0..=SPAWN_SIZE.1 / 2.0),
            );
            let player = objects.first().unwrap();
            if (player.position() - pos).norm() < 3.0 * OBJECT_SIZE {
                continue;
            }

            if objects
                .iter()
                .all(|obj| (obj.position() - pos).norm() > 2.0 * OBJECT_SIZE)
            {
                break pos;
            }
        };

        objects.push(Box::new(
            Circle::<Dynamic>::new(
                pos,
                OBJECT_SIZE / 2.0,
                BodyColor {
                    fill: color,
                    hitbox: Some(WHITE),
                },
                2.0,
                0.5,
            )
            .unwrap(),
        ));
        objects.push(if rng.gen_bool(0.7) {
            Box::new(
                Circle::<Dynamic>::new(
                    pos,
                    OBJECT_SIZE / 2.0,
                    BodyColor {
                        fill: color,
                        hitbox: Some(WHITE),
                    },
                    2.0,
                    0.5,
                )
                .unwrap(),
            )
        } else {
            Box::new(
                Polygon::<Dynamic>::new_rect(
                    pos,
                    OBJECT_SIZE * rng.gen_range(1.0..=1.0),
                    OBJECT_SIZE * rng.gen_range(1.0..=1.0),
                    if rng.gen_bool(1.0) {
                        0.0
                    } else {
                        rng.gen_range(0.0..=90.0)
                    },
                    BodyColor {
                        fill: color,
                        hitbox: Some(WHITE),
                    },
                    2.0,
                    0.5,
                )
                .unwrap(),
            )
        });
    }
}

pub async fn entry_point() -> GameResult {
    let mut objects: Vec<Box<dyn GameObject>> = Vec::new();
    // objects.push(Box::new(
    //     Circle::<Dynamic>::new(
    //         Vec2f::new(0.0, 0.0),
    //         OBJECT_SIZE / 2.0,
    //         BodyColor {
    //             fill: ORANGE,
    //             hitbox: Some(WHITE),
    //         },
    //         2.0,
    //         0.5,
    //     )
    //     .unwrap(),
    // ));
    objects.push(Box::new(
        Polygon::<Dynamic>::new_rect(
            Vec2f::new(0.0, 0.0),
            OBJECT_SIZE,
            OBJECT_SIZE,
            0.0,
            BodyColor {
                fill: ORANGE,
                hitbox: Some(WHITE),
            },
            2.0,
            0.5,
        )
        .unwrap(),
    ));

    let arrow_position = Vec2f::new(-6.0, -4.0);
    objects.push(
        // (10,  -2) _____(10.5, -2)
        //          |     \
        //          |      \ (11, -2.5)
        //          |      /
        //          |_____/
        // (10, -3)      (10.5, -3)
        Box::new(
            Polygon::<Dynamic>::new(
                &[
                    Vec2f::new(arrow_position.x, arrow_position.y),
                    Vec2f::new(arrow_position.x + OBJECT_SIZE / 2.0, arrow_position.y),
                    Vec2f::new(
                        arrow_position.x + OBJECT_SIZE,
                        arrow_position.y - OBJECT_SIZE / 2.0,
                    ),
                    Vec2f::new(
                        arrow_position.x + OBJECT_SIZE / 2.0,
                        arrow_position.y - OBJECT_SIZE,
                    ),
                    Vec2f::new(arrow_position.x, arrow_position.y - OBJECT_SIZE),
                ],
                BodyColor {
                    fill: BLUE,
                    hitbox: Some(WHITE),
                },
                1.0,
                0.0,
            )
            .unwrap(),
        ),
    );
    spawn_shapes(&mut objects);

    let zoom = INITAL_ZOOM;
    let target = (0.0, 0.0);
    let mut last_update = get_time();

    loop {
        let dt = (get_time() - last_update) as f32;
        last_update = get_time();

        // Camera movement ########################################################################
        // match mouse_wheel() {
        //     (_x, y) if y != 0.0 => {
        //         zoom *= 1.1f32.powf(y);
        //     }
        //     _ => (),
        // }
        // match (is_key_down(KeyCode::Up), is_key_down(KeyCode::Down)) {
        //     (true, false) => target.1 += 1.0,
        //     (false, true) => target.1 -= 1.0,
        //     _ => (),
        // }
        // match (is_key_down(KeyCode::Right), is_key_down(KeyCode::Left)) {
        //     (true, false) => target.0 += 1.0,
        //     (false, true) => target.0 -= 1.0,
        //     _ => (),
        // }
        set_camera(&Camera2D {
            target: vec2(target.0, target.1) * 0.01 / zoom,
            zoom: macroquad::prelude::vec2(zoom, zoom * screen_width() / screen_height()),
            ..Default::default()
        });

        // Player movement ########################################################################
        let dir_x = match (
            is_key_down(KeyCode::L) || is_key_down(KeyCode::D),
            is_key_down(KeyCode::H) || is_key_down(KeyCode::A),
        ) {
            (true, false) => 1.0,
            (false, true) => -1.0,
            _ => 0.0,
        };
        let dir_y = match (
            is_key_down(KeyCode::K) || is_key_down(KeyCode::W),
            is_key_down(KeyCode::J) || is_key_down(KeyCode::S),
        ) {
            (true, false) => 1.0,
            (false, true) => -1.0,
            _ => 0.0,
        };
        let dir = Vec2f::new(dir_x, dir_y)
            .try_normalize(0.1)
            .unwrap_or(Vec2f::zeros());
        *objects.first_mut().unwrap().linear_velocity_mut() = PLAYER_LINEAR_SPEED * dir;

        let rot_dir = match (is_key_down(KeyCode::Q), is_key_down(KeyCode::E)) {
            (true, false) => 1.0,
            (false, true) => -1.0,
            _ => 0.0,
        };
        *objects.first_mut().unwrap().rotation_velocity_mut() =
            PLAYER_ROTATION_SPEED.to_radians() * rot_dir;

        // Update #################################################################################
        for object in &mut objects {
            object.update(dt);
        }

        // Drawing ################################################################################
        clear_background(BLACK);
        if DEBUG_DRAW_SPAWN_AREA {
            draw_rectangle(
                -SPAWN_SIZE.0 / 2.0,
                -SPAWN_SIZE.1 / 2.0,
                SPAWN_SIZE.0,
                SPAWN_SIZE.1,
                GRAY,
            );
        }
        for object in &objects {
            object.draw();
        }

        next_frame().await
    }
}

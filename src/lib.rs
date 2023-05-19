use std::{thread, time};

use ::rand::{thread_rng, Rng};
use body::{
    circle::Circle,
    components::{Colored, Movable, Positionable, Rotatable},
    polygon::Polygon,
    Body, RigidBodyType,
};
use collision::CollisionWith;
use itertools::Itertools;
use macroquad::{color::colors::*, prelude::*};

use crate::body::components::BodyColor;

mod body;
mod collision;
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

const DEBUG_DRAW_SPAWN_AREA: bool = false;

const INITAL_ZOOM: f32 = 1.0 / 30.0;
const OBJECT_SIZE: f32 = 2.0; // [m]
const SHAPE_BORDER_WIDTH: f32 = 0.15; // [m]
const SPAWN_SIZE: (f32, f32) = (50.0, 30.0);

const PLAYER_LINEAR_SPEED: f32 = 10.0; // [m/s]
const PLAYER_ROTATION_SPEED: f32 = 80.0; // [deg/s]

fn spawn_shapes(objects: &mut Vec<Body>) {
    const COLORS: [Color; 20] = [
        BEIGE, BLUE, BROWN, DARKBLUE, DARKBROWN, DARKGRAY, DARKGREEN, DARKPURPLE, GOLD, GRAY,
        GREEN, LIGHTGRAY, LIME, MAGENTA, MAROON, PINK, PURPLE, SKYBLUE, VIOLET, YELLOW,
    ];

    let mut rng = thread_rng();
    for _ in 0..=3 {
        for color in COLORS {
            let mut pos_i = 0;
            let mut min_dist = 2.0 * OBJECT_SIZE;
            let pos = loop {
                pos_i += 1;
                if pos_i > 1000 {
                    min_dist /= 1.9;
                }
                let pos = Vec2::new(
                    rng.gen_range(-SPAWN_SIZE.0 / 2.0..=SPAWN_SIZE.0 / 2.0),
                    rng.gen_range(-SPAWN_SIZE.1 / 2.0..=SPAWN_SIZE.1 / 2.0),
                );

                if objects
                    .iter()
                    .all(|obj| (obj.position() - pos).length() > min_dist)
                {
                    break pos;
                }
            };

            objects.push(if rng.gen_bool(0.5) {
                Body::new_circle(
                    pos,
                    OBJECT_SIZE / 2.0,
                    BodyColor {
                        fill: color,
                        hitbox: Some(WHITE),
                    },
                    RigidBodyType::Dynamic,
                    2.0,
                    0.5,
                )
                .unwrap()
            } else {
                Body::new_rect(
                    pos,
                    OBJECT_SIZE * rng.gen_range(1.0..=1.0),
                    OBJECT_SIZE * rng.gen_range(1.0..=1.0),
                    if rng.gen_bool(0.8) {
                        0.0
                    } else {
                        rng.gen_range(0.0..=90.0)
                    },
                    BodyColor {
                        fill: color,
                        hitbox: Some(WHITE),
                    },
                    RigidBodyType::Dynamic,
                    2.0,
                    0.5,
                )
                .unwrap()
            });
        }
    }
}

pub async fn entry_point() -> GameResult {
    let mut objects: Vec<Body> = Vec::new();
    objects.push(
        Body::new_circle(
            Vec2::new(0.0, 0.0),
            OBJECT_SIZE / 2.0,
            BodyColor {
                fill: ORANGE,
                hitbox: Some(WHITE),
            },
            RigidBodyType::Dynamic,
            2.0,
            0.5,
        )
        .unwrap(),
    );
    // objects.push(
    //     Body::new_rect(
    //         Vec2::new(0.0, 0.0),
    //         OBJECT_SIZE,
    //         OBJECT_SIZE,
    //         0.0,
    //         BodyColor {
    //             fill: ORANGE,
    //             hitbox: Some(WHITE),
    //         },
    //         RigidBodyType::Dynamic,
    //         2.0,
    //         0.5,
    //     )
    //     .unwrap(),
    // );

    let arrow_position = Vec2::new(-6.0, -4.0);
    objects.push(
        //     [0]             [1]
        // (10,  -2) _____(10.5, -2)
        //          |     \
        //          |      \ (11, -2.5) [2]
        //          |      /
        //     [4]  |_____/    [3]
        // (10, -3)      (10.5, -3)
        Body::new_polygon(
            &[
                Vec2::new(arrow_position.x, arrow_position.y),
                Vec2::new(arrow_position.x + OBJECT_SIZE / 2.0, arrow_position.y),
                Vec2::new(
                    arrow_position.x + OBJECT_SIZE,
                    arrow_position.y - OBJECT_SIZE / 2.0,
                ),
                Vec2::new(
                    arrow_position.x + OBJECT_SIZE / 2.0,
                    arrow_position.y - OBJECT_SIZE,
                ),
                Vec2::new(arrow_position.x, arrow_position.y - OBJECT_SIZE),
            ],
            BodyColor {
                fill: BLUE,
                hitbox: Some(WHITE),
            },
            RigidBodyType::Dynamic,
            1.0,
            0.0,
        )
        .unwrap(),
    );
    spawn_shapes(&mut objects);

    let zoom = INITAL_ZOOM;
    let target = (0.0, 0.0);
    let mut last_update = get_time();

    loop {
        let update_time = get_time();
        let dt = (update_time - last_update) as f32;

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
        let dir = Vec2::new(dir_x, dir_y).normalize_or_zero();

        *objects.first_mut().unwrap().linear_velocity_mut() = PLAYER_LINEAR_SPEED * dir;

        let rot_dir = match (is_key_down(KeyCode::Q), is_key_down(KeyCode::E)) {
            (true, false) => 1.0,
            (false, true) => -1.0,
            _ => 0.0,
        };
        *objects.first_mut().unwrap().rotation_velocity_mut() = PLAYER_ROTATION_SPEED * rot_dir;

        // Background #############################################################################
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

        // Update #################################################################################
        for object in &mut objects {
            object.update(dt);
        }

        for object in &mut objects {
            object.change_hitbox_color(WHITE);
        }
        for p in (0..objects.len()).combinations(2) {
            let (i, j) = (p[0], p[1]);
            if let Some(collision) = objects[i].collides(&objects[j]) {
                objects[i].change_hitbox_color(RED);
                objects[j].change_hitbox_color(RED);

                match (objects[i].body_type(), objects[j].body_type()) {
                    (RigidBodyType::Dynamic, RigidBodyType::Dynamic) => {
                        objects[i].offset(-collision.normal * collision.depth / 2.0);
                        objects[j].offset(collision.normal * collision.depth / 2.0);
                    }
                    (RigidBodyType::Dynamic, RigidBodyType::Fixed) => {
                        objects[i].offset(-collision.normal * collision.depth);
                    }
                    (RigidBodyType::Fixed, RigidBodyType::Dynamic) => {
                        objects[j].offset(collision.normal * collision.depth);
                    }
                    (_, _) => (),
                }
            }
        }

        // Drawing ################################################################################

        for object in &objects {
            object.draw();
        }

        set_default_camera();
        let fps = (1.0 / dt) as i32;
        draw_text(format!("{fps} fps").as_str(), 10.0, 25.0, 20.0, WHITE);

        // End of Frame ###########################################################################
        const FRAME_LIMIT: f32 = 60.0;
        let wait_time = 1.0 / FRAME_LIMIT - dt;
        if wait_time > 0.0 {
            thread::sleep(time::Duration::from_secs_f32(wait_time));
        }

        last_update = update_time;
        next_frame().await
    }
}

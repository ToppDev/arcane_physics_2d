use itertools::Itertools;
use macroquad::prelude::*;

use crate::body::{circle::Circle, components::Positionable, polygon::Polygon};

const DEBUG_DRAW_COLLISION: bool = true;
const DEBUG_DRAW_COLLISION_CIRCLE_CIRCLE: bool = false;
const DEBUG_DRAW_COLLISION_POLYGON_POLYGON: bool = false;
const DEBUG_DRAW_COLLISION_CIRCLE_POLYGON: bool = false;

pub struct CollisionResponse {
    pub normal: Vec2,
    pub depth: f32,
}

pub trait CollisionWith<T> {
    fn collides(&self, other: &T) -> Option<CollisionResponse>;
}

impl CollisionWith<Circle> for Circle {
    fn collides(&self, other: &Circle) -> Option<CollisionResponse> {
        let normal = other.position() - self.position();
        let distance = normal.length();
        let radii = self.radius() + other.radius();

        if DEBUG_DRAW_COLLISION && DEBUG_DRAW_COLLISION_CIRCLE_CIRCLE {
            let end = self.position() + (1.0 + self.radius()) * normal.normalize();
            draw_line(
                self.position().x,
                self.position().y,
                end.x,
                end.y,
                0.05,
                GRAY,
            );
            let end = other.position() + (1.0 + other.radius()) * -normal.normalize();
            draw_line(
                other.position().x,
                other.position().y,
                end.x,
                end.y,
                0.05,
                GRAY,
            );
        }

        if distance >= radii {
            return None;
        }

        Some(CollisionResponse {
            normal: normal.normalize(),
            depth: radii - distance,
        })
    }
}

impl CollisionWith<Polygon> for Polygon {
    fn collides(&self, other: &Polygon) -> Option<CollisionResponse> {
        let self_vertices: Vec<Vec2> = self
            .vertices()
            .iter()
            .map(|&v| self.position() + v)
            .collect();
        let other_vertices: Vec<Vec2> = other
            .vertices()
            .iter()
            .map(|&v| other.position() + v)
            .collect();

        let normals: Vec<(Vec2, Vec2)> = self_vertices
            .iter()
            .circular_tuple_windows() // Get all sides (wrap last point and first)
            .chain(other_vertices.iter().circular_tuple_windows()) // Do also for other shape
            .map(|(&p1, &p2)| (p1 + (p2 - p1) / 2.0, (p1 - p2).yx() * Vec2::new(1.0, -1.0))) // Calculate normal
            .unique_by(|(_, normal)| {
                let x_axis = Vec2::X;
                let det = normal.x * x_axis.y - normal.y * x_axis.x;
                let mut angle = normal.dot(Vec2::X).atan2(det).to_degrees();
                if angle < 0.0 {
                    angle += 360.0;
                }
                let angle = (angle * 100.0) as i32 % 18000;
                angle
            })
            .collect();

        if DEBUG_DRAW_COLLISION && DEBUG_DRAW_COLLISION_POLYGON_POLYGON {
            for (start, normal) in &normals {
                let end = *start + normal.normalize();
                draw_line(start.x, start.y, end.x, end.y, 0.05, GRAY);
            }
        }

        let mut response_depth = f32::MAX;
        let mut response_normal = Vec2::ZERO;

        for (start, normal) in &normals {
            // Separating Axis Theorem (SAT)
            let (self_min, self_max) =
                match self_vertices.iter().map(|vert| vert.dot(*normal)).minmax() {
                    itertools::MinMaxResult::MinMax(min, max) => (min, max),
                    _ => panic!("We cannot have single point polygons"),
                };

            let (other_min, other_max) =
                match other_vertices.iter().map(|vert| vert.dot(*normal)).minmax() {
                    itertools::MinMaxResult::MinMax(min, max) => (min, max),
                    _ => panic!("We cannot have single point polygons"),
                };

            let gap = self_min >= other_max || other_min >= self_max;

            if DEBUG_DRAW_COLLISION && DEBUG_DRAW_COLLISION_CIRCLE_POLYGON {
                let end = *start + normal.normalize();
                draw_line(
                    start.x,
                    start.y,
                    end.x,
                    end.y,
                    0.05,
                    if gap { GREEN } else { RED },
                );
            }
            if gap {
                return None;
            }

            let axis_depth = (other_max - self_min).min(self_max - other_min);
            if axis_depth < response_depth {
                response_depth = axis_depth;
                response_normal = *normal;
            }
        }

        // Normals where not normalized, so we need to transform the response depth and normal
        response_depth /= response_normal.length();
        response_normal = response_normal.normalize();

        // Normal is not always pointing in direction 'self to other'
        let direction = other.position() - self.position();
        if direction.dot(response_normal) < 0.0 {
            // Pointing in opposite directions
            response_normal = -response_normal;
        }

        Some(CollisionResponse {
            normal: response_normal,
            depth: response_depth,
        })
    }
}

impl CollisionWith<Polygon> for Circle {
    fn collides(&self, other: &Polygon) -> Option<CollisionResponse> {
        let other_vertices: Vec<Vec2> = other
            .vertices()
            .iter()
            .map(|&v| other.position() + v)
            .collect();

        let mut normals: Vec<(Vec2, Vec2)> = other_vertices
            .iter()
            .circular_tuple_windows() // Get all sides (wrap last point and first)
            .map(|(&p1, &p2)| {
                (
                    p1 + (p2 - p1) / 2.0,
                    (p1 - p2).yx().normalize() * Vec2::new(1.0, -1.0),
                )
            }) // Calculate normal
            .unique_by(|(_, normal)| {
                let x_axis = Vec2::X;
                let det = normal.x * x_axis.y - normal.y * x_axis.x;
                let mut angle = normal.dot(Vec2::X).atan2(det).to_degrees();
                if angle < 0.0 {
                    angle += 360.0;
                }
                let angle = (angle * 100.0) as i32 % 18000;
                angle
            })
            .collect();
        // Connection from circle center to closest edge of polygon
        normals.push((
            self.position(),
            other_vertices
                .iter()
                .map(|&vert| vert - self.position())
                .min_by(|x, y| x.length().total_cmp(&y.length()))
                .unwrap()
                .normalize(),
        ));

        if DEBUG_DRAW_COLLISION && DEBUG_DRAW_COLLISION_CIRCLE_POLYGON {
            for (start, normal) in &normals {
                let end = *start + (1.0 + self.radius()) * normal.normalize();
                draw_line(start.x, start.y, end.x, end.y, 0.05, GRAY);
            }
        }

        let mut response_depth = f32::MAX;
        let mut response_normal = Vec2::ZERO;

        for (start, normal) in &normals {
            // Separating Axis Theorem (SAT)
            let circle_proj = self.position().dot(*normal);
            let self_min = circle_proj - self.radius();
            let self_max = circle_proj + self.radius();

            let (other_min, other_max) =
                match other_vertices.iter().map(|vert| vert.dot(*normal)).minmax() {
                    itertools::MinMaxResult::MinMax(min, max) => (min, max),
                    _ => panic!("We cannot have single point polygons"),
                };

            let gap = self_min >= other_max || other_min >= self_max;

            if DEBUG_DRAW_COLLISION && DEBUG_DRAW_COLLISION_CIRCLE_POLYGON {
                let end = *start + normal.normalize();
                draw_line(
                    start.x,
                    start.y,
                    end.x,
                    end.y,
                    0.05,
                    if gap { GREEN } else { RED },
                );
            }
            if gap {
                return None;
            }

            let axis_depth = (other_max - self_min).min(self_max - other_min);
            if axis_depth < response_depth {
                response_depth = axis_depth;
                response_normal = *normal;
            }
        }

        // Normal is not always pointing in direction 'self to other'
        let direction = other.position() - self.position();
        if direction.dot(response_normal) < 0.0 {
            // Pointing in opposite directions
            response_normal *= -1.0;
        }

        Some(CollisionResponse {
            normal: response_normal,
            depth: response_depth,
        })
    }
}

impl CollisionWith<Circle> for Polygon {
    fn collides(&self, other: &Circle) -> Option<CollisionResponse> {
        let mut collision_response = other.collides(self);
        if let Some(mut response) = collision_response {
            response.normal *= -1.0;
            collision_response = Some(response);
        }
        collision_response
    }
}

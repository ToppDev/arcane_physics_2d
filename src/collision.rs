use itertools::Itertools;

use crate::{
    body::{circle::Circle, components::Positionable, polygon::Polygon, BodyType},
    draw::*,
    math::Vec2f,
    DEBUG_DRAW_COLLISION,
};

pub struct CollisionResponse {
    pub normal: Vec2f,
    pub depth: f32,
}

pub trait CollisionWith<T> {
    fn collides(&self, other: &T) -> Option<CollisionResponse>;
}

impl<T: BodyType, U: BodyType> CollisionWith<Circle<U>> for Circle<T> {
    fn collides(&self, other: &Circle<U>) -> Option<CollisionResponse> {
        let normal = other.position() - self.position();
        let distance = normal.norm();
        let radii = self.radius() + other.radius();

        if distance >= radii {
            return None;
        }

        Some(CollisionResponse {
            normal: normal.normalize(),
            depth: radii - distance,
        })
    }
}

impl<T: BodyType, U: BodyType> CollisionWith<Polygon<U>> for Polygon<T> {
    fn collides(&self, other: &Polygon<U>) -> Option<CollisionResponse> {
        let self_vertices: Vec<Vec2f> = self
            .vertices()
            .iter()
            .map(|v| self.position() + v)
            .collect();
        let other_vertices: Vec<Vec2f> = other
            .vertices()
            .iter()
            .map(|v| other.position() + v)
            .collect();

        let normals: Vec<(Vec2f, Vec2f)> = self_vertices
            .iter()
            .circular_tuple_windows() // Get all sides (wrap last point and first)
            .chain(other_vertices.iter().circular_tuple_windows()) // Do also for other shape
            .map(|(p1, p2)| {
                (
                    p1 + (p2 - p1) / 2.0,
                    (p2 - p1)
                        .yx()
                        .normalize()
                        .component_mul(&Vec2f::new(1.0, -1.0)),
                )
            }) // Calculate normal
            .unique_by(|(_, normal)| {
                let x_axis = Vec2f::x_axis();
                let det = normal.x * x_axis.y - normal.y * x_axis.x;
                let mut angle = normal.dot(&Vec2f::x()).atan2(det).to_degrees();
                if angle < 0.0 {
                    angle += 360.0;
                }
                let angle = (angle * 100.0) as i32 % 18000;
                angle
            })
            .collect();

        if DEBUG_DRAW_COLLISION {
            for (start, normal) in &normals {
                let end = start + normal;
                draw_line(start.x, start.y, end.x, end.y, 0.05, GRAY)
            }
        }

        for (start, normal) in &normals {
            // Separating Axis Theorem (SAT)
            let (self_min, self_max) =
                match self_vertices.iter().map(|vert| vert.dot(normal)).minmax() {
                    itertools::MinMaxResult::MinMax(min, max) => (min, max),
                    _ => panic!("We cannot have single point polygons"),
                };

            let (other_min, other_max) =
                match other_vertices.iter().map(|vert| vert.dot(normal)).minmax() {
                    itertools::MinMaxResult::MinMax(min, max) => (min, max),
                    _ => panic!("We cannot have single point polygons"),
                };

            let gap = self_min >= other_max || other_min >= self_max;

            if DEBUG_DRAW_COLLISION {
                let end = start + normal;
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
        }

        Some(CollisionResponse {
            normal: Vec2f::x(),
            depth: 0.0,
        })
    }
}

impl<T: BodyType, U: BodyType> CollisionWith<Polygon<U>> for Circle<T> {
    fn collides(&self, other: &Polygon<U>) -> Option<CollisionResponse> {
        None
    }
}

impl<T: BodyType, U: BodyType> CollisionWith<Circle<U>> for Polygon<T> {
    fn collides(&self, other: &Circle<U>) -> Option<CollisionResponse> {
        other.collides(self)
    }
}

use bevy::{math::Vec2Swizzles, prelude::*};
use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin, DebugShapes};
use itertools::Itertools;

use super::shape::{circle::Circle, convex_polygon::ConvexPolygon};

pub struct CollisionResponse {
    pub normal: Vec2,
    pub depth: f32,
}

pub trait CollisionWith<T> {
    fn collides(
        &self,
        other: &T,
        transform: &Transform,
        other_transform: &Transform,
        lines: &mut ResMut<DebugLines>,
    ) -> Option<CollisionResponse>;
}

impl CollisionWith<Circle> for Circle {
    fn collides(
        &self,
        other: &Circle,
        transform: &Transform,
        other_transform: &Transform,
        lines: &mut ResMut<DebugLines>,
    ) -> Option<CollisionResponse> {
        let normal = (other_transform.translation - transform.translation).truncate();
        let distance = normal.length();
        let radii = self.radius() + other.radius();

        // let end = transform.translation.truncate() + (1.0 + self.radius()) * normal.normalize();
        // lines.line_colored(transform.translation, end.extend(0.0), 0.0, Color::RED);
        // let end =
        //     other_transform.translation.truncate() + (1.0 + other.radius()) * -normal.normalize();
        // lines.line_colored(
        //     other_transform.translation,
        //     end.extend(0.0),
        //     0.0,
        //     Color::RED,
        // );

        if distance >= radii {
            return None;
        }

        Some(CollisionResponse {
            normal: normal.normalize(),
            depth: radii - distance,
        })
    }
}

impl CollisionWith<ConvexPolygon> for Circle {
    fn collides(
        &self,
        other: &ConvexPolygon,
        transform: &Transform,
        other_transform: &Transform,
        lines: &mut ResMut<DebugLines>,
    ) -> Option<CollisionResponse> {
        let other_vertices: Vec<Vec2> = other
            .vertices()
            .iter()
            .map(|&v| other_transform.transform_point(v.extend(0.0)).truncate())
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
            transform.translation.truncate(),
            other_vertices
                .iter()
                .map(|&vert| vert - transform.translation.truncate())
                .min_by(|x, y| x.length().total_cmp(&y.length()))
                .unwrap()
                .normalize(),
        ));

        // for (start, normal) in &normals {
        //     let end = *start + (1.0 + self.radius()) * normal.normalize();
        //     lines.line_colored(start.extend(0.0), end.extend(0.0), 0.0, Color::RED);
        // }

        let mut response_depth = f32::MAX;
        let mut response_normal = Vec2::ZERO;

        for (start, normal) in &normals {
            // Separating Axis Theorem (SAT)
            let circle_proj = transform.translation.truncate().dot(*normal);
            let self_min = circle_proj - self.radius();
            let self_max = circle_proj + self.radius();

            let (other_min, other_max) =
                match other_vertices.iter().map(|vert| vert.dot(*normal)).minmax() {
                    itertools::MinMaxResult::MinMax(min, max) => (min, max),
                    _ => panic!("We cannot have single point polygons"),
                };

            let gap = self_min >= other_max || other_min >= self_max;

            // let end = *start + normal.normalize();
            // lines.line_colored(
            //     start.extend(0.0),
            //     end.extend(0.0),
            //     0.0,
            //     if gap { Color::GREEN } else { Color::RED },
            // );

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
        let direction = other_transform.translation.truncate() - transform.translation.truncate();
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

impl CollisionWith<Circle> for ConvexPolygon {
    fn collides(
        &self,
        other: &Circle,
        transform: &Transform,
        other_transform: &Transform,
        lines: &mut ResMut<DebugLines>,
    ) -> Option<CollisionResponse> {
        let mut collision_response = other.collides(self, other_transform, transform, lines);
        if let Some(mut response) = collision_response {
            response.normal *= -1.0;
            collision_response = Some(response);
        }
        collision_response
    }
}

impl CollisionWith<ConvexPolygon> for ConvexPolygon {
    fn collides(
        &self,
        other: &ConvexPolygon,
        transform: &Transform,
        other_transform: &Transform,
        lines: &mut ResMut<DebugLines>,
    ) -> Option<CollisionResponse> {
        let self_vertices: Vec<Vec2> = self
            .vertices()
            .iter()
            .map(|&v| transform.transform_point(v.extend(0.0)).truncate())
            .collect();
        let other_vertices: Vec<Vec2> = other
            .vertices()
            .iter()
            .map(|&v| other_transform.transform_point(v.extend(0.0)).truncate())
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

        // for (start, normal) in &normals {
        //     let end = *start + normal.normalize();
        //     lines.line_colored(start.extend(0.0), end.extend(0.0), 0.0, Color::RED);
        // }

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

            // let end = *start + normal.normalize();
            // lines.line_colored(
            //     start.extend(0.0),
            //     end.extend(0.0),
            //     0.0,
            //     if gap { Color::GREEN } else { Color::RED },
            // );

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
        let direction = other_transform.translation.truncate() - transform.translation.truncate();
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

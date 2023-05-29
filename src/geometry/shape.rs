use bevy::prelude::Vec2;
use enum_as_inner::EnumAsInner;

use self::{circle::Circle, convex_polygon::ConvexPolygon};

pub mod circle;
pub mod convex_polygon;

#[derive(EnumAsInner)]
pub enum Shape {
    Circle(Circle),
    ConvexPolygon(ConvexPolygon),
}

impl Shape {
    pub fn circle(radius: f32) -> Self {
        Self::Circle(Circle::new(radius))
    }

    pub fn convex_polygon(vertices: Vec<Vec2>) -> Self {
        Self::ConvexPolygon(ConvexPolygon::new(vertices))
    }

    pub fn rect(width: f32, height: f32) -> Self {
        let width_half = width / 2.0;
        let height_half = height / 2.0;

        let vertices = vec![
            Vec2::new(width_half, height_half),
            Vec2::new(width_half, -height_half),
            Vec2::new(-width_half, -height_half),
            Vec2::new(-width_half, height_half),
        ];

        Self::ConvexPolygon(ConvexPolygon::new(vertices))
    }

    pub fn regular_polygon(radius: f32, sides: usize) -> Self {
        let mut vertices: Vec<Vec2> = Vec::new();

        let dphi = 360.0 / sides as f64;
        for s in 0..sides {
            let phi = (90.0 + s as f64 * dphi).to_radians();
            vertices.push(radius * Vec2::new(phi.cos() as f32, phi.sin() as f32));
        }

        Self::ConvexPolygon(ConvexPolygon::new(vertices))
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use crate::math::Vector2;

    use super::*;

    #[test]
    fn rect() {
        // (-w/2,  h/2)__________________(w/2,  h/2)
        //     [3]     |                |    [0]
        //             |                |
        //             |     (0,0)      |
        //             |                |
        //             |                |
        //     [2]     |________________|    [1]
        // (-w/2, -h/2)                  (w/2, -h/2)
        let (w, h) = (20.0, 100.0);
        let vertices = Shape::rect(w, h)
            .as_convex_polygon()
            .unwrap()
            .vertices()
            .clone();
        assert_eq!(
            vertices,
            [
                Vec2::new(w / 2.0, h / 2.0),
                Vec2::new(w / 2.0, -h / 2.0),
                Vec2::new(-w / 2.0, -h / 2.0),
                Vec2::new(-w / 2.0, h / 2.0),
            ]
        );
    }

    #[test]
    fn regular_triangle() {
        let sides = 3;
        let radius = 2.0;
        let phi = 2.0 * std::f64::consts::PI / sides as f64;
        let phi = phi - std::f64::consts::FRAC_PI_2;
        let vertices = vec![
            radius * Vec2::new(0.0, 1.0),
            radius * Vec2::new(-phi.cos() as f32, -phi.sin() as f32),
            radius * Vec2::new(phi.cos() as f32, -phi.sin() as f32),
        ];

        let shape_vertices = Shape::regular_polygon(radius, sides)
            .as_convex_polygon()
            .unwrap()
            .vertices()
            .clone();

        for i in 0..3 {
            assert_abs_diff_eq!(Vector2::from(vertices[i]), Vector2::from(shape_vertices[i]));
        }
    }

    #[test]
    fn regular_square() {
        let sides = 4;
        let radius = 2.0;
        let vertices = [
            radius * Vec2::new(0.0, 1.0),
            radius * Vec2::new(-1.0, 0.0),
            radius * Vec2::new(0.0, -1.0),
            radius * Vec2::new(1.0, 0.0),
        ];

        let shape_vertices = Shape::regular_polygon(radius, sides)
            .as_convex_polygon()
            .unwrap()
            .vertices()
            .clone();

        for i in 0..3 {
            assert_abs_diff_eq!(vertices[i].x, shape_vertices[i].x);
            assert_abs_diff_eq!(vertices[i].y, shape_vertices[i].y);
        }
    }
}

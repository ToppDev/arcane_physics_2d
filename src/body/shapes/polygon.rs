use itertools::Itertools;

use crate::{
    body::{Drawable, Movable, Position},
    draw::{draw_triangle, Color},
    math::Vec2f,
};

use super::SHAPE_BORDER_WIDTH;

#[derive(Debug)]
pub struct Polygon {
    centroid: Vec2f,
    vertices: Vec<Vec2f>,
    fill_color: Color,
    hitbox_color: Option<Color>,
}

impl Polygon {
    pub fn new(points: &[Vec2f], fill_color: Color, hitbox_color: Option<Color>) -> Self {
        let centroid = Polygon::calc_centroid(points);
        Self {
            centroid,
            vertices: points.iter().map(|x| x - centroid).collect(),
            fill_color,
            hitbox_color,
        }
    }

    fn calc_centroid(points: &[Vec2f]) -> Vec2f {
        let n = points.len();
        let mut centroid = Vec2f::zeros();
        let mut area = 0.0;

        for i in 0..n {
            let j = (i + 1) % n;
            let cross = points[i].x * points[j].y - points[j].x * points[i].y;
            centroid.x += (points[i].x + points[j].x) * cross;
            centroid.y += (points[i].y + points[j].y) * cross;
            area += cross;
        }

        if area != 0.0 {
            centroid /= 3.0 * area;
        }

        centroid
    }
}

impl Position for Polygon {
    fn position(&self) -> Vec2f {
        self.centroid
    }
}

impl Movable for Polygon {
    fn move_to(&mut self, destination: Vec2f) {
        self.centroid = destination;
    }
    fn offset(&mut self, offset: Vec2f) {
        self.centroid += offset;
    }
}

impl Drawable for Polygon {
    fn draw(&self) {
        if let Some(hitbox_color) = self.hitbox_color {
            self.vertices
                .iter()
                .map(|v| v + self.centroid)
                .circular_tuple_windows()
                .for_each(|(v1, v2, v3)| draw_triangle(v1, v2, v3, hitbox_color));
        }
        self.vertices
            .iter()
            .map(|v| v - v.normalize() * SHAPE_BORDER_WIDTH + self.centroid)
            .combinations(3)
            .for_each(|p| draw_triangle(p[0], p[1], p[2], self.fill_color));
    }
}

use itertools::Itertools;

extern crate nalgebra as na;

pub type Vec2f = na::Vector2<f32>;
pub type Vec2d = na::Vector2<f64>;
pub type Point2f = na::Point2<f32>;
pub type Point2d = na::Point2<f64>;

pub fn calc_polygon_area(points: &[Vec2f]) -> f32 {
    let sum: f32 = points
        .iter()
        .circular_tuple_windows()
        .map(|(p1, p2)| (p1.y + p2.y) * (p1.x - p2.x))
        .sum();
    0.5 * sum.abs()
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use super::*;

    #[test]
    fn area_rectangle() {
        let points = [
            Vec2f::new(0.0, 0.0),
            Vec2f::new(10.0, 0.0),
            Vec2f::new(10.0, 10.0),
            Vec2f::new(0.0, 10.0),
        ];
        assert_eq!(calc_polygon_area(&points), 10.0 * 10.0);
    }
    #[test]
    fn area_hexagon() {
        let cos_60 = 60.0_f32.to_radians().cos();
        let sin_60 = 60.0_f32.to_radians().sin();
        let points = [
            Vec2f::new(1.0, 0.0),
            Vec2f::new(cos_60, sin_60),
            Vec2f::new(-cos_60, sin_60),
            Vec2f::new(-1.0, 0.0),
            Vec2f::new(-cos_60, -sin_60),
            Vec2f::new(cos_60, -sin_60),
        ];
        let edge_length: f32 = 1.0;
        let area_regular_hexagon = (3.0 * 3.0_f32.sqrt() / 2.0) * (edge_length).powi(2);
        assert_approx_eq!(calc_polygon_area(&points), area_regular_hexagon, 2.4e-7_f32);
    }
    #[test]
    fn area_arrow() {
        // (10,  -2) _____(15, -2)
        //          |     \
        //          |      \ (18, -6)
        //          |      /
        //          |_____/
        // (10, -10)      (15, -10)
        let points = [
            Vec2f::new(10.0, -2.0),
            Vec2f::new(15.0, -2.0),
            Vec2f::new(18.0, -6.0),
            Vec2f::new(15.0, -10.0),
            Vec2f::new(10.0, -10.0),
        ];
        let area_rectangle = ((points[1].x - points[0].x) * (points[4].y - points[0].y)).abs();
        let area_triangle = 0.5 * ((points[3].y - points[1].y) * (points[2].x - points[1].x)).abs();
        assert_eq!(calc_polygon_area(&points), area_rectangle + area_triangle);
    }
}

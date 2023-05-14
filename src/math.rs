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

pub fn calc_polygon_centroid(points: &[Vec2f]) -> Vec2f {
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

pub fn calc_rect_vertices(x: f32, y: f32, w: f32, h: f32, rot_deg: f32) -> [Vec2f; 4] {
    let half_width = (w / 2.0) as f64;
    let half_height = (h / 2.0) as f64;
    let pos = Vec2d::new(x as f64, y as f64);

    let vertices = [
        Vec2d::new(-half_width, -half_height),
        Vec2d::new(half_width, -half_height),
        Vec2d::new(half_width, half_height),
        Vec2d::new(-half_width, half_height),
    ];
    let rotation = nalgebra::Rotation2::new((rot_deg as f64).to_radians());

    vertices
        .map(|x| rotation * x)
        .map(|x| pos + x)
        .map(|x| nalgebra::convert(x))
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

    #[test]
    fn gen_vertices_centered() {
        let (x, y, w, h, r) = (0.0, 0.0, 20.0, 100.0, 0.0);
        let vertices = calc_rect_vertices(x, y, w, h, r);
        assert_eq!(
            vertices,
            [
                Vec2f::new(-w / 2.0, -h / 2.0),
                Vec2f::new(w / 2.0, -h / 2.0),
                Vec2f::new(w / 2.0, h / 2.0),
                Vec2f::new(-w / 2.0, h / 2.0),
            ]
        );
    }

    #[test]
    fn gen_vertices_shifted() {
        let (x, y, w, h, r) = (10.0, 50.0, 20.0, 100.0, 0.0);
        let vertices = calc_rect_vertices(x, y, w, h, r);
        assert_eq!(
            vertices,
            [
                Vec2f::new(x - w / 2.0, y - h / 2.0),
                Vec2f::new(x + w / 2.0, y - h / 2.0),
                Vec2f::new(x + w / 2.0, y + h / 2.0),
                Vec2f::new(x - w / 2.0, y + h / 2.0),
            ]
        );
    }

    #[test]
    fn gen_vertices_rot_90() {
        let (x, y, w, h, r) = (0.0, 0.0, 20.0, 100.0, 90.0);
        let vertices = calc_rect_vertices(x, y, w, h, r);
        assert_eq!(
            vertices,
            [
                Vec2f::new(h / 2.0, -w / 2.0),
                Vec2f::new(h / 2.0, w / 2.0),
                Vec2f::new(-h / 2.0, w / 2.0),
                Vec2f::new(-h / 2.0, -w / 2.0),
            ]
        );
    }

    #[test]
    fn gen_vertices_rot_m90() {
        let (x, y, w, h, r) = (0.0, 0.0, 20.0, 100.0, -90.0);
        let vertices = calc_rect_vertices(x, y, w, h, r);
        assert_eq!(
            vertices,
            [
                Vec2f::new(-h / 2.0, w / 2.0),
                Vec2f::new(-h / 2.0, -w / 2.0),
                Vec2f::new(h / 2.0, -w / 2.0),
                Vec2f::new(h / 2.0, w / 2.0),
            ]
        );
    }

    #[test]
    fn gen_vertices_rot_45() {
        let (x, y, w, h, r) = (0.0, 0.0, 20.0, 20.0, 45.0);
        let vertices = calc_rect_vertices(x, y, w, h, r);
        let diag = ((w / 2.0).powi(2) + (h / 2.0).powi(2)).sqrt();

        assert_approx_eq!(vertices[0].x, 0.0);
        assert_approx_eq!(vertices[0].y, -diag);

        assert_approx_eq!(vertices[1].x, diag);
        assert_approx_eq!(vertices[1].y, 0.0);

        assert_approx_eq!(vertices[2].x, 0.0);
        assert_approx_eq!(vertices[2].y, diag);

        assert_approx_eq!(vertices[3].x, -diag);
        assert_approx_eq!(vertices[3].y, 0.0);
    }
}

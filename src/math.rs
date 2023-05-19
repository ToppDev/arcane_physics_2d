use itertools::Itertools;
use macroquad::prelude::*;

pub fn calc_polygon_area(points: &[Vec2]) -> f32 {
    let sum: f32 = points
        .iter()
        .circular_tuple_windows()
        .map(|(p1, p2)| (p1.y + p2.y) * (p1.x - p2.x))
        .sum();
    0.5 * sum.abs()
}

pub fn calc_polygon_centroid(points: &[Vec2]) -> Vec2 {
    let n = points.len();
    let mut centroid = Vec2::ZERO;
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

pub fn calc_rect_vertices(x: f32, y: f32, w: f32, h: f32, rot_deg: f32) -> [Vec2; 4] {
    let half_width = (w / 2.0) as f64;
    let half_height = (h / 2.0) as f64;
    let pos = DVec2::new(x as f64, y as f64);

    // (-w/2,  h/2)__________________(w/2,  h/2)
    //     [3]     |                |    [0]
    //             |                |
    //             |     (0,0)      |
    //             |                |
    //             |                |
    //     [2]     |________________|    [1]
    // (-w/2, -h/2)                  (w/2, -h/2)
    let vertices = [
        DVec2::new(half_width, half_height),
        DVec2::new(half_width, -half_height),
        DVec2::new(-half_width, -half_height),
        DVec2::new(-half_width, half_height),
    ];
    let rotation = DAffine2::from_angle((rot_deg as f64).to_radians());

    vertices
        .map(|x| rotation.transform_vector2(x))
        .map(|x| (pos + x).as_vec2())
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use super::*;

    #[test]
    fn area_rectangle() {
        let points = [
            Vec2::new(0.0, 0.0),
            Vec2::new(10.0, 0.0),
            Vec2::new(10.0, 10.0),
            Vec2::new(0.0, 10.0),
        ];
        assert_eq!(calc_polygon_area(&points), 10.0 * 10.0);
    }
    #[test]
    fn area_hexagon() {
        let cos_60 = 60.0_f32.to_radians().cos();
        let sin_60 = 60.0_f32.to_radians().sin();
        let points = [
            Vec2::new(1.0, 0.0),
            Vec2::new(cos_60, sin_60),
            Vec2::new(-cos_60, sin_60),
            Vec2::new(-1.0, 0.0),
            Vec2::new(-cos_60, -sin_60),
            Vec2::new(cos_60, -sin_60),
        ];
        let edge_length: f32 = 1.0;
        let area_regular_hexagon = (3.0 * 3.0_f32.sqrt() / 2.0) * (edge_length).powi(2);
        assert_approx_eq!(calc_polygon_area(&points), area_regular_hexagon, 2.4e-7_f32);
    }
    #[test]
    fn area_arrow() {
        // (10,  -2) _____(15, -2) [1]
        //    [0]   |     \
        //          |      \ (18, -6) [2]
        //          |      /
        //    [4]   |_____/
        // (10, -10)      (15, -10) [3]
        let points = [
            Vec2::new(10.0, -2.0),
            Vec2::new(15.0, -2.0),
            Vec2::new(18.0, -6.0),
            Vec2::new(15.0, -10.0),
            Vec2::new(10.0, -10.0),
        ];
        let area_rectangle = ((points[1].x - points[0].x) * (points[4].y - points[0].y)).abs();
        let area_triangle = 0.5 * ((points[3].y - points[1].y) * (points[2].x - points[1].x)).abs();
        assert_eq!(calc_polygon_area(&points), area_rectangle + area_triangle);
    }

    #[test]
    fn gen_vertices_centered() {
        // (-w/2,  h/2)__________________(w/2,  h/2)
        //     [3]     |                |    [0]
        //             |                |
        //             |     (0,0)      |
        //             |                |
        //             |                |
        //     [2]     |________________|    [1]
        // (-w/2, -h/2)                  (w/2, -h/2)
        let (x, y, w, h, r) = (0.0, 0.0, 20.0, 100.0, 0.0);
        let vertices = calc_rect_vertices(x, y, w, h, r);
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
    fn gen_vertices_shifted() {
        let (x, y, w, h, r) = (10.0, 50.0, 20.0, 100.0, 0.0);
        let vertices = calc_rect_vertices(x, y, w, h, r);
        assert_eq!(
            vertices,
            [
                Vec2::new(x + w / 2.0, y + h / 2.0),
                Vec2::new(x + w / 2.0, y - h / 2.0),
                Vec2::new(x - w / 2.0, y - h / 2.0),
                Vec2::new(x - w / 2.0, y + h / 2.0),
            ]
        );
    }

    #[test]
    fn gen_vertices_rot_90() {
        //                                              (-h/2,  w/2)________h________(h/2,  w/2)
        // (-w/2,  h/2)_________w________(w/2,  h/2)         [0]    |               |     [1]
        //     [3]     |                |    [0]                    |               |
        //             |                |           90°             |               |
        //             h     (0,0)      h          ====>            w     (0,0)     w
        //             |                |                           |               |
        //             |                |                           |               |
        //     [2]     |________w_______|    [1]                    |               |
        // (-w/2, -h/2)                  (w/2, -h/2)         [3]    |_______h_______|     [2]
        //                                              (-h/2, -w/2)                 (h/2, -w/2)
        let (x, y, w, h, r) = (0.0, 0.0, 20.0, 100.0, 90.0);
        let vertices = calc_rect_vertices(x, y, w, h, r);
        assert_eq!(
            vertices,
            [
                Vec2::new(-h / 2.0, w / 2.0),
                Vec2::new(h / 2.0, w / 2.0),
                Vec2::new(h / 2.0, -w / 2.0),
                Vec2::new(-h / 2.0, -w / 2.0),
            ]
        );
    }

    #[test]
    fn gen_vertices_rot_m90() {
        //                                              (-h/2,  w/2)________h________(h/2,  w/2)
        // (-w/2,  h/2)_________w________(w/2,  h/2)         [2]    |               |     [3]
        //     [3]     |                |    [0]                    |               |
        //             |                |          -90°             |               |
        //             h     (0,0)      h          ====>            w     (0,0)     w
        //             |                |                           |               |
        //             |                |                           |               |
        //     [2]     |________w_______|    [1]                    |               |
        // (-w/2, -h/2)                  (w/2, -h/2)         [1]    |_______h_______|     [0]
        //                                              (-h/2, -w/2)                 (h/2, -w/2)
        let (x, y, w, h, r) = (0.0, 0.0, 20.0, 100.0, -90.0);
        let vertices = calc_rect_vertices(x, y, w, h, r);
        assert_eq!(
            vertices,
            [
                Vec2::new(h / 2.0, -w / 2.0),
                Vec2::new(-h / 2.0, -w / 2.0),
                Vec2::new(-h / 2.0, w / 2.0),
                Vec2::new(h / 2.0, w / 2.0),
            ]
        );
    }

    #[test]
    fn gen_vertices_rot_45() {
        //                                                        (0, d)
        // (-w/2,  h/2)_________w________(w/2,  h/2)            [0] /\
        //     [3]     |                |    [0]                   /  \
        //             |                |           45°           /    \  [1]
        //             h     (0,0)      h          ====> (-d, 0) /     / (d, 0)
        //             |                |                    [3] \    /
        //             |                |                         \  /
        //     [2]     |________w_______|    [1]                   \/[2]
        // (-w/2, -h/2)                  (w/2, -h/2)             (0, -d)
        //
        let (x, y, w, h, r) = (0.0, 0.0, 20.0, 20.0, 45.0);
        let vertices = calc_rect_vertices(x, y, w, h, r);
        let diag = ((w / 2.0).powi(2) + (h / 2.0).powi(2)).sqrt();

        assert_approx_eq!(vertices[0].x, 0.0);
        assert_approx_eq!(vertices[0].y, diag);

        assert_approx_eq!(vertices[1].x, diag);
        assert_approx_eq!(vertices[1].y, 0.0);

        assert_approx_eq!(vertices[2].x, 0.0);
        assert_approx_eq!(vertices[2].y, -diag);

        assert_approx_eq!(vertices[3].x, -diag);
        assert_approx_eq!(vertices[3].y, 0.0);
    }
}

use crate::math::{Vec2d, Vec2f};

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
    use super::*;
    use assert_approx_eq::assert_approx_eq;

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

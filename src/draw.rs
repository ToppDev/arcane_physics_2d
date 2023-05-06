use crate::math::Vec2f;

pub type Color = macroquad::color::Color;
pub use macroquad::color::colors;

/// Draws a solid circle centered at [x, y] with a given radius r and color.
pub fn draw_circle(x: f32, y: f32, r: f32, color: Color) {
    macroquad::shapes::draw_circle(x, y, r, color);
}

/// Draws a circle outline centered at [x, y] with a given radius, line thickness and color.
pub fn draw_circle_lines(x: f32, y: f32, r: f32, thickness: f32, color: Color) {
    macroquad::shapes::draw_circle_lines(x, y, r, thickness, color);
}

/// Draws an outlined solid hexagon centered at [x, y] with a radius size,
/// outline thickness defined by border, orientation defined by vertical
/// (when true, the hexagon points along the y axis), and colors for outline
/// given by border_color and fill by fill_color.
pub fn draw_hexagon(
    x: f32,
    y: f32,
    size: f32,
    border: f32,
    vertical: bool,
    border_color: Color,
    fill_color: Color,
) {
    macroquad::shapes::draw_hexagon(x, y, size, border, vertical, border_color, fill_color);
}

/// Draws a line between points [x1, y1] and [x2, y2] with a given thickness and color.
pub fn draw_line(x1: f32, y1: f32, x2: f32, y2: f32, thickness: f32, color: Color) {
    macroquad::shapes::draw_line(x1, y1, x2, y2, thickness, color);
}

/// Draws a solid regular polygon centered at [x, y] with a given number of sides, radius, clockwise rotation (in degrees) and color.
pub fn draw_poly(x: f32, y: f32, sides: u8, radius: f32, rotation: f32, color: Color) {
    macroquad::shapes::draw_poly(x, y, sides, radius, rotation, color);
}

/// Draws a regular polygon outline centered at [x, y] with a given number of sides, radius, clockwise rotation (in degrees), line thickness, and color.
pub fn draw_poly_lines(
    x: f32,
    y: f32,
    sides: u8,
    radius: f32,
    rotation: f32,
    thickness: f32,
    color: Color,
) {
    macroquad::shapes::draw_poly_lines(x, y, sides, radius, rotation, thickness, color);
}

/// Draws a solid rectangle with its top-left corner at [x, y] with size [w, h] (width going to the right, height going down), with a given color.
pub fn draw_rectangle(x: f32, y: f32, w: f32, h: f32, color: Color) {
    macroquad::shapes::draw_rectangle(x, y, w, h, color);
}

/// Draws a rectangle outline with its top-left corner at [x, y] with size [w, h] (width going to the right, height going down), with a given line thickness and color.
pub fn draw_rectangle_lines(x: f32, y: f32, w: f32, h: f32, thickness: f32, color: Color) {
    macroquad::shapes::draw_rectangle_lines(x, y, w, h, thickness, color);
}

/// Draws a solid triangle between points v1, v2, and v3 with a given color.
pub fn draw_triangle(v1: Vec2f, v2: Vec2f, v3: Vec2f, color: Color) {
    let v1 = macroquad::math::vec2(v1.x, v1.y);
    let v2 = macroquad::math::vec2(v2.x, v2.y);
    let v3 = macroquad::math::vec2(v3.x, v3.y);
    macroquad::shapes::draw_triangle(v1, v2, v3, color)
}

/// Draws a triangle outline between points v1, v2, and v3 with a given line thickness and color.
pub fn draw_triangle_lines(v1: Vec2f, v2: Vec2f, v3: Vec2f, thickness: f32, color: Color) {
    let v1 = macroquad::math::vec2(v1.x, v1.y);
    let v2 = macroquad::math::vec2(v2.x, v2.y);
    let v3 = macroquad::math::vec2(v3.x, v3.y);
    macroquad::shapes::draw_triangle_lines(v1, v2, v3, thickness, color);
}

pub mod circle;
pub mod components;
pub mod polygon;

pub trait BodyType {}
pub struct Dynamic;
pub struct Static;
impl BodyType for Dynamic {}
impl BodyType for Static {}

pub trait Drawable {
    fn draw(&self);
}
pub trait Updatable {
    fn update(&mut self, dt: f32);
}

pub struct PhysicalProperties {
    /// Body density in [g/cm^3]
    pub density: f32,
    /// Body mass in [kg]
    pub mass: f32,
    /// The coefficient of restitution (COR, also denoted by e), is the ratio of the final to
    /// initial relative speed between two objects after they collide.
    ///
    /// It normally ranges from 0 to 1 where 1 would be a perfectly elastic collision.
    /// A perfectly inelastic collision has a coefficient of 0, but a 0 value does not have to be
    /// perfectly inelastic
    pub restitution: f32,
    /// Area of the body in [m^2]
    pub area: f32,
}

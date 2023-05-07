pub const MIN_BODY_SIZE: f32 = 0.01 * 0.01; // [m^2]
pub const MAX_BODY_SIZE: f32 = 64.0 * 64.0; // [m^2]

pub const MIN_DENSITY: f32 = 0.2; // [g/cm^3]
pub const MAX_DENSITY: f32 = 21.4; // [g/cm^3] (density of platinum)

pub fn validate_body_parameters(area: f32, density: f32, restitution: f32) -> Result<f32, String> {
    if area < MIN_BODY_SIZE {
        return Err(format!(
            "Area too small. Min area is {MIN_BODY_SIZE} ({area} requested)"
        ));
    }
    if area > MAX_BODY_SIZE {
        return Err(format!(
            "Area too large. Max area is {MAX_BODY_SIZE} ({area} requested)"
        ));
    }
    if density < MIN_DENSITY {
        return Err(format!(
            "Density too small. Min density is {MIN_DENSITY} ({density} requested)"
        ));
    }
    if density > MAX_DENSITY {
        return Err(format!(
            "Density too large. Max density is {MAX_DENSITY} ({density} requested)"
        ));
    }

    let restitution = restitution.clamp(0.0, 1.0);

    Ok(restitution)
}

use crate::modules::vec3::Vec3;

pub type Color = Vec3;

/*
 * Convert a linear color component to a gamma color component via gamma 2 transform.
 * 
 * Images with data that are written without being transformed are said to be in linear space, 
 * whereas images that are transformed are said to be in gamma space. 
 */
pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 { linear_component.sqrt() } else { 0.0 }
}

pub fn write_color(pixel_color: Color) -> String {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    // Apply gamma 2 transform
    let r = linear_to_gamma(r);
    let g = linear_to_gamma(g);
    let b = linear_to_gamma(b);

    // Translate [0, 1] range to [0, 255] range, clamped
    format!("{} {} {}\n", 
        (256.0 * r.clamp(0.0, 0.999)) as u32, 
        (256.0 * g.clamp(0.0, 0.999)) as u32, 
        (256.0 * b.clamp(0.0, 0.999)) as u32)
}
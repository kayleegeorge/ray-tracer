use crate::modules::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(pixel_color: Color) -> String {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    // Translate [0, 1] range to [0, 255] range, clamped
    format!("{} {} {}", 
        (256.0 * r.clamp(0.0, 0.999)) as u32, 
        (256.0 * g.clamp(0.0, 0.999)) as u32, 
        (256.0 * b.clamp(0.0, 0.999)) as u32)
}
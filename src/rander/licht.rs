use crate::rander::rander_model::Vec3;
use crate::rander::rander_model::rgb;

pub fn vec3_dot(a: Vec3, b: Vec3) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

pub fn vec3_normalize(v: Vec3) -> Vec3 {
    let len = (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();
    Vec3 {
        x: v.x / len,
        y: v.y / len,
        z: v.z / len,
    }
}

pub fn apply_light(color: u32, normal: Vec3, light_dir: Vec3, strake: f32) -> u32 {
    let n = vec3_normalize(normal);
    let l = vec3_normalize(light_dir);
    let mut intensity = vec3_dot(n, l).max(0.0);

    // Optional: ambient light
    intensity = (intensity + strake).min(1.0);

    let r = ((color >> 16) & 0xFF) as f32 * intensity;
    let g = ((color >> 8) & 0xFF) as f32 * intensity;
    let b = (color & 0xFF) as f32 * intensity;

    rgb(r.min(255.0) as u8, g.min(255.0) as u8, b.min(255.0) as u8)
}

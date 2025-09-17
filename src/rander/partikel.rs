// rander/partiklel.rs

use rand::{random};

pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub dx: f32,
    pub dy: f32,
    pub dz: f32,
    pub life: f32,
}

pub fn partikel_lode(anzahl: usize, life: f32) -> Vec<Particle> {
    let mut particles = Vec::new();

    for _ in 0..anzahl {
        particles.push(Particle {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            dx: (random::<f32>() - 0.5) * 0.5,
            dy: (random::<f32>() - 0.5) * 0.5,
            dz: (random::<f32>() - 0.5) * 0.5,
            life,
        });
    }

    particles
}

fn lerp(a: u8, b: u8, t: f32) -> u8 {
    ((a as f32) + (b as f32 - a as f32) * t).round() as u8
}

/// Neue Version von `rander_partikel` – berücksichtigt Kamerarotation (yaw und pitch)
pub fn rander_partikel(
    particles: &mut Vec<Particle>,
    buffer: &mut [u32],
    zbuffer: &mut [f32],
    width: usize,
    height: usize,
    player_pos: (f32, f32, f32),
    player_rot: (f32, f32), // (yaw, pitch)
    moveing: (f32, f32, f32),
    r: f32,
    g: f32,
    b: f32,
) {
    let fov: f32 = 60.0;

    let yaw = -player_rot.0;
    let pitch = -player_rot.1;

    let cos_yaw = yaw.cos();
    let sin_yaw = yaw.sin();
    let cos_pitch = pitch.cos();
    let sin_pitch = pitch.sin();

    particles.retain_mut(|p| {
        // Move
        let dt_x = 1.0 / moveing.0.max(0.001);
        let dt_y = 1.0 / moveing.1.max(0.001);
        let dt_z = 1.0 / moveing.2.max(0.001);

        p.x += p.dx * dt_x;
        p.y += p.dy * dt_y;
        p.z += p.dz * dt_z;
        p.life -= 0.05;

        if p.life <= 0.0 {
            return false;
        }

        // Relative to camera
        let rel_x = p.x - player_pos.0;
        let rel_y = p.y - player_pos.1;
        let rel_z = p.z - player_pos.2;

        // Rotate around Y (Yaw)
        let x1 = rel_x * cos_yaw - rel_z * sin_yaw;
        let z1 = rel_x * sin_yaw + rel_z * cos_yaw;

        // Rotate around X (Pitch)
        let y1 = rel_y * cos_pitch - z1 * sin_pitch;
        let z2 = rel_y * sin_pitch + z1 * cos_pitch;

        let fov_rad = (fov.to_radians() / 2.0).tan();
        let aspect_ratio = width as f32 / height as f32;

        if z2 <= 0.1 {
            return true;
        }

        let px = x1 / (z2 * fov_rad * aspect_ratio);
        let py = y1 / (z2 * fov_rad);

        if px.abs() > 1.0 || py.abs() > 1.0 {
            return true;
        }

        let screen_x = ((px + 1.0) * 0.5 * width as f32).round() as isize;
        let screen_y = ((1.0 - (py + 1.0) * 0.5) * height as f32).round() as isize;

        let point_size = ((5.0 / z2).clamp(1.0, 4.0)) as isize;

        let dist_sq = rel_x * rel_x + rel_y * rel_y + rel_z * rel_z;
        if dist_sq > 1000.0 {
            p.life -= 0.2;
        }

        if rel_x.abs() > 100.0 || rel_y.abs() > 100.0 || rel_z.abs() > 100.0 {
            return true;
        }

        for dy in -point_size..=point_size {
            for dx in -point_size..=point_size {
                let sx = screen_x + dx;
                let sy = screen_y + dy;

                if sx >= 0 && sx < width as isize && sy >= 0 && sy < height as isize {
                    let idx = sy as usize * width + sx as usize;
                    if z2 < zbuffer[idx] {
                        zbuffer[idx] = z2;

                        let intensity = (p.life / 20.0).clamp(0.0, 1.0);
                        let r = (r * intensity) as u32;
                        let g = (g * intensity) as u32;
                        let b = (b * intensity) as u32;
                        buffer[idx] = (r << 16) | (g << 8) | b;
                    }
                }
            }
        }

        true // keep the particle
    });
}

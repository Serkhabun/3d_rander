// logik/hitbox.rs

use crate::rander::rander_model::{Vec3, load_obj};

#[derive(Debug)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

pub fn load_obj_hitbox(path: &str) -> Option<AABB> {
    let model = load_obj(path);

    if model.vertices.is_empty() {
        return None;
    }

    let mut min = model.vertices[0];
    let mut max = model.vertices[0];

    for v in &model.vertices[1..] {
        min.x = min.x.min(v.x);
        min.y = min.y.min(v.y);
        min.z = min.z.min(v.z);

        max.x = max.x.max(v.x);
        max.y = max.y.max(v.y);
        max.z = max.z.max(v.z);
    }

    Some(AABB { min, max })
}

pub fn check_aabb_collision(a: &AABB, b: &AABB, a_pos: (f32, f32, f32), b_pos: (f32, f32, f32)) -> bool {
    let a_min = Vec3 {
        x: a.min.x + a_pos.0,
        y: a.min.y + a_pos.1,
        z: a.min.z + a_pos.2,
    };
    let a_max = Vec3 {
        x: a.max.x + a_pos.0,
        y: a.max.y + a_pos.1,
        z: a.max.z + a_pos.2,
    };

    let b_min = Vec3 {
        x: b.min.x + b_pos.0,
        y: b.min.y + b_pos.1,
        z: b.min.z + b_pos.2,
    };
    let b_max = Vec3 {
        x: b.max.x + b_pos.0,
        y: b.max.y + b_pos.1,
        z: b.max.z + b_pos.2,
    };

    a_min.x <= b_max.x && a_max.x >= b_min.x &&
    a_min.y <= b_max.y && a_max.y >= b_min.y &&
    a_min.z <= b_max.z && a_max.z >= b_min.z
}

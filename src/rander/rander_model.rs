use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use crate::rander::licht::*;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug)]
pub struct FaceVertex {
    pub vertex_index: usize,
    pub texcoord_index: Option<usize>,
    pub normal_index: Option<usize>,
}

pub struct Model {
    pub vertices: Vec<Vec3>,
    pub texcoords: Vec<(f32, f32)>,
    pub normals: Vec<Vec3>,
    pub faces: Vec<(Vec<FaceVertex>, String)>, // <--- Neue Struktur
    pub edges: Vec<(usize, usize)>,
}

pub fn load_obj(path: &str) -> Model {
    let file = File::open(path).expect("Konnte Datei nicht öffnen");
    let reader = BufReader::new(file);

    let mut vertices = Vec::new();
    let mut edges = Vec::new();
    let mut faces = Vec::new();
    let mut texcoords = Vec::new();
    let mut normals = Vec::new();

    let mut current_material = String::new(); // martiral

    for line in reader.lines() {
        let line = line.unwrap();
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.is_empty() { continue; }

        match tokens[0] {
            "v" => {
                let x: f32 = tokens[1].parse().unwrap();
                let y: f32 = tokens[2].parse().unwrap();
                let z: f32 = tokens[3].parse().unwrap();
                vertices.push(Vec3 { x, y, z });
            }
            "vt" => {
                let u: f32 = tokens[1].parse().unwrap();
                let v: f32 = tokens[2].parse().unwrap();
                texcoords.push((u, v));
            }
            "vn" => {
                let x: f32 = tokens[1].parse().unwrap();
                let y: f32 = tokens[2].parse().unwrap();
                let z: f32 = tokens[3].parse().unwrap();
                normals.push(Vec3 { x, y, z });
            }
            "l" => {
                let i1 = tokens[1].parse::<usize>().unwrap() - 1;
                let i2 = tokens[2].parse::<usize>().unwrap() - 1;
                edges.push((i1, i2));
            }
            "usemtl" => {
                current_material = tokens[1].to_string();
            }
            "f" => {
                let mut face = Vec::new();
                for s in &tokens[1..] {
                    let mut parts = s.split('/');
                    let v_idx = parts.next().unwrap().parse::<usize>().unwrap() - 1;
                    let t_idx = parts.next().and_then(|s| s.parse::<usize>().ok()).map(|i| i - 1);
                    let n_idx = parts.next().and_then(|s| s.parse::<usize>().ok()).map(|i| i - 1);
                    face.push(FaceVertex {
                        vertex_index: v_idx,
                        texcoord_index: t_idx,
                        normal_index: n_idx,
                    });
                }

                // Optional: edges für wireframe
                for i in 0..face.len() {
                    let a = face[i].vertex_index;
                    let b = face[(i + 1) % face.len()].vertex_index;
                    edges.push((a, b));
                }

                faces.push((face, current_material.clone()));
            }
            _ => {}
        }
    }

    Model {
        vertices,
        texcoords,
        normals,
        edges,
        faces,
    }
}

pub fn load_mtl(path: &str) -> HashMap<String, (u8, u8, u8)> {
    let file = File::open(path).expect("Konnte .mtl Datei nicht öffnen");
    let reader = BufReader::new(file);

    let mut materials = HashMap::new();
    let mut current_name = String::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.is_empty() {
            continue;
        }

        match tokens[0] {
            "newmtl" => {
                current_name = tokens[1].to_string();
            }
            "Kd" => {
                let r: f32 = tokens[1].parse().unwrap_or(1.0);
                let g: f32 = tokens[2].parse().unwrap_or(1.0);
                let b: f32 = tokens[3].parse().unwrap_or(1.0);

                let r_u8 = (r * 255.0) as u8;
                let g_u8 = (g * 255.0) as u8;
                let b_u8 = (b * 255.0) as u8;

                materials.insert(current_name.clone(), (r_u8, g_u8, b_u8));
            }
            _ => {}
        }
    }

    materials
}

fn draw_filled_polygon(
    points: &[(isize, isize)],
    depths: &[f32],
    buffer: &mut [u32],
    zbuffer: &mut [f32],
    width: usize,
    height: usize,
    color_fn: impl Fn(f32, isize, isize) -> u32,
) {
    if points.len() < 3 {
        return; // Nicht genug Punkte für Fläche
    }
    let avg_depth = depths.iter().copied().sum::<f32>() / depths.len() as f32;

    // 1. Finde min/max Y
    let min_y = points.iter().map(|p| p.1).min().unwrap();
    let max_y = points.iter().map(|p| p.1).max().unwrap();

    // 2. Für jede Scanline zwischen min_y und max_y
    for y in min_y..=max_y {
        if y < 0 || y >= height as isize {
            continue;
        }

        let mut intersections = Vec::new();

        // 3. Finde alle Schnittpunkte der Scanline mit Polygonkanten
        for i in 0..points.len() {
            let (x1, y1) = points[i];
            let (x2, y2) = points[(i + 1) % points.len()];

            if (y1 <= y && y2 > y) || (y2 <= y && y1 > y) {
                // Kante schneidet Scanline, berechne Schnittpunkt x
                let x = x1 + (y - y1) * (x2 - x1) / (y2 - y1);
                intersections.push(x);
            }
        }

        // 4. Sortiere Schnittpunkte und fülle zwischen je zwei Punkten
        intersections.sort();

        for pair in intersections.chunks(2) {
            if pair.len() == 2 {
                let x_start = pair[0].max(0);
                let x_end = pair[1].min(width as isize - 1);

                
                for x in x_start..=x_end {
                    if zbuffer[y as usize * width + x as usize] > avg_depth {
                        zbuffer[y as usize * width + x as usize] = avg_depth;
                        buffer[y as usize * width + x as usize] = color_fn(avg_depth, x, y);
                    }
                }
            }
        }
    }
}


fn inverse_transform(v: Vec3, angle: (f32, f32, f32), pos: (f32, f32, f32)) -> Vec3 {
    // Erst Translation rückgängig machen
    let mut x = v.x - pos.0;
    let mut y = v.y - pos.1;
    let mut z = v.z - pos.2;

    // Dann inverse Rotation (entgegengesetzt)
    let (rx, ry, rz) = angle;
    
    // Umgekehrte Z-Rotation
    let sin_z = (-rz).sin();
    let cos_z = (-rz).cos();
    let x2 = x * cos_z - y * sin_z;
    let y2 = x * sin_z + y * cos_z;
    x = x2;
    y = y2;

    // Umgekehrte Y-Rotation
    let sin_y = (-ry).sin();
    let cos_y = (-ry).cos();
    let x1 = x * cos_y - z * sin_y;
    let z1 = x * sin_y + z * cos_y;
    x = x1;
    z = z1;

    // Umgekehrte X-Rotation
    let sin_x = (-rx).sin();
    let cos_x = (-rx).cos();
    let y1 = y * cos_x - z * sin_x;
    let z2 = y * sin_x + z * cos_x;
    y = y1;
    z = z2;

    Vec3 { x, y, z }
}

pub fn reader(
    model: &Model,
    world_pos: (f32, f32, f32, f32, f32, f32), // Kamera: x, y, z, rot_x, rot_y, rot_z
    object_pos: (f32, f32, f32),              // NEU: Position des Objekts im Raum
    scale: f32,
    buffer: &mut [u32],
    zbuffer: &mut [f32],
    width: usize,
    height: usize,
    material_map: &HashMap<String, (u8, u8, u8)>,
    light_dir: Vec3,
    strake: f32,
) {
    let camera_rot = (world_pos.3, world_pos.4, world_pos.5);
    let camera_pos = (world_pos.0, world_pos.1, world_pos.2);

    for (face, mat_name) in &model.faces {
        let mut poly_points = Vec::new();
        let mut depths = Vec::new();

        for fv in face {
            let mut vertex = model.vertices[fv.vertex_index];

            // 🧠 Objekt-Position anwenden
            vertex.x += object_pos.0;
            vertex.y += object_pos.1;
            vertex.z += object_pos.2;

            // 🌍 In Kamerakoordinaten transformieren
            let v = inverse_transform(vertex, camera_rot, camera_pos);

            depths.push(v.z);
            poly_points.push(project(v, width, height, scale));
        }

        // 🎨 Basisfarbe aus Materialmap
        let base_color = material_map
            .get(mat_name)
            .map(|(r, g, b)| rgb(*r, *g, *b))
            .unwrap_or(rgb(255, 0, 255)); // Pink als Fehlerfarbe

        // 🔁 Normale des Faces (erste verfügbare)
        let face_normal = face
            .iter()
            .find_map(|fv| fv.normal_index)
            .map(|i| model.normals[i])
            .unwrap_or(Vec3 { x: 0.0, y: 0.0, z: 1.0 });

        // 🎨 Licht anwenden über Modul
        let shaded_color = apply_light(base_color, face_normal, light_dir, strake);

        draw_filled_polygon(&poly_points, &depths, buffer, zbuffer, width, height, |_z, _x, _y| shaded_color);
    }
}

fn project(v: Vec3, width: usize, height: usize, scale: f32) -> (isize, isize) {
    let aspect_ratio = width as f32 / height as f32;

    if v.z <= 0.0001 {
        return (2000, 2000); // hinter der Kamera oder zu nah
    }

    let x = (v.x / (v.z * scale)) * width as f32 / 2.0 * (1.0 / aspect_ratio * 12.0) + width as f32 / 2.0;
    let y = (v.y / (v.z * scale)) * height as f32 / 2.0 * (1.0 / aspect_ratio * 16.0) + height as f32 / 2.0;

    (x as isize, y as isize)
}

pub fn rgb(r: u8, g: u8, b: u8) -> u32 {
    (255u32 << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

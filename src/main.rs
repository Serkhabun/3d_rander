use minifb::{Key, Window, WindowOptions};
use rand::{Rng, thread_rng};
use std::f32::consts::PI;
use std::time::Instant;

mod rander;
mod logik;

use rander::rander_model::*;
use rander::partikel::*;
use rander::fps::*;
use logik::hitbox::*;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut zbuffer: Vec<f32> = vec![f32::MAX; WIDTH * HEIGHT];
    let mut world_pos = (0.0 as f32, -0.5 as f32, -5.0 as f32, 0.0 as f32, 0.0 as f32, 0.0 as f32);
    let mut window = Window::new(
        "game_for_idk",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("Fehler beim Öffnen des Fensters: {}", e);
    });
    let mut rng = thread_rng();
    // 💡 Lichtquelle – global oder konfigurierbar
    let light_dir = Vec3 { x: 0.1, y: 1.0, z: -0.1};
    let mut starke = (0.0, false);
    let mut t = starke.0;
    let mut light_color = (
        0.9 * t + 0.1, // R
        0.85 * t + 0.15, // G
        0.6 + (1.0 - t) * 0.4, // B – mehr Blau bei Nacht
    );



    // for fps
    let mut fps_counter = 0;
    let mut current_fps = 0;
    let mut last_fps_time = Instant::now();
    let mut fps_text: String;

    let cube = (
        load_obj("assets/cube.obj"),
        (0.0, 0.0, 0.0), // der pos von cube
        15.0,
        rgb(100, 170, 255),
        load_obj_hitbox("assets/cube.obj"),
        load_mtl("assets/cube.mtl"),
    );

    let plate = (
        load_obj("assets/plate.obj"),
        (5.0, 0.0, 5.0), // der pos von cube
        1.0, 
        rgb(87, 87, 87),
        load_obj_hitbox("assets/plate.obj"),
        load_mtl("assets/plate.mtl"),
    );

    let trasch = (
        load_obj("assets/trasch.obj"),
        (15.0, -1.0, 15.0), // der pos von cube
        15.0, 
        rgb(87, 87, 87),
        load_obj_hitbox("assets/trasch.obj"),
        load_mtl("assets/trasch.mtl"),
    );

    let player = load_obj_hitbox("assets/player.obj");

    // Bewegungsgeschwindigkeit
    let speed = 0.1;

    // Richtungsvektoren basierend auf Y-Rotation (world_pos.4)
    let mut dir_x = world_pos.4.cos();
    let mut dir_z = world_pos.4.sin();

    // seitliche Richtung (rechtwinklig zur Blickrichtung)
    let mut side_x = -world_pos.4.sin();
    let mut side_z = world_pos.4.cos();
    
    let mut particles_fire = partikel_lode(200, 20.0);
    let mut particles_lomm = partikel_lode(150, 20.0);

    // Haupt-Loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        zbuffer.fill(f32::MAX);
        
        // Pixel setzen (ein einfacher Farbverlauf)
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let brightness = starke.0;

                let red = ((y as f32 / HEIGHT as f32) * 255.0 * brightness) as u32;
                let green = 10;
                let blue = ((1.0 - brightness) * 100.0 + 150.0) as u32;

                buffer[y * WIDTH + x] = (red << 16) | (green << 8) | blue;
            }
        }
        let old_pos = (world_pos.0, world_pos.1, world_pos.2);

        if window.is_key_down(Key::S) {
            world_pos.0 += dir_x * speed;
            world_pos.2 += dir_z * speed;
        }
        if window.is_key_down(Key::W) {
            world_pos.0 -= dir_x * speed;
            world_pos.2 -= dir_z * speed;
        }
        if window.is_key_down(Key::A) {
            world_pos.0 -= side_x * speed;
            world_pos.2 -= side_z * speed;
        }
        if window.is_key_down(Key::D) {
            world_pos.0 += side_x * speed;
            world_pos.2 += side_z * speed;
        }

        if window.is_key_down(Key::Space) {
            world_pos.1 += speed;
        }
        if window.is_key_down(Key::Q) {
            world_pos.1 -= speed;
        }

        if window.is_key_down(Key::Left) {
            world_pos.4 += 0.1; 
        }
        if window.is_key_down(Key::Right) {
            world_pos.4 -= 0.1;
        }

        if window.is_key_down(Key::Down) {
            world_pos.3 += 0.1; 
        }
        if window.is_key_down(Key::Up) {
            world_pos.3 -= 0.1;
        }

        if world_pos.4 > PI {
            world_pos.4 -= 2.0 * PI;
        }
        if world_pos.4 < -PI {
            world_pos.4 += 2.0 * PI;
        } 

        if world_pos.5 > PI {
            world_pos.5 -= 2.0 * PI;
        }
        if world_pos.5 < -PI {
            world_pos.5 += 2.0 * PI;
        } 

        // Richtungsvektoren basierend auf Y-Rotation (world_pos.4)
        dir_x = world_pos.4.sin();
        dir_z = -world_pos.4.cos();

        // seitliche Richtung (rechtwinklig zur Blickrichtung)
        side_x = world_pos.4.cos();
        side_z = world_pos.4.sin();


        if window.is_key_down(Key::F) {
            particles_fire.extend(partikel_lode(250, 40.0).into_iter().map(|mut p| {
                // Ursprung bei Spieler
                p.x = world_pos.0;
                p.y = world_pos.1;
                p.z = world_pos.2+1.0;
                p
            }));
        }


        if let (Some(player_hitbox), Some(cube_hitbox)) = &(&player, &cube.4) {
            let kollidiert = check_aabb_collision(
                player_hitbox,
                cube_hitbox,
                (world_pos.0, world_pos.1, world_pos.2),  // Position des Würfels
                cube.1 // Position der Platte
            );

            if kollidiert {
                world_pos.0 = old_pos.0;
                world_pos.1 = old_pos.1;
                world_pos.2 = old_pos.2;
            }
        }

        if let (Some(player_hitbox), Some(trasch_hitbox)) = &(&player, &trasch.4) {
            let kollidiert = check_aabb_collision(
                player_hitbox,
                trasch_hitbox,
                (world_pos.0, world_pos.1, world_pos.2),  // Position des Würfels
                trasch.1 // Position der Platte
            );

            if kollidiert {
                world_pos.0 = old_pos.0;
                world_pos.1 = old_pos.1;
                world_pos.2 = old_pos.2;

                particles_lomm.extend(partikel_lode(150, 20.0).into_iter().map(|mut p| {
                    // Ursprung bei Spieler
                    p.x = trasch.1.0;
                    p.y = trasch.1.1;
                    p.z = trasch.1.2+1.0;
                    p
                }));
            }
        }

        // Buffer anzeigen
        reader(&cube.0, world_pos, cube.1, cube.2, &mut buffer, &mut zbuffer, WIDTH, HEIGHT, &cube.5, light_dir, starke.0);
        reader(&plate.0, world_pos, plate.1, plate.2, &mut buffer, &mut zbuffer, WIDTH, HEIGHT, &plate.5, light_dir, starke.0);
        reader(&trasch.0, world_pos, trasch.1, trasch.2, &mut buffer, &mut zbuffer, WIDTH, HEIGHT, &trasch.5, light_dir, starke.0);
        rander_partikel(
            &mut particles_fire,
            &mut buffer,
            &mut zbuffer,
            WIDTH,
            HEIGHT,
            (world_pos.0, world_pos.1, world_pos.2),
            (world_pos.4, world_pos.3), // yaw, pitch
            (60.0, 60.0, 60.0),
            255.0,
            100.0,
            50.0,
        );
        rander_partikel(
            &mut particles_lomm,
            &mut buffer,
            &mut zbuffer,
            WIDTH,
            HEIGHT,
            (world_pos.0, world_pos.1, world_pos.2),
            (world_pos.4, world_pos.3), // yaw, pitch
            (15.0, 15.0, 15.0),
            50.0,
            100.0,
            255.0,
        );

        // for fps
        fps_counter += 1;
        if last_fps_time.elapsed().as_secs_f32() >= 1.0 {
            current_fps = fps_counter;
            fps_counter = 0;
            last_fps_time = Instant::now();
        }
        fps_text = format!("FPS: {}", current_fps);

        if starke.0 < 0.0 {
            starke.1 = false;
        }
        if starke.0 > 1.0 {
            starke.1 = true;
        }
        if starke.1 == true {
            starke.0 -= 0.0005;
        }
        if starke.1 == false {
            starke.0 += 0.0005;
        }

        t = starke.0;
        light_color = (
            0.9 * t + 0.1, // R
            0.85 * t + 0.15, // G
            0.6 + (1.0 - t) * 0.4, // B – mehr Blau bei Nacht
        );

        draw_Fps(10, 10, HEIGHT, WIDTH, &fps_text, 0xFFFFFF, &mut buffer);

        window.
            update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}

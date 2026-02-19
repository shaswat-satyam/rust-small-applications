use macroquad::prelude::*;

#[macroquad::main("3D")]
async fn main() {
    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;
    let mut z: f32 = 0.0;

    let mut yaw: f32 = 0.0;
    let mut pitch: f32 = 0.3;
    let delta: f32 = 0.02;

    let mut radius: f32 = 50.0;

    loop {
        clear_background(BLACK);

        if is_key_down(KeyCode::L) { yaw += delta; }
        if is_key_down(KeyCode::J) { yaw -= delta; }

        if is_key_down(KeyCode::I) { pitch += delta; }
        if is_key_down(KeyCode::K) { pitch -= delta; }

        if is_key_down(KeyCode::U) { radius += 1.0; }
        if is_key_down(KeyCode::O) { radius -= 1.0; }
        radius = radius.max(5.0);
        if pitch < -std::f32::consts::PI/2. {
            pitch = std::f32::consts::PI/2.;
        } else if pitch > std::f32::consts::PI/2. {
            pitch = -std::f32::consts::PI/2.;
        }

        if is_key_down(KeyCode::W) { x += 1.0; }
        if is_key_down(KeyCode::S) { x -= 1.0; }
        if is_key_down(KeyCode::A) { z -= 1.0; }
        if is_key_down(KeyCode::D) { z += 1.0; }
        if is_key_down(KeyCode::Q) { y -= 1.0; }
        if is_key_down(KeyCode::E) { y += 1.0; }

        if is_key_pressed(KeyCode::Space) {
            x = 0.0;
            y = 0.0;
            z = 0.0;
            yaw = 0.0;
            pitch = 0.3;
            radius = 50.0;
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        let cam_x = radius * pitch.cos() * yaw.sin();
        let cam_y = radius * pitch.sin();
        let cam_z = radius * pitch.cos() * yaw.cos();

        set_camera(&Camera3D {
            position: vec3(cam_x + x, cam_y + y, cam_z + z),
            target: vec3(0., 0., 0.),   
            up: vec3(0.0, 1.0, 0.0),
            ..Default::default()
        });

        draw_cube_wires(vec3(x, y, z), vec3(10., 10., 10.), WHITE);

        next_frame().await
    }
}

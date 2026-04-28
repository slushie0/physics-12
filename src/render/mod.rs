use crate::world::GameState;
use macroquad::prelude::*;

pub mod camera;

pub fn draw(world: &GameState) {
    clear_background(BLACK);

    for body in world.bodies.iter() {
        //let (sx, sy) = camera.relative_to_screen(rel_x, rel_y);
        let screen_radius = (body.radius * 1e-7/*camera.linear_zoom_f64()*/) as f32;

        let max_chord_error = 0.25_f32;
        let segments = if screen_radius > 1.0 {
            let n = std::f32::consts::PI / (1.0 - max_chord_error / screen_radius).acos();
            (n.ceil() as u32).clamp(8, 255) as u8
        } else {
            8
        };

        draw_circle(body.pos.x as f32, body.pos.y as f32, body.radius as f32, body.color);
    }
}

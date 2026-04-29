use egui_macroquad::egui::{self, InnerResponse, Widget};
use egui_macroquad::egui::{Color32, Pos2, Stroke};
use std::default;

use crate::render::camera::CameraController;
use crate::world::{Body, GameState};
use macroquad::prelude::*;

pub(crate) mod camera;

/*
pub fn draw(world: &GameState) {
    //clear_background(BLACK);
    draw_rectangle(0., 0., screen_width(), screen_height(), Color::new(0.0, 0.0, 0.0, 0.1));

    for body in world.bodies.iter() {
        draw_circle(body.pos.x as f32, body.pos.y as f32, body.radius as f32, body.color);
    }
}
*/
pub fn draw(world: &GameState, camera: &mut CameraController, material: &mut Material) {
    clear_background(BLACK);
    material.set_uniform("u_resolution", (screen_width(), screen_height()));
    for body in world.bodies.iter() {
        /*draw_circle(
            body.pos.x as f32,
            body.pos.y as f32,
            body.radius as f32,
            body.color,
        );*/
        draw_body(body, camera, material);
    }

    egui_macroquad::ui(|ctx| {
        egui::Window::new("Debug Panel").show(ctx, |ui| {
            ui.label("Select camera target:");

            egui::ComboBox::from_id_salt(123)
                .selected_text(&world.bodies[camera.target_id as usize].name)
                .show_ui(ui, |ui| {
                    for (i, body) in world.bodies.iter().enumerate() {
                        if ui
                            .selectable_value(&mut camera.target_id, i, &body.name)
                            .clicked()
                        {
                            camera.offset = crate::world::Vec2::ZERO;
                        }
                    }
                });
            ui.end_row();
            egui::Checkbox::new(&mut camera.show_labels, "Show Labels").ui(ui);
        });
        egui::Area::new("scale_bar_area".into())
            .anchor(egui::Align2::LEFT_BOTTOM, egui::vec2(0.0, 0.0))
            .show(ctx, |ui| {
                draw_scale_bar(ui, camera.zoom());
            });
    });

    // --- Draw egui on top ---
    egui_macroquad::draw();
}

pub fn draw_body(body: &Body, camera: &CameraController, material: &mut Material) {
    let screen_coords = camera.world_to_screen(body.pos.clone());
    // Keep calculations in f64 as long as possible for precision
    let screen_radius_f64 = body.radius * camera.zoom() as f64;
    let screen_width_f64 = screen_width() as f64;
    let screen_height_f64 = screen_height() as f64;

    // Calculate center offsets with better precision handling
    let center_x = (screen_coords.x as f64) - screen_width_f64 * 0.5;
    let center_y = (screen_coords.y as f64) - screen_height_f64 * 0.5;

    material.set_uniform("u_center", (center_x as f32, center_y as f32));
    material.set_uniform("u_radius", (screen_radius_f64 * 10.0) as f32);
    material.set_uniform(
        "u_color",
        (body.color.r, body.color.g, body.color.b, body.color.a),
    );

    gl_use_material(&material);
    draw_rectangle(0.0, 0.0, screen_width(), screen_height(), WHITE);
    gl_use_default_material();

    if camera.show_labels {
        draw_text(&body.name, screen_coords.x, screen_coords.y, 16., WHITE);
    }
}

pub fn draw_scale_bar(ui: &mut egui::Ui, linear_zoom: f64) {
    let rect = ui.available_rect_before_wrap();
    let painter = ui.painter();

    let target_pixels = 150.0;

    let world_per_pixel = 1.0 / linear_zoom;
    let raw_world = world_per_pixel * target_pixels;

    let magnitude = 10_f64.powf(raw_world.log10().floor());
    let nice = [1.0, 2.0, 5.0, 10.0]
        .into_iter()
        .map(|x| x * magnitude)
        .find(|x| x * linear_zoom >= target_pixels)
        .unwrap_or(magnitude);

    let bar_pixels = nice * linear_zoom;

    let (value, unit) = format_distance(nice as f32);
    let label = format!("{:.2} {}", value, unit);

    // Layout (bottom-left like your original)
    let margin = 20.0;
    let y = rect.bottom() - margin;
    let x_start = rect.left() + margin;
    let x_end = x_start + bar_pixels as f32;
    let tick_h = 6.0;

    // Main line
    painter.line_segment(
        [Pos2::new(x_start, y), Pos2::new(x_end, y)],
        Stroke::new(2.0, Color32::WHITE),
    );

    // ticks
    painter.line_segment(
        [
            Pos2::new(x_start, y - tick_h),
            Pos2::new(x_start, y + tick_h),
        ],
        Stroke::new(2.0, Color32::WHITE),
    );

    painter.line_segment(
        [Pos2::new(x_end, y - tick_h), Pos2::new(x_end, y + tick_h)],
        Stroke::new(2.0, Color32::WHITE),
    );

    // centered text
    painter.text(
        Pos2::new(x_start + bar_pixels as f32 * 0.5, y - tick_h - 4.0),
        egui::Align2::CENTER_BOTTOM,
        label,
        egui::FontId::proportional(16.0),
        Color32::WHITE,
    );
}

fn format_distance(meters: f32) -> (f32, &'static str) {
    const AU: f32 = 1.496e11;

    if meters < 1e-9 {
        (meters * 1e9, "nm")
    } else if meters < 1e-6 {
        (meters * 1e6, "µm")
    } else if meters < 1e-3 {
        (meters * 1e3, "mm")
    } else if meters < 1.0 {
        (meters, "m")
    } else if meters < 1e3 {
        (meters, "m")
    } else if meters < 1e6 {
        (meters / 1e3, "km")
    } else if meters < 1e9 {
        (meters / 1e6, "Mm")
    } else if meters < AU {
        (meters / 1e9, "Gm")
    } else {
        (meters / AU, "AU")
    }
}
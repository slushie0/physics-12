use macroquad::prelude::*;

use crate::world::{self, Body, GameState};

const SCROLL_SENSITIVITY: f64 = 0.005;

pub struct CameraController {
    pub target_id: usize,
    pub target: world::Vec2,
    pub offset: world::Vec2,
    pub zoom: f64, // <- store zoom in log space

    pub show_labels: bool,

    is_dragging: bool,
    last_mouse: world::Vec2,
}

impl CameraController {
    pub fn new(target_id: usize, zoom: f64) -> Self {
        Self {
            target_id,
            target: world::Vec2::ZERO,
            offset: world::Vec2::ZERO,
            zoom,

            show_labels: false,

            is_dragging: false,
            last_mouse: world::Vec2::ZERO,
        }
    }

    pub fn zoom(&self) -> f64 {
        self.zoom.exp()
    }

    pub fn update(&mut self, world: &GameState) {
        let mouse: Vec2 = mouse_position().into();
        let mouse_f64 = world::Vec2::from_components(mouse.x as f64, mouse.y as f64);

        // --- PAN ---
        if is_mouse_button_pressed(MouseButton::Left) {
            self.is_dragging = true;
            self.last_mouse = mouse_f64.clone();
        }

        if is_mouse_button_released(MouseButton::Left) {
            self.is_dragging = false;
        }

        if self.is_dragging {
            let delta = world::Vec2::from_components(
                mouse_f64.x - self.last_mouse.x,
                mouse_f64.y - self.last_mouse.y,
            );
            self.offset = world::Vec2::from_components(
                self.offset.x - delta.x / self.zoom(),
                self.offset.y - delta.y / self.zoom(),
            );
            self.last_mouse = mouse_f64.clone();
        }

        // --- ZOOM ---
        let (_x, scroll_y) = mouse_wheel();

        if scroll_y != 0.0 {

            self.zoom += scroll_y as f64 * SCROLL_SENSITIVITY;
            let zoom_before = self.zoom();

            // adjust log zoom directly
            let zoom_speed = 0.15;
            //self.log_zoom += scroll_y * zoom_speed;
            //self.log_zoom = self.log_zoom.clamp(self.min_log_zoom, self.max_log_zoom);

            let zoom_after = self.zoom();

            // --- zoom toward mouse (NO matrices) ---
            let screen_center_f64 = world::Vec2::from_components(
                screen_center().x as f64,
                screen_center().y as f64,
            );
            
            let mouse_world_before = world::Vec2::from_components(
                self.offset.x + (mouse_f64.x - screen_center_f64.x) / zoom_before,
                self.offset.y + (mouse_f64.y - screen_center_f64.y) / zoom_before,
            );

            let mouse_world_after = world::Vec2::from_components(
                self.offset.x + (mouse_f64.x - screen_center_f64.x) / zoom_after,
                self.offset.y + (mouse_f64.y - screen_center_f64.y) / zoom_after,
            );

            self.offset = world::Vec2::from_components(
                self.offset.x + mouse_world_before.x - mouse_world_after.x,
                self.offset.y + mouse_world_before.y - mouse_world_after.y,
            );
        }
        let target_body = world.bodies[self.target_id].clone();
        self.target = world::Vec2::from_components(target_body.pos.x + self.offset.x, target_body.pos.y + self.offset.y);
    }

    pub fn camera(&mut self) -> Camera2D {
        let zoom = self.zoom();

        Camera2D {
            target: self.target.to_glam_f32(),
            zoom: vec2(
                zoom as f32 * 2.0 / screen_width(),
                zoom as f32 * 2.0 / screen_height(),
            ),
            ..Default::default()
        }
    }

    pub fn world_to_screen(&self, world_pos: world::Vec2) -> Vec2 {
        let zoom = self.zoom();
        let relative_pos = world::Vec2::from_components(
            world_pos.x - self.target.x,
            world_pos.y - self.target.y,
        );
        
        let screen_center_f64 = world::Vec2::from_components(
            screen_center().x as f64,
            screen_center().y as f64,
        );
        
        let screen_x = screen_center_f64.x + relative_pos.x * zoom;
        let screen_y = screen_center_f64.y + relative_pos.y * zoom;
        
        vec2(screen_x as f32, screen_y as f32)
    }
}

fn screen_center() -> Vec2 {
    vec2(screen_width() * 0.5, screen_height() * 0.5)
}
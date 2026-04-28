use macroquad::prelude::*;

const SCROLL_SENSITIVITY: f64 = 0.005;

pub struct CameraController {
    pub target: Vec2,

    zoom: f64, // <- store zoom in log space

    is_dragging: bool,
    last_mouse: Vec2,
}

impl CameraController {
    pub fn new(zoom: f64) -> Self {
        Self {
            target: vec2(0.0, 0.0),

            zoom: zoom,

            is_dragging: false,
            last_mouse: vec2(0.0, 0.0),
        }
    }

    fn zoom(&self) -> f32 {
        self.zoom.exp() as f32
    }

    pub fn update(&mut self) {
        let mouse: Vec2 = mouse_position().into();

        // --- PAN ---
        if is_mouse_button_pressed(MouseButton::Left) {
            self.is_dragging = true;
            self.last_mouse = mouse;
        }

        if is_mouse_button_released(MouseButton::Left) {
            self.is_dragging = false;
        }

        if self.is_dragging {
            let delta = mouse - self.last_mouse;
            self.target -= delta / self.zoom();
            self.last_mouse = mouse;
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
            let mouse_world_before =
                self.target + (mouse - screen_center()) / zoom_before;

            let mouse_world_after =
                self.target + (mouse - screen_center()) / zoom_after;

            self.target += mouse_world_before - mouse_world_after;
        }
    }

    pub fn camera(&self) -> Camera2D {
        let zoom = self.zoom();

        Camera2D {
            target: self.target,
            zoom: vec2(
                zoom * 2.0 / screen_width(),
                zoom * 2.0 / screen_height(),
            ),
            ..Default::default()
        }
    }
}

fn screen_center() -> Vec2 {
    vec2(screen_width() * 0.5, screen_height() * 0.5)
}
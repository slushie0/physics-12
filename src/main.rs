use macroquad::prelude::*;
use macroquad::window::Conf;

use crate::world::{Body, Vec2};
use crate::render::camera::CameraController;

mod render;
mod sim;
mod world;

fn window_conf() -> Conf {
    Conf {
        window_title: "Physics 12 Simulator".to_string(),
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    /*let mut world = world::GameState {
        bodies: vec![
            Body {
                name: "Earth".to_string(),
                pos: Vec2 { x: 0., y: 0. },
                vel: Vec2 {
                    x: 0.,
                    y: -(7.35e22 * 1022.0) / 5.98e24,
                }, // Earth recoil so total momentum == 0
                mass: 5.98e24,
                radius: 6.38e6,
                charge: 0.0,
                color: BLUE,
            }, // Earth
            Body {
                name: "Moon".to_string(),
                pos: Vec2 { x: 3.844e8, y: 0. },
                vel: Vec2 { x: 0., y: 1022.0 },
                mass: 7.35e22,
                radius: 1.74e6,
                charge: 0.0,
                color: GRAY,
            }, // Moon
        ],
    };*/
    let mut world = world::GameState {
        //3 body problem
        bodies: vec![
            Body { name: "1".to_string(), pos: Vec2 { x: 0.97000436e9, y: -0.24308753e9 }, vel: Vec2 { x: 0.4662036850 * 631.5, y: 0.4323657300 * 631.5 }, mass: 5.972e24, radius: 1e8, charge: 0., color: RED },
            Body { name: "2".to_string(), pos: Vec2 { x: -0.97000436e9, y: 0.24308753e9 }, vel: Vec2 { x: 0.4662036850 * 631.5, y: 0.4323657300 * 631.5 }, mass: 5.972e24, radius: 1e8, charge: 0., color: GREEN },
            Body { name: "3".to_string(), pos: Vec2 { x: 0.0, y: 0.0 }, vel: Vec2 { x: -0.93240737 * 631.5, y: -0.86473146 * 631.5 }, mass: 5.972e24, radius: 1e8, charge: 0., color: BLUE },
        ],
    };

    let mut camera = CameraController::new(-17.);

    loop {
        sim::tick(&mut world, 50000.0_f64);

        camera.update();
        set_camera(&camera.camera());

        render::draw(&world);
        next_frame().await;
    }
}

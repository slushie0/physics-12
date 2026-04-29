use macroquad::prelude::*;
use macroquad::window::Conf;

use crate::render::camera::CameraController;
use crate::world::{Body, Vec2};

mod render;
mod sim;
mod world;

const VERTEX_SHADER: &str = include_str!("render/vert.glsl");
const FRAGMENT_SHADER: &str = include_str!("render/frag.glsl");

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
                id: 1,
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
                id: 2,
                name: "Moon".to_string(),
                pos: Vec2 { x: 3.844e8, y: 0. },
                vel: Vec2 { x: 0., y: 1022.0 },
                mass: 7.35e22,
                radius: 1.74e6,
                charge: 0.0,
                color: GRAY,
            }, // Moon
        ],
    };
    let mut camera = CameraController::new(0, -14.);

    let mut world = world::GameState {
        //3 body problem
        bodies: vec![
            Body { id: 1, name: "Origin".to_string(), pos: Vec2::ZERO, vel: Vec2::ZERO, mass: 0., radius: 100., charge: 0., color: YELLOW},
            Body { id: 2, name: "1".to_string(), pos: Vec2 { x: 0.97000436e9, y: -0.24308753e9 }, vel: Vec2 { x: 0.4662036850 * 631.5, y: 0.4323657300 * 631.5 }, mass: 5.972e24, radius: 1e8, charge: 0., color: RED },
            Body { id: 3, name: "2".to_string(), pos: Vec2 { x: -0.97000436e9, y: 0.24308753e9 }, vel: Vec2 { x: 0.4662036850 * 631.5, y: 0.4323657300 * 631.5 }, mass: 5.972e24, radius: 1e8, charge: 0., color: GREEN },
            Body { id: 4, name: "3".to_string(), pos: Vec2 { x: 0.0, y: 0.0 }, vel: Vec2 { x: -0.93240737 * 631.5, y: -0.86473146 * 631.5 }, mass: 5.972e24, radius: 1e8, charge: 0., color: BLUE },
        ],
    };
    let mut camera = CameraController::new(0, -16.);*/
    let mut world = world::GameState {
        bodies: vec![
            Body {
                id: 0,
                name: "Sun".to_string(),

                pos: Vec2 {
                    x: -8.572865e+08,
                    y: -7.346089e+08,
                },
                vel: Vec2 {
                    x: 1.239416e+01,
                    y: -6.346203e+00,
                },

                mass: 1.989000e+30,
                radius: 6.963400e+08,

                charge: 0.0,
                color: YELLOW,
            },
            Body {
                id: 1,
                name: "Mercury".to_string(),

                pos: Vec2 {
                    x: -5.879699e+10,
                    y: -2.492820e+10,
                },
                vel: Vec2 {
                    x: 8.711978e+03,
                    y: -4.284857e+04,
                },

                mass: 3.301000e+23,
                radius: 2.440000e+06,

                charge: 0.0,
                color: GRAY,
            },
            Body {
                id: 2,
                name: "Venus".to_string(),

                pos: Vec2 {
                    x: 6.697320e+10,
                    y: 8.337172e+10,
                },
                vel: Vec2 {
                    x: -2.735549e+04,
                    y: 2.182743e+04,
                },

                mass: 4.867000e+24,
                radius: 6.052000e+06,

                charge: 0.0,
                color: ORANGE,
            },
            Body {
                id: 3,
                name: "Earth".to_string(),

                pos: Vec2 {
                    x: -2.758795e+10,
                    y: 1.439240e+11,
                },
                vel: Vec2 {
                    x: -2.977687e+04,
                    y: -5.535811e+03,
                },

                mass: 5.972000e+24,
                radius: 6.371000e+06,

                charge: 0.0,
                color: BLUE,
            },
            Body {
                id: 4,
                name: "Moon".to_string(),

                pos: Vec2 {
                    x: -2.743590e+10,
                    y: 1.435752e+11,
                },
                vel: Vec2 {
                    x: -2.884424e+04,
                    y: -5.089319e+03,
                },

                mass: 7.342000e+22,
                radius: 1.737000e+06,

                charge: 0.0,
                color: LIGHTGRAY,
            },
            Body {
                id: 5,
                name: "Mars".to_string(),

                pos: Vec2 {
                    x: -7.890038e+10,
                    y: 2.274372e+11,
                },
                vel: Vec2 {
                    x: -2.199760e+04,
                    y: -5.787403e+03,
                },

                mass: 6.417000e+23,
                radius: 3.389000e+06,

                charge: 0.0,
                color: RED,
            },
            Body {
                id: 6,
                name: "Phobos".to_string(),

                pos: Vec2 {
                    x: -7.889662e+10,
                    y: 2.274455e+11,
                },
                vel: Vec2 {
                    x: -2.373508e+04,
                    y: -4.849854e+03,
                },

                mass: 1.065900e+16,
                radius: 1.100000e+04,

                charge: 0.0,
                color: GRAY,
            },
            Body {
                id: 7,
                name: "Deimos".to_string(),

                pos: Vec2 {
                    x: -7.887971e+10,
                    y: 2.274441e+11,
                },
                vel: Vec2 {
                    x: -2.232556e+04,
                    y: -4.498659e+03,
                },

                mass: 1.476200e+15,
                radius: 6.200000e+03,

                charge: 0.0,
                color: GRAY,
            },
            Body {
                id: 8,
                name: "Jupiter".to_string(),

                pos: Vec2 {
                    x: 1.571231e+11,
                    y: 7.429840e+11,
                },
                vel: Vec2 {
                    x: -1.293245e+04,
                    y: 3.325784e+03,
                },

                mass: 1.898000e+27,
                radius: 6.991100e+07,

                charge: 0.0,
                color: BROWN,
            },
            Body {
                id: 9,
                name: "Io".to_string(),

                pos: Vec2 {
                    x: 1.567913e+11,
                    y: 7.427243e+11,
                },
                vel: Vec2 {
                    x: -2.289631e+03,
                    y: -1.036623e+04,
                },

                mass: 8.930000e+22,
                radius: 1.821000e+06,

                charge: 0.0,
                color: YELLOW,
            },
            Body {
                id: 10,
                name: "Europa".to_string(),

                pos: Vec2 {
                    x: 1.577881e+11,
                    y: 7.429454e+11,
                },
                vel: Vec2 {
                    x: -1.205079e+04,
                    y: 1.712933e+04,
                },

                mass: 4.800000e+22,
                radius: 1.560000e+06,

                charge: 0.0,
                color: WHITE,
            },
            Body {
                id: 11,
                name: "Ganymede".to_string(),

                pos: Vec2 {
                    x: 1.580935e+11,
                    y: 7.425362e+11,
                },
                vel: Vec2 {
                    x: -8.370296e+03,
                    y: 1.321210e+04,
                },

                mass: 1.480000e+23,
                radius: 2.634000e+06,

                charge: 0.0,
                color: GRAY,
            },
            Body {
                id: 12,
                name: "Callisto".to_string(),

                pos: Vec2 {
                    x: 1.557608e+11,
                    y: 7.442947e+11,
                },
                vel: Vec2 {
                    x: -1.862956e+04,
                    y: -2.522418e+03,
                },

                mass: 1.080000e+23,
                radius: 2.410000e+06,

                charge: 0.0,
                color: DARKGRAY,
            },
            Body {
                id: 13,
                name: "Saturn".to_string(),

                pos: Vec2 {
                    x: 1.414498e+12,
                    y: -2.647172e+11,
                },
                vel: Vec2 {
                    x: 1.240656e+03,
                    y: 9.473549e+03,
                },

                mass: 5.683000e+26,
                radius: 5.823200e+07,

                charge: 0.0,
                color: YELLOW,
            },
            Body {
                id: 14,
                name: "Titan".to_string(),

                pos: Vec2 {
                    x: 1.415601e+12,
                    y: -2.643687e+11,
                },
                vel: Vec2 {
                    x: -7.860499e+02,
                    y: 1.429595e+04,
                },

                mass: 1.345000e+23,
                radius: 2.575000e+06,

                charge: 0.0,
                color: ORANGE,
            },
            Body {
                id: 15,
                name: "Enceladus".to_string(),

                pos: Vec2 {
                    x: 1.414654e+12,
                    y: -2.645640e+11,
                },
                vel: Vec2 {
                    x: -8.296985e+03,
                    y: 1.714280e+04,
                },

                mass: 1.080000e+20,
                radius: 2.520000e+05,

                charge: 0.0,
                color: WHITE,
            },
            Body {
                id: 16,
                name: "Uranus".to_string(),

                pos: Vec2 {
                    x: 1.660222e+12,
                    y: 2.406966e+12,
                },
                vel: Vec2 {
                    x: -5.655925e+03,
                    y: 3.549249e+03,
                },

                mass: 8.681000e+25,
                radius: 2.536200e+07,

                charge: 0.0,
                color: DARKBLUE,
            },
            Body {
                id: 17,
                name: "Titania".to_string(),

                pos: Vec2 {
                    x: 1.660443e+12,
                    y: 2.406867e+12,
                },
                vel: Vec2 {
                    x: -8.701154e+03,
                    y: 3.941379e+03,
                },

                mass: 3.400000e+21,
                radius: 7.890000e+05,

                charge: 0.0,
                color: GRAY,
            },
            Body {
                id: 18,
                name: "Oberon".to_string(),

                pos: Vec2 {
                    x: 1.660130e+12,
                    y: 2.407067e+12,
                },
                vel: Vec2 {
                    x: -2.619041e+03,
                    y: 2.972053e+03,
                },

                mass: 3.000000e+21,
                radius: 7.610000e+05,

                charge: 0.0,
                color: GRAY,
            },
            Body {
                id: 19,
                name: "Neptune".to_string(),

                pos: Vec2 {
                    x: 4.469116e+12,
                    y: -9.560778e+10,
                },
                vel: Vec2 {
                    x: 8.064118e+01,
                    y: 5.465732e+03,
                },

                mass: 1.024000e+26,
                radius: 2.462200e+07,

                charge: 0.0,
                color: BLUE,
            },
            Body {
                id: 20,
                name: "Triton".to_string(),

                pos: Vec2 {
                    x: 4.468831e+12,
                    y: -9.581258e+10,
                },
                vel: Vec2 {
                    x: -1.162885e+03,
                    y: 8.021338e+03,
                },

                mass: 2.140000e+22,
                radius: 1.353000e+06,

                charge: 0.0,
                color: DARKBLUE,
            },
        ],
        delta_time: 1.0,
        steps_per_frame: 1000,
    };
    let mut camera = CameraController::new(0, -20.);

    let mut material = load_material(
        ShaderSource::Glsl {
            vertex: VERTEX_SHADER,
            fragment: FRAGMENT_SHADER,
        },
        MaterialParams {
            uniforms: vec![
                UniformDesc::new("u_resolution", UniformType::Float2),
                UniformDesc::new("u_center", UniformType::Float2),
                UniformDesc::new("u_radius", UniformType::Float1),
                UniformDesc::new("u_color", UniformType::Float4),
                //UniformDesc::new("u_zoom", UniformType::Float1),
            ],
            ..Default::default()
        },
    )
    .unwrap();

    loop {
        for i in 0..world.steps_per_frame {
            sim::tick(&mut world);
        }

        camera.update(&world);
        //set_camera(&camera.camera());

        render::draw(&mut world, &mut camera, &mut material);
        next_frame().await;
    }
}

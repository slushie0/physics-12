use crate::world::*;

mod gravity;
mod charge;

pub fn tick(world: &mut GameState, delta_time: f64) {
    let forces = accumulate_forces(&world.bodies);

    for i in 0..forces.len() {
        world.bodies[i].vel.x += (forces[i].x / world.bodies[i].mass) * delta_time;
        world.bodies[i].vel.y += (forces[i].y / world.bodies[i].mass) * delta_time;

        world.bodies[i].pos.x += world.bodies[i].vel.x * delta_time;
        world.bodies[i].pos.y += world.bodies[i].vel.y * delta_time;
    }
}

fn accumulate_forces(bodies: &[Body]) -> Vec<Force> {
    let mut forces: Vec<Force> = vec![Force::from_components(0., 0.); bodies.len()];

    // Pairwise interactions (gravity, springs, etc.)
    for i in 0..bodies.len() {
        for j in (i + 1)..bodies.len() {
            let Fg = gravity::compute(&bodies[i], &bodies[j]);
            let Fe = charge::compute(&bodies[i], &bodies[j]);
            forces[i] -= Fg.clone() - Fe.clone();
            forces[j] += Fg + Fe; // Newton's 3rd law
        }
    }

    // External/unary forces (drag, buoyancy, etc.)
    for (i, body) in bodies.iter().enumerate() {
        //forces[i] += drag(body);
    }

    forces
}

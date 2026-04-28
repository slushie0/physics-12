use crate::world::*;

pub fn compute(body1: &Body, body2: &Body) -> Force {
    let mut dist = distance(&body1, &body2);
    if dist < body1.radius + body2.radius {
        dist = body1.radius + body2.radius; // prevent division by very small numbers
    }
    let force_mag = (Ke * body1.charge * body2.charge) / dist.powf(2.);

    let dx = body1.pos.x - body2.pos.x;
    let dy = body1.pos.y - body2.pos.y;

    Force::from_components(force_mag * dx / dist, force_mag * dy / dist)
}

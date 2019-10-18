use specs::prelude::*;

use crate::game::components::*;

pub struct MoveGameObjects {}

impl<'a> System<'a> for MoveGameObjects {
    type SystemData = (
        WriteStorage<'a, Geometry>,
        ReadStorage<'a, Velocity>
    );

    fn run(&mut self, (mut geometries, velocities): Self::SystemData) {
        for (geometry, velocity) in (&mut geometries, &velocities).join() {
            geometry.x += velocity.0;
            geometry.y += velocity.1;
        }
    }
}
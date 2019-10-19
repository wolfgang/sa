use std::cmp::{max, min};

use specs::{join::Join, Read, ReadStorage, System, WriteStorage};

use crate::game::builder::GameConfig;
use crate::game::components::{Geometry, IsEnemy, Velocity};

pub struct HandleEnemyMovement;

impl<'a> System<'a> for HandleEnemyMovement {
    type SystemData = (
        ReadStorage<'a, IsEnemy>,
        WriteStorage<'a, Geometry>,
        WriteStorage<'a, Velocity>,
        Read<'a, GameConfig>
    );

    fn run(&mut self, (enemies, mut geometries, mut velocities, config): Self::SystemData) {
        for (_, geometry, velocity) in (&enemies, &mut geometries, &mut velocities).join() {
            let max_x = (config.dimensions.0 - geometry.width) as i32;
            if geometry.x >= max_x || geometry.x <= 0 {
                velocity.0 *= -1;
                geometry.x = max(0, min(max_x, geometry.x));
            }
        }
    }
}
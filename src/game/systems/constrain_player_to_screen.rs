use std::cmp::{max, min};

use specs::prelude::*;

use crate::game::builder::GameConfig;
use crate::game::components::*;

pub struct ConstrainPlayerToScreen {}

impl<'a> System<'a> for ConstrainPlayerToScreen {
    type SystemData = (
        WriteStorage<'a, Geometry>,
        ReadStorage<'a, IsPlayer>,
        Read<'a, GameConfig>
    );

    fn run(&mut self, (mut gos, players, config): Self::SystemData) {
        for (geometry, _) in (&mut gos, &players).join() {
            let max_x = (config.dimensions.0 - geometry.width) as i32;
            geometry.x = max(0, min(geometry.x, max_x));
        }
    }
}

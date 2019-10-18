use specs::prelude::*;

use crate::game::builder::GameConfig;
use crate::game::components::*;
use crate::game::GameState;

pub struct HandlePlayerMovement {}

impl<'a> System<'a> for HandlePlayerMovement {
    type SystemData = (
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, IsPlayer>,
        Read<'a, GameConfig>,
        Read<'a, GameState>
    );


    fn run(&mut self, (mut velocities, players, config, game_state): Self::SystemData) {
        for (velocity, _) in (&mut velocities, &players).join() {
            if game_state.moving_left {
                velocity.0 = config.ship_speed as i32 * -1;
            } else if game_state.moving_right {
                velocity.0 = config.ship_speed as i32;
            } else {
                velocity.0 = 0;
            }
        }
    }
}
use raylib::consts::*;
use specs::prelude::*;

use crate::game::builder::GameConfig;
use crate::game::components::*;
use crate::game::GameState;
use crate::game::input::InputRef;

pub struct HandlePlayerInput { input: InputRef }

impl HandlePlayerInput {
    pub fn new(input: InputRef) -> Self {
        Self { input }
    }
}

impl<'a> System<'a> for HandlePlayerInput {
    type SystemData = (
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Geometry>,
        ReadStorage<'a, IsPlayer>,
        Read<'a, GameConfig>,
        Read<'a, LazyUpdate>,
        Entities<'a>,
        Write<'a, GameState>
    );


    fn run(&mut self, (
        mut velocities,
        gos,
        players,
        config,
        updater,
        entities,
        mut game_state): Self::SystemData)
    {
        for (velocity, player_geom, _) in (&mut velocities, &gos, &players).join() {
            if self.input.borrow().is_key_down(KEY_SPACE) {
                if game_state.current_tick - game_state.last_bullet_tick >= config.autofire_delay {
                    game_state.last_bullet_tick = game_state.current_tick;
                    let bullet = entities.create();
                    let x = player_geom.x as u32 + config.ship_dimensions.0 / 2 - config.bullet_dimensions.0 / 2;
                    let y = player_geom.y as u32 - config.bullet_dimensions.1;
                    updater.insert(bullet, Geometry {
                        x: x as i32,
                        y: y as i32,
                        width: config.bullet_dimensions.0,
                        height: config.bullet_dimensions.1,
                    });
                    updater.insert(bullet, Velocity(0, -1 * config.bullet_speed as i32));
                    updater.insert(bullet, Sprite { id: 1 });
                }
            } else {
                game_state.last_bullet_tick = 0;
            }

            if self.input.borrow().is_key_down(KEY_LEFT) {
                velocity.0 = config.ship_speed as i32 * -1;
            } else if self.input.borrow().is_key_down(KEY_RIGHT) {
                velocity.0 = config.ship_speed as i32;
            } else {
                velocity.0 = 0;
            }
        }
    }
}
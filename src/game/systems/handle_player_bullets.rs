use specs::prelude::*;

use crate::game::builder::GameConfig;
use crate::game::components::*;
use crate::game::GameState;

pub struct HandlePlayerBullets {}

impl<'a> System<'a> for HandlePlayerBullets {
    type SystemData = (
        ReadStorage<'a, Geometry>,
        ReadStorage<'a, IsPlayer>,
        Read<'a, GameConfig>,
        Read<'a, LazyUpdate>,
        Entities<'a>,
        Write<'a, GameState>
    );


    fn run(&mut self, (
        gos,
        players,
        config,
        updater,
        entities,
        mut game_state): Self::SystemData)
    {
        for (player_geom, _) in (&gos, &players).join() {
            if game_state.shooting {
                if game_state.current_tick - game_state.last_bullet_tick >= config.autofire_delay {
                    game_state.last_bullet_tick = game_state.current_tick;
                    let bullet = entities.create();
                    updater.insert(bullet, IsBullet);
                    updater.insert(bullet, Self::get_bullet_geometry(&config, player_geom));
                    updater.insert(bullet, Velocity(0, -1 * config.bullet_speed as i32));
                    updater.insert(bullet, Sprite { id: 1 });
                }
            } else {
                game_state.last_bullet_tick = 0;
            }
        }
    }
}

impl HandlePlayerBullets {
    fn get_bullet_geometry(config: &GameConfig, player_geom: &Geometry) -> Geometry {
        let x = player_geom.x as u32 + config.ship_dimensions.0 / 2 - config.bullet_dimensions.0 / 2;
        let y = player_geom.y as u32 - config.bullet_dimensions.1;
        Geometry {
            x: x as i32,
            y: y as i32,
            width: config.bullet_dimensions.0,
            height: config.bullet_dimensions.1,
        }
    }
}

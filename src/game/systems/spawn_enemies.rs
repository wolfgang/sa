use specs::{Entities, LazyUpdate, Read, System, Write};

use crate::game::builder::GameConfig;
use crate::game::components::{Geometry, IsEnemy, Sprite, Velocity};
use crate::game::GameState;

pub struct SpawnEnemies {}

impl<'a> System<'a> for SpawnEnemies {
    type SystemData = (
        Entities<'a>,
        Read<'a, LazyUpdate>,
        Read<'a, GameConfig>,
        Write<'a, GameState>
    );

    fn run(&mut self, (entities, updater, config, mut game_state): Self::SystemData) {
        if game_state.active_enemies != config.enemy_count {
            let enemy = entities.create();

            updater.insert(enemy, IsEnemy);
            updater.insert(enemy, Geometry {
                x: 0,
                y: 0,
                width: config.enemy_dimensions.0,
                height: config.enemy_dimensions.1,
            });

            updater.insert(enemy, Velocity(config.enemy_speed.0 as i32, config.enemy_speed.1 as i32));
            updater.insert(enemy, Sprite { id: 2 });

            game_state.active_enemies += 1;
        }
    }
}
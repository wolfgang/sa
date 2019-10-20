use specs::prelude::*;

use crate::game::components::*;
use crate::game::systems::*;

pub fn setup() -> (World, Dispatcher<'static, 'static>) {
    let mut world = World::new();
    world.register::<Geometry>();
    world.register::<Velocity>();
    world.register::<IsPlayer>();
    world.register::<Sprite>();
    world.register::<IsBullet>();
    world.register::<IsEnemy>();

    let dispatcher = DispatcherBuilder::new()
        .with(HandlePlayerMovement {}, "handle_player_movement", &[])
        .with(HandlePlayerBullets {}, "handle_player_bullets", &[])
        .with(SpawnEnemies {}, "spawn_enemies", &[])

        .with(
            MoveGameObjects {},
            "move_game_objects",
            &["handle_player_movement", "handle_player_bullets", "spawn_enemies"])
        .with(
            ConstrainPlayerToScreen {},
            "constrain_player_to_screen",
            &["move_game_objects"])
        .with(
            ExpirePlayerBullets {},
            "expire_player_bullets",
            &["move_game_objects"])

        .with(HandleEnemyMovement {}, "handle_enemy_movement", &["move_game_objects"])
        .with(CollidePlayerBullets {}, "collide_player_bullets", &["move_game_objects"])

        .build();

    (world, dispatcher)
}
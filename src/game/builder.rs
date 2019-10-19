use specs::{Builder, DispatcherBuilder, World, WorldExt};

use crate::game::systems::*;

use super::components::*;
use super::Game;
use super::GameState;
use super::input::{InputNull, InputRef};

#[derive(Clone, Default)]
pub struct GameConfig {
    pub dimensions: (u32, u32),
    pub ship_dimensions: (u32, u32),
    pub ship_speed: u32,
    pub bullet_dimensions: (u32, u32),
    pub bullet_speed: u32,
    pub autofire_delay: u32,
    pub enemy_count: u32,
    pub enemy_dimensions: (u32, u32),
    pub enemy_speed: (u32, u32),
}

#[derive(Clone)]
pub struct GameBuilder {
    input: InputRef,
    pub(crate) config: GameConfig,
}


impl GameBuilder {
    pub fn new() -> Self {
        Self {
            input: InputNull::new_rc(),
            config: GameConfig::default(),
        }
    }
    pub fn with_dimensions(&mut self, width: u32, height: u32) -> &mut Self {
        self.config.dimensions = (width, height);
        self
    }

    pub fn with_ship_dimensions(&mut self, width: u32, height: u32) -> &mut Self {
        self.config.ship_dimensions = (width, height);
        self
    }

    pub fn with_ship_speed(&mut self, speed: u32) -> &mut Self {
        self.config.ship_speed = speed;
        self
    }

    pub fn with_bullet_dimensions(&mut self, width: u32, height: u32) -> &mut Self {
        self.config.bullet_dimensions = (width, height);
        self
    }


    pub fn with_bullet_speed(&mut self, speed: u32) -> &mut Self {
        self.config.bullet_speed = speed;
        self
    }

    pub fn with_autofire_delay(&mut self, delay: u32) -> &mut Self {
        self.config.autofire_delay = delay;
        self
    }

    pub fn with_enemy_count(&mut self, count: u32) -> &mut Self {
        self.config.enemy_count = count;
        self
    }
    pub fn with_enemy_dimensions(&mut self, width: u32, height: u32) -> &mut Self {
        self.config.enemy_dimensions = (width, height);
        self
    }

    pub fn with_enemy_speed(&mut self, x: u32, y: u32) -> &mut Self {
        self.config.enemy_speed = (x, y);
        self
    }

    pub fn with_input(&mut self, input: InputRef) -> &mut Self {
        self.input = input;
        self
    }

    pub fn build(&self) -> Game {
        let mut world = World::new();
        world.register::<Geometry>();
        world.register::<Velocity>();
        world.register::<IsPlayer>();
        world.register::<Sprite>();
        world.register::<IsBullet>();
        world.register::<IsEnemy>();

        let (ship_width, ship_height) = self.config.ship_dimensions;
        let (width, height) = self.config.dimensions;
        let x = width / 2 - ship_width / 2;
        let y = height - ship_height;
        world.create_entity()
            .with(Geometry { x: x as i32, y: y as i32, width: ship_width, height: ship_height })
            .with(Velocity(0, 0))
            .with(Sprite { id: 0 })
            .with(IsPlayer)
            .build();


        if self.config.enemy_count == 1 {
            world.create_entity()
                .with(IsEnemy)
                .with(Geometry {
                    x: 0,
                    y: 0,
                    width: self.config.enemy_dimensions.0,
                    height: self.config.enemy_dimensions.1,
                })
                .with(Velocity(self.config.enemy_speed.0 as i32, self.config.enemy_speed.1 as i32))
                .with(Sprite { id: 2 })
                .build();
        }


        world.insert(self.config.clone());
        world.insert(GameState { current_tick: 1000, ..Default::default() });

        let dispatcher = DispatcherBuilder::new()
            .with(HandlePlayerMovement {}, "handle_player_movement", &[])
            .with(HandlePlayerBullets {}, "handle_player_bullets", &[])
            .with(
                MoveGameObjects {},
                "move_game_objects",
                &["handle_player_movement", "handle_player_bullets"])
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


        Game::new(world, dispatcher, self.input.clone())
    }
}
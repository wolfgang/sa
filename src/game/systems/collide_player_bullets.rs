use specs::{Entities, join::Join, ReadStorage, System, Write};

use crate::core::rectangle::RectangleU32;
use crate::game::components::{Geometry, IsBullet, IsEnemy};
use crate::game::GameState;

pub struct CollidePlayerBullets;

impl<'a> System<'a> for CollidePlayerBullets {
    type SystemData = (
        ReadStorage<'a, IsEnemy>,
        ReadStorage<'a, IsBullet>,
        ReadStorage<'a, Geometry>,
        Entities<'a>,
        Write<'a, GameState>,
    );

    fn run(&mut self, (enemies, bullets, geometries, entities, mut game_state): Self::SystemData) {
        for (_, bullet_geometry, bullet_entity) in (&bullets, &geometries, &*entities).join() {
            let bullet_rect = rect_from(&bullet_geometry);
            for (_, enemy_geometry, enemy_entity) in (&enemies, &geometries, &*entities).join() {
                let enemy_rect = rect_from(&enemy_geometry);
                if RectangleU32::intersect(&bullet_rect, &enemy_rect) {
                    entities.delete(bullet_entity).unwrap();
                    entities.delete(enemy_entity).unwrap();
                    game_state.active_enemies -= 1;
                }
            }
        }
    }
}

fn rect_from(geometry: &Geometry) -> RectangleU32 {
    RectangleU32 { x: geometry.x, y: geometry.y, width: geometry.width, height: geometry.height }
}

use specs::{Entities, join::Join, ReadStorage, System};

use crate::game::components::{Geometry, IsBullet};

pub struct ExpirePlayerBullets {}

impl<'a> System<'a> for ExpirePlayerBullets {
    type SystemData = (
        ReadStorage<'a, IsBullet>,
        ReadStorage<'a, Geometry>,
        Entities<'a>
    );

    fn run(&mut self, (bullets, geometries, entities): Self::SystemData) {
        for (_, geometry, entity) in (&bullets, &geometries, &*entities).join() {
            if geometry.y <= -1 * (geometry.height as i32) {
                entities.delete(entity).unwrap();
            }
        }
    }
}

use specs::prelude::*;

use crate::game::components::*;

struct ExpireBullets {}

impl<'a> System<'a> for ExpireBullets {
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

#[test]
fn remove_bullets_that_leave_screen() {
    let mut world = World::new();
    world.register::<IsBullet>();
    world.register::<Geometry>();

    let bullet1 = world.create_entity()
        .with(IsBullet)
        .with(Geometry { x: 1, y: 2, width: 1, height: 2 })
        .build();

    let bullet2 = world.create_entity()
        .with(IsBullet)
        .with(Geometry { x: 1, y: -2, width: 1, height: 2 })
        .build();

    let non_bullet = world.create_entity().build();

    assert!(world.entities().is_alive(non_bullet));
    assert!(world.entities().is_alive(bullet1));
    assert!(world.entities().is_alive(bullet2));

    let mut system = ExpireBullets {};
    system.run_now(&mut world);

    world.maintain();

    assert!(world.entities().is_alive(non_bullet));
    assert!(world.entities().is_alive(bullet1));
    assert!(!world.entities().is_alive(bullet2), "Expected bullet to have expired");
}

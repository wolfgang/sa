use specs::prelude::*;

use crate::game::components::{Geometry, IsBullet};
use crate::game::systems::ExpirePlayerBullets;

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

    let mut system = ExpirePlayerBullets {};
    system.run_now(&mut world);

    world.maintain();

    assert!(world.entities().is_alive(non_bullet));
    assert!(world.entities().is_alive(bullet1));
    assert!(!world.entities().is_alive(bullet2), "Expected bullet to have expired");
}

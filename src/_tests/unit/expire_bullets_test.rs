use specs::prelude::*;

use crate::game::components::*;

#[test]
fn how_to_delete_an_entity() {
    let mut world = World::new();
    world.register::<IsBullet>();

    let bullet1 = world.create_entity().with(IsBullet).build();

    assert!(world.entities().is_alive(bullet1));

    world.entities().delete(bullet1).unwrap();
    world.maintain();

    assert!(!world.entities().is_alive(bullet1));
}
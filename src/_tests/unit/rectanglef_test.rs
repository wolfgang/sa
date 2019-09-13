use crate::game::geometry::{Rectanglef, Vector2};

#[test]
fn intersect() {
    let r1 = Rectanglef::with(Vector2::with(1.0, 2.0), 3.0, 4.0);
    let r2 = Rectanglef::with(Vector2::with(0.0, 1.0), 2.0, 3.0);
    let r3 = Rectanglef::with(Vector2::with(6.0, 7.0), 2.0, 3.0);
    let r4 = Rectanglef::with(Vector2::with(10.0, 7.0), 2.0, 3.0);
    let r5 = Rectanglef::with(Vector2::with(1.0, 10.0), 2.0, 3.0);
    let r6 = Rectanglef::with(Vector2::with(3.0, 1.0), 5.0, 3.0);

    assert!(r2.intersects(&r1));
    assert!(!r3.intersects(&r1));
    assert!(!r4.intersects(&r1));
    assert!(!r5.intersects(&r1));
    assert!(r6.intersects(&r1));
}

#[test]
fn inside_means_intersection() {
    let r1 = Rectanglef::with(Vector2::with(1.0, 1.0), 10.0, 10.0);
    let r2 = Rectanglef::with(Vector2::with(2.0, 3.0), 5.0, 4.0);
    assert!(r1.intersects(&r2));
    assert!(r2.intersects(&r1));
}

#[test]
fn is_inside_of() {
    let r1 = Rectanglef::with(Vector2::with(1.0, 1.0), 10.0, 10.0);
    let r2 = Rectanglef::with(Vector2::with(2.0, 3.0), 5.0, 4.0);
    let r3 = Rectanglef::with(Vector2::with(2.0, 3.0), 20.0, 30.0);
    assert!(r1.is_inside_of(&r1));
    assert!(r2.is_inside_of(&r1));
    assert!(!r3.is_inside_of(&r1));
}



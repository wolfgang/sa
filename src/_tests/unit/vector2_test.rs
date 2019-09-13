use crate::game::geometry::{Vector2, Vector2F};

#[test]
fn scale_vector_with_other_vector_of_sane_or_convertible_type() {
    let mut v1: Vector2F = Vector2::new(2.0, 3.0);

    v1.scale(&Vector2::new(4.0, 5.0));
    assert_eq!(v1, Vector2::new(8.0, 15.0));

    v1.scale(&Vector2::new(4, 5));
    assert_eq!(v1, Vector2::new(32.0, 75.0));
}

#[test]
fn multiply_with_scalar() {
    let scaled = Vector2::new(2.0, 3.0) * 20.0;
    assert_eq!(scaled, Vector2::new(40.0, 60.0));
}

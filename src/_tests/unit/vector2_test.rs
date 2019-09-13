use crate::game::geometry::Vector2;

#[test]
fn multiply_with_scalar() {
    let scaled = Vector2::with(2.0, 3.0) * 20.0;
    assert_eq!(scaled, Vector2::with(40.0, 60.0));
}

#[test]
fn multiply_with_vector() {
    let v1 = Vector2::with(2.0, 3.0);
    let v2 = Vector2::with(4.0, 5.0);
    assert_eq!(Vector2::with(8.0, 15.0), v1 * v2);
}

#[test]
fn add_assign() {
    let mut v1 = Vector2::with(5.0, 7.0);
    v1 += Vector2::with(11.0, 20.0);
    assert_eq!(v1, Vector2::with(16.0, 27.0));
}
use crate::game::geometry::Vector2;

#[test]
fn scale_vector_with_other_vector_of_sane_or_convertible_type() {
    let mut v1: Vector2<f64> = Vector2::new(2.0, 3.0);

    v1.scale(&Vector2::new(4.0, 5.0));
    assert_eq!(v1.x, 8.0);
    assert_eq!(v1.y, 15.0);

    v1.scale(&Vector2::new(4, 5));
    assert_eq!(v1.x, 32.0);
    assert_eq!(v1.y, 75.0);
}
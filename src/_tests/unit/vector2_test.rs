use crate::game::geometry::Vector2;

#[test]
fn scale_vector_with_other_vector_of_different_type() {
    let mut v1 = Vector2::new(2.0, 3.0);
    let v2 = Vector2::new(4.0, 5.0);

    v1.scale(&v2);

    assert_eq!(v1.x, 8.0);
    assert_eq!(v1.y, 15.0);

//    let mut vector1 = Vector2::new(i1, i2);
//    let vector2 = Vector2::new(u1, u2);
//
//    vector1.scale(&vector2);
//
//    assert_eq!(vector1.x, 50);
//    assert_eq!(vector1.y, -140);
}
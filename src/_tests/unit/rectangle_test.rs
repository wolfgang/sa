use crate::_tests::helpers::rect_world::RectWorld;
use crate::core::rectangle::RectangleU32;

#[test]
fn intersection() {
    let rects = RectWorld::new(vec![
        "..4.......",
        "....4.....",
        ".1----|...",
        ".|..2-|-|.",
        ".|..|.|.|.",
        ".|----1.|.",
        "..3.|...|.",
        ".3..|---2.",
    ]);

    assert!(RectangleU32::intersect(&rects.rectangle(1), &rects.rectangle(2)));
    assert!(RectangleU32::intersect(&rects.rectangle(2), &rects.rectangle(1)));
    assert!(!RectangleU32::intersect(&rects.rectangle(1), &rects.rectangle(3)));
    assert!(!RectangleU32::intersect(&rects.rectangle(1), &rects.rectangle(4)));
    assert!(!RectangleU32::intersect(&rects.rectangle(3), &rects.rectangle(4)));
}
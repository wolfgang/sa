use crate::_tests::helpers::rect_world::RectWorld;
use crate::core::rectangle::RectangleU32;

#[test]
fn get_rectangles() {
    let rects = RectWorld::new(vec![
        ".1----|...",
        ".|..2-|-|.",
        ".|..|.|.|.",
        ".|----1.|.",
        "..3.|...|.",
        ".3..|---2.",
    ]);

    assert_eq!(rects.rectangle(1), RectangleU32 { x: 1, y: 0, width: 6, height: 4 });
    assert_eq!(rects.rectangle(2), RectangleU32 { x: 4, y: 1, width: 5, height: 5 });
    assert_eq!(rects.rectangle(3), RectangleU32 { x: 2, y: 4, width: 2, height: 2 });
}

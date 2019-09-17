pub trait Sprite {
    fn position(&self) -> (i32, i32);
    fn id(&self) -> u8;
}

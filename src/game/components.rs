use specs::prelude::*;
use specs_derive::Component;

#[derive(Component)]
pub struct Geometry {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[derive(Component)]
pub struct Velocity(pub i32, pub i32);

#[derive(Component)]
pub struct Sprite {
    pub id: u8
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct IsPlayer;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct IsBullet;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct IsEnemy;

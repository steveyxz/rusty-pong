use bevy::{
    ecs::component::Component,
    prelude::{Bundle, Vec2},
    sprite::SpriteBundle,
};

#[derive(Component, Default)]
pub struct Collider;

#[derive(Component, Default)]
pub struct Velocity {
    pub vel: Vec2,
}

#[derive(Bundle, Default)]
pub struct Paddle {
    pub vel: Velocity,
    pub renderer: SpriteBundle,
    pub collider: Collider,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Ai;

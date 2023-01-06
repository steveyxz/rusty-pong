use bevy::{
    ecs::system::Query,
    prelude::{
        Camera2dBundle, Color, Commands, GlobalTransform, Input, KeyCode, Rect, Res, Transform,
        Vec2,
    },
    sprite::{Sprite, SpriteBundle},
    utils::default,
};
use bevy::math::Vec3;

use crate::common::constants;

use super::components::{Paddle, Player, Velocity};

pub fn update_velocities(mut query: Query<(&Velocity, &mut Transform)>) {
    let mut iter = query.iter_combinations_mut();
    while let Some([(vel, mut pos)]) = iter.fetch_next() {
        pos.translation.x += vel.vel.x;
        pos.translation.y += vel.vel.y;
    }
}

pub fn initialize(mut commands: Commands) {
    //Spawn camera
    commands.spawn(Camera2dBundle::default());

    //Spawn player
    commands.spawn((
        Paddle {
            renderer: SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(10.0, 100.0)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(1.0, 50.0, 0.0),
                    ..default()
                },
                ..default()
            },
            ..default()
        },
        Player,
    ));
}

pub fn keyboard_input(input: Res<Input<KeyCode>>, mut query: Query<(&mut Velocity, &Player)>) {
    let mut new_y_vel = 0.0;
    if input.pressed(KeyCode::S) {
        new_y_vel -= constants::PADDLE_SPEED;
    }
    if input.pressed(KeyCode::W) {
        new_y_vel += constants::PADDLE_SPEED;
    }

    let mut iter = query.iter_combinations_mut();
    while let Some([(mut vel, _player)]) = iter.fetch_next() {
        vel.vel.y = new_y_vel;
    }
}

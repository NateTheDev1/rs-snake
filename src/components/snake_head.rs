use bevy::{
    prelude::{Color, Component},
    reflect::Reflect,
};

pub const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);

#[derive(Component, Reflect, Default)]
pub struct SnakeHead;

mod components;
mod inspector;
mod systems;

use bevy::{
    prelude::{App, PluginGroup},
    window::{WindowDescriptor, WindowPlugin},
    DefaultPlugins,
};
use components::snake_head::SnakeHead;
use inspector::InspectorPlugin;
use systems::{camera::CameraPlugin, snake_system::SnakePlugin};

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: WIDTH,
                height: HEIGHT,
                title: "SnakeRS".to_string(),
                ..Default::default()
            },
            ..Default::default()
        }))
        .register_type::<SnakeHead>()
        .add_plugin(InspectorPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(SnakePlugin)
        .run();
}

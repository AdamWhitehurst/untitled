use bevy::ecs::archetype::Archetypes;
use bevy::ecs::component::Components;
use bevy::prelude::*;
use bevy::reflect::TypeRegistration;
use bevy::{window::WindowDescriptor, DefaultPlugins};
mod camera;
mod mouse;
mod tiles;

fn main() {
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        width: 1270.0,
        height: 720.0,
        title: String::from(""),
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(camera::Plugin)
    .add_plugin(tiles::TilesPlugin)
    .add_plugin(mouse::Plugin)
    .run();
}

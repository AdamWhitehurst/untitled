#![feature(int_abs_diff)]

use bevy::prelude::*;
use bevy::{window::WindowDescriptor, DefaultPlugins};
mod camera;
mod mouse;
mod pathfinding;
mod player;
mod sprite;
mod tile_editor;
mod tiles;

#[cfg(target_arch = "wasm32")]
mod canvas_resizer;
fn main() {
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        resizable: true,
        width: 1270.0,
        height: 720.0,
        title: String::from(""),
        ..Default::default()
    })
    .insert_resource(ClearColor(
        Color::hex("291e31").expect("Color::hex(\"291e31\")"),
    ))
    .add_plugins(DefaultPlugins)
    .add_plugin(camera::Plugin)
    .add_plugin(tiles::Plugin)
    .add_plugin(tile_editor::Plugin)
    .add_plugin(mouse::Plugin)
    .add_plugin(pathfinding::Plugin)
    .add_plugin(player::Plugin);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(canvas_resizer::WebCanvasResizerPlugin);
    app.run();
}

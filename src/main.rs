use bevy::prelude::*;
use bevy::{window::WindowDescriptor, DefaultPlugins};
mod camera;
mod mouse;
mod player;
mod sprite;
mod tiles;

#[cfg(target_arch = "wasm32")]
mod canvas_resizer;
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
    .add_plugin(player::Plugin);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(canvas_resizer::WebCanvasResizerPlugin);

    app.run();
}

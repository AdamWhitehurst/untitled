use bevy::prelude::*;
use bevy::{window::WindowDescriptor, DefaultPlugins};
mod camera;
mod mouse;
mod player;
mod sprite;
// mod tile_editor;
mod tiles;

use bevy_ecs_tilemap::TilemapStage;

#[derive(Debug, Clone, PartialEq, Eq, Hash, StageLabel)]
pub struct MyTilemapStage;

use bevy_ecs_tilemap::prelude::*;
use bevy_tileset_map::prelude::*;
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
    .add_plugin(TilemapPlugin);
    app.add_plugin(camera::Plugin)
        .add_plugin(tiles::TilesPlugin)
        .add_plugin(mouse::Plugin)
        .add_plugin(player::Plugin);
    app.add_plugin(TilesetPlugin::default())
        .add_plugin(TilesetMapPlugin);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(canvas_resizer::WebCanvasResizerPlugin);
    app.run();
}

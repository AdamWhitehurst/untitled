use bevy::prelude::*;
use bevy::{math::Vec3Swizzles, prelude::Plugin as BevyPlugin};

use crate::pathfinding::TilePath;
use crate::{
    camera::{WorldCamera, SCALE},
    pathfinding::Destination,
    player::PlayerCharacter,
};
use bevy::math::Vec4Swizzles;
use bevy_ecs_tilemap::{MapQuery, TilePos, TilemapPlugin};
use bevy_tileset_map::prelude::*;

/// The position of the cursor at the time of this event
/// and whether it is pressed or not
pub struct ClickEvent(pub Vec2, pub bool);

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TilemapPlugin)
            .add_plugin(TilesetPlugin::default())
            .add_plugin(TilesetMapPlugin)
            .add_event::<ClickEvent>()
            .init_resource::<TerrainTileset>()
            .add_startup_system(load_tiles)
            .add_system(build_map)
            .add_system(on_click)
            .add_system(on_tile_click);
    }
}

#[derive(Default)]
struct TerrainTileset {
    /// This stores the handle to our tileset so it doesn't get unloaded
    handle: Option<Handle<Tileset>>,
}

/// Starts the tileset loading process
fn load_tiles(mut my_tileset: ResMut<TerrainTileset>, asset_server: Res<AssetServer>) {
    my_tileset.handle = Some(asset_server.load("tilesets/tileset.ron"));
}

/// A local state noting if the map has been built or not
#[derive(Default)]
struct BuildMapState {
    built: bool,
}

/// A system used to build the tilemap
fn build_map(
    tilesets: Tilesets,
    mut commands: Commands,
    mut map_query: MapQuery,
    mut local_state: Local<BuildMapState>,
    my_tileset: Res<TerrainTileset>,
) {
    if local_state.built {
        return;
    }

    if let Some(tileset) = tilesets.get(&my_tileset.handle.clone().unwrap()) {
        crate::tiles::load_map(&mut commands, &mut map_query, tileset);
        local_state.built = true;
    }
}

pub fn iso_to_world(p: &Vec2) -> Vec2 {
    Vec2::new((p.x - p.y) * 8., -(p.y + p.x) * 4.)
}

pub fn project_iso(p: &Vec2) -> Vec2 {
    Vec2::new((p.x / 16.) - (p.y / 8.), (-p.x / 16.) - (p.y / 8.))
}

pub fn on_click(
    query: Query<&Transform, With<WorldCamera>>,
    wnds: Res<Windows>,
    buttons: Res<Input<MouseButton>>,
    mut event_writer: EventWriter<ClickEvent>,
) {
    let just_pressed = buttons.just_pressed(MouseButton::Left);
    let just_released = buttons.just_released(MouseButton::Left);

    if !just_pressed && !just_released {
        return;
    }

    let wnd = wnds.get_primary().unwrap();
    if let Some(pos) = wnd.cursor_position() {
        let cam = query.single();
        let p = window_to_world(&pos, wnd, cam);

        let ppos = project_iso(&p);
        if ppos.x < 0. || ppos.y < 0. {
            return;
        }
        event_writer.send(ClickEvent(p, just_pressed));
    }
}

pub fn window_to_world(position: &Vec2, window: &Window, camera: &Transform) -> Vec2 {
    // get the size of the window
    let size = Vec2::new(window.width() as f32, window.height() as f32);

    // the default orthographic projection is in pixels from the center;
    // just undo the translation and apply camera scale
    let p = (*position - size / 2.0) * SCALE;

    // apply the camera transform
    (camera.compute_matrix() * p.extend(0.0).extend(1.0)).xy()
}

/// A system that adds/removes tiles when clicked
fn on_tile_click(
    mut event_reader: EventReader<ClickEvent>,
    mut query: Query<(Entity, &Transform), With<PlayerCharacter>>,
    mut commands: Commands,
) {
    for ClickEvent(ref p, pressed) in event_reader.iter() {
        if !pressed {
            continue;
        }

        if let Some((e, t)) = query.get_single_mut().ok() {
            let tp = project_iso(p);
            let ptp = project_iso(&t.translation.xy());

            commands
                .entity(e)
                .remove::<TilePath>()
                .insert(Destination::new(
                    TilePos(ptp.x as u32, ptp.y as u32),
                    TilePos(tp.x as u32, tp.y as u32),
                ));
        }
    }
}

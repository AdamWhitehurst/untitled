use bevy::prelude::Plugin as BevyPlugin;
use bevy::prelude::*;
use bevy_ecs_tilemap::Map;
use bevy_ecs_tilemap::MapQuery;
use bevy_ecs_tilemap::{Tile, TilePos};
use bevy_tiled_camera::TiledProjection;
pub struct Plugin;
pub type GlobalCursorPosition = (Option<Vec2>, Option<Vec2>);
pub type CursorTilePosition = (Option<TilePos>, Option<TilePos>);

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_global_cursor_pos)
            .insert_resource::<GlobalCursorPosition>((None, None))
            .add_system(update_cursor_tile_pos)
            .insert_resource::<CursorTilePosition>((None, None))
            .add_system(tile_change);
    }
}

fn update_global_cursor_pos(
    windows: Res<Windows>,
    query: Query<(&GlobalTransform, &TiledProjection, &Camera)>,
    mut global_cursor: ResMut<GlobalCursorPosition>,
) {
    // let (last_pos, _) = *global_cursor;
    // let (gt, tp, c) = query.single();
    // tp.screen_to_world(c, windows, gt, screen_pos)
    // let win = windows.get_primary().expect("primary_window");
    //     for (t, o) in query.iter() {
    //         *global_cursor = if let Some(cursor_screen_pos) = win.cursor_position() {
    //             let win_half_dims = Vec2::new(win.width() / 2.0, win.height() / 2.0);
    //             let cam_global_pos = Vec2::new(t.translation.x, t.translation.y);
    //
    //             (
    //                 Some((cursor_screen_pos - win_half_dims) * o. + cam_global_pos),
    //                 last_pos,
    //             )
    //         } else {
    //             (None, last_pos)
    //         };
    //         // info!("{:?}", global_cursor);
    //     }
}

fn update_cursor_tile_pos(
    mut map: MapQuery,
    t: Query<&Transform, With<Map>>,
    cursor_position: Res<GlobalCursorPosition>,
    mut cursor_tile: ResMut<CursorTilePosition>,
) {
    let (last_cursor_tile_pos, _) = *cursor_tile;
    let (maybe_curr, _) = *cursor_position;

    let curr_cursor_tile_pos = if let (Some(pos), Some(trfm)) = (maybe_curr, t.iter().next()) {
        let (_, l) = map.get_layer(0u16, 0u16).expect("Map Layer");

        // Computer Map bounds
        let map_bl_pos = trfm.translation;
        let map_pixel_dims = (
            l.settings.tile_size.0 * l.settings.chunk_size.0 as f32 * l.settings.map_size.0 as f32,
            l.settings.tile_size.1 * l.settings.chunk_size.1 as f32 * l.settings.map_size.1 as f32,
        );
        let map_tr_pos = (
            map_bl_pos.x + map_pixel_dims.0,
            map_bl_pos.y + map_pixel_dims.1,
        );

        if pos.x >= map_bl_pos.x
            && pos.x <= map_tr_pos.0
            && pos.y >= map_bl_pos.y
            && pos.y <= map_tr_pos.1
        {
            let offsetted_cursor = (pos.x - map_bl_pos.x, pos.y - map_bl_pos.y);
            let t = TilePos(
                ((offsetted_cursor.0) / l.settings.tile_size.0) as u32,
                ((offsetted_cursor.1) / l.settings.tile_size.1) as u32,
            );

            Some(t)
        } else {
            None
        }
    } else {
        None
    };

    *cursor_tile = (curr_cursor_tile_pos, last_cursor_tile_pos);
}

fn tile_change(
    mut map: MapQuery,
    cursor_tile: ResMut<CursorTilePosition>,
    mut tile_query: Query<&mut Tile>,
    buttons: Res<Input<MouseButton>>,
) {
    let (maybe_curr, _) = *cursor_tile;

    if let Some(pos) = maybe_curr {
        let te = map.get_tile_entity(pos, 0u16, 0u16).expect("tile");
        if let Ok(mut tile) = tile_query.get_mut(te) {
            *tile = Tile {
                texture_index: if buttons.just_pressed(MouseButton::Left) {
                    tile.texture_index.checked_add(1).unwrap_or(0)
                } else if buttons.just_pressed(MouseButton::Right) {
                    tile.texture_index.checked_sub(1).unwrap_or(0)
                } else {
                    tile.texture_index
                },
                ..*tile
            };
            info!("Index: {:?}", tile.texture_index);
            map.notify_chunk_for_tile(pos, 0u16, 0u16);
        }
    }
}

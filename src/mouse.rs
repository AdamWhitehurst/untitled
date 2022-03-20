use bevy::prelude::Plugin as BevyPlugin;
use bevy::prelude::*;
use bevy_ecs_tilemap::Map;
use bevy_ecs_tilemap::MapQuery;
use bevy_ecs_tilemap::TilePos;
pub struct Plugin;
pub type GlobalCursorPosition = (Option<Vec2>, Option<Vec2>);
pub type CursorTilePosition = (Option<TilePos>, Option<TilePos>);

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_global_cursor_pos)
            .insert_resource::<GlobalCursorPosition>((None, None))
            .add_system(update_cursor_tile_pos)
            .insert_resource::<CursorTilePosition>((None, None));
        // .add_system(tile_change);
    }
}

fn update_global_cursor_pos(
    windows: Res<Windows>,
    query: Query<(&Transform, &OrthographicProjection, &Camera)>,
    mut global_cursor: ResMut<GlobalCursorPosition>,
) {
    let (last_pos, _) = *global_cursor;
    let win = windows.get_primary().expect("primary_window");
    for (t, o, _) in query.iter() {
        *global_cursor = if let Some(cursor_screen_pos) = win.cursor_position() {
            let win_half_dims = Vec2::new(win.width() / 2.0, win.height() / 2.0);
            let cam_global_pos = Vec2::new(t.translation.x, t.translation.y);

            (
                Some((cursor_screen_pos - win_half_dims) * o.scale + cam_global_pos),
                last_pos,
            )
        } else {
            (None, last_pos)
        };
        // info!("GC: {:?}", global_cursor);
    }
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
            l.settings.grid_size.x * l.settings.chunk_size.0 as f32 * l.settings.map_size.0 as f32,
            l.settings.grid_size.y * l.settings.chunk_size.1 as f32 * l.settings.map_size.1 as f32,
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
                ((offsetted_cursor.0) / l.settings.grid_size.x) as u32,
                ((offsetted_cursor.1) / l.settings.grid_size.y) as u32,
            );

            Some(t)
        } else {
            // info!(
            //     " None
            //     - {:?}
            //     - {:?}
            //     - {:?}
            //     - {:?}
            //     ",
            //     pos, map_bl_pos, map_pixel_dims, map_tr_pos
            // );
            None
        }
    } else {
        // info!("None 2");
        None
    };

    *cursor_tile = (curr_cursor_tile_pos, last_cursor_tile_pos);
}

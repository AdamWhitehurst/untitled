use bevy::prelude::Plugin as BevyPlugin;
use bevy::prelude::*;
use serde_json;

use crate::camera::{WorldCamera, SCALE};
use bevy::math::Vec4Swizzles;
use bevy::render::render_resource::TextureUsages;
use bevy_ecs_tilemap::{MapQuery, TilePos, TilemapPlugin};
use bevy_tileset_map::prelude::*;

pub fn set_texture_filters_to_nearest(
    mut texture_events: EventReader<AssetEvent<Image>>,
    mut textures: ResMut<Assets<Image>>,
) {
    // quick and dirty, run this for all textures anytime a texture is created.
    for event in texture_events.iter() {
        match event {
            AssetEvent::Created { handle } => {
                if let Some(mut texture) = textures.get_mut(handle) {
                    texture.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING
                        | TextureUsages::COPY_SRC
                        | TextureUsages::COPY_DST;
                }
            }
            _ => (),
        }
    }
}
/// The name of the tileset we'll be loading in this example
///
/// This could be any string and doesn't need to be a constant or static.
///
/// Additionally, we could just use the handle to the tileset to access it, but we'll
/// use this because the `DebugTilesetPlugin` expects it
const MY_TILESET: &str = "terrain";
/// The position of the cursor at the time of this event
/// and whether it is pressed or not
pub struct ClickEvent(pub Vec2, pub bool);

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TilemapPlugin)
            .add_plugin(TilesetPlugin::default())
            .add_plugin(TilesetMapPlugin)
            .init_resource::<MyTileset>()
            .init_resource::<SavedMap>()
            .insert_resource(BuildMode {
                tile_name: String::from("grass"),
                active_layer: 0u16,
                mode: 0,
            })
            .add_event::<ClickEvent>()
            .add_startup_system(load_tiles)
            .add_startup_system(setup_hud)
            .add_system(build_map)
            .add_system(on_keypress)
            .add_system(on_click)
            .add_system(set_texture_filters_to_nearest)
            .add_system(update_text)
            .add_system(on_tile_click);
    }
}

#[derive(Default)]
struct MyTileset {
    /// This stores the handle to our tileset so it doesn't get unloaded
    handle: Option<Handle<Tileset>>,
}

#[derive(Default)]
struct SavedMap {
    /// The currently saved tilemap
    map: Option<SerializableTilemap>,
}

/// Starts the tileset loading process
fn load_tiles(mut my_tileset: ResMut<MyTileset>, asset_server: Res<AssetServer>) {
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
    my_tileset: Res<MyTileset>,
) {
    if local_state.built {
        return;
    }

    if let Some(tileset) = tilesets.get(&my_tileset.handle.clone().unwrap()) {
        crate::tiles::load_map(&mut commands, &mut map_query, tileset);
        local_state.built = true;
    }
}

/// A simple resource to control what layer and tile we're using
/// as well as the placement mode
#[derive(Debug)]
struct BuildMode {
    tile_name: String,
    active_layer: u16,
    mode: usize,
}

/// A simple enum that controls which placement method we're using
///
/// See [`TilePlacer`] for details on each
#[derive(Debug)]
enum PlacementMode {
    Place,
    TryPlace,
    Toggle,
    ToggleMatch,
    Replace,
    Remove,
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
    tilesets: Tilesets,
    build_mode: Res<BuildMode>,
    mut event_reader: EventReader<ClickEvent>,
    mut placer: TilePlacer,
) {
    if let Some(tileset) = tilesets.get_by_name(MY_TILESET) {
        for ClickEvent(ref p, pressed) in event_reader.iter() {
            if !pressed {
                continue;
            }

            let px = p.x / 8.;
            let py = p.y / 4.;
            let x = (px - py) / 2.;
            let y = (px + py) / 2.;
            let ppos = Vec2::new(x, -y);
            if (ppos.x < 0. || ppos.y < 0.) {
                info!("\n!\n");
                continue;
            }

            let tileset_id = tileset.id().clone();
            let layer_id = build_mode.active_layer;
            let tile_name = &build_mode.tile_name;
            let pos: TilePos = (ppos.floor()).as_uvec2().into();
            info!("\n\n{:?} -> {:?} -> {:?} \n", p, ppos, pos);

            if let Some(group_id) = tileset.get_tile_group_id(tile_name) {
                let tile_id = TileId::new(*group_id, tileset_id);

                let place_mode = &PLACE_MODES[build_mode.mode];
                let error = match place_mode {
                    PlacementMode::Place => placer.place(tile_id, pos, 0u16, layer_id).err(),
                    PlacementMode::TryPlace => placer.try_place(tile_id, pos, 0u16, layer_id).err(),
                    PlacementMode::Toggle => placer.toggle(tile_id, pos, 0u16, layer_id).err(),
                    PlacementMode::ToggleMatch => {
                        placer.toggle_matching(tile_id, pos, 0u16, layer_id).err()
                    }
                    PlacementMode::Replace => placer.replace(tile_id, pos, 0u16, layer_id).err(),
                    PlacementMode::Remove => placer.remove(pos, 0u16, layer_id).err(),
                };

                if let Some(err) = error {
                    // Just print any errors to the console without panicking
                    eprintln!("Could not place tile: {}", err);
                }
            }
        }
    }
}

/// System controlling the "Build Mode"
fn on_keypress(
    keys: Res<Input<KeyCode>>,
    mut build_mode: ResMut<BuildMode>,
    mut serializer: TilemapSerializer,
    mut saved: ResMut<SavedMap>,
) {
    if keys.just_pressed(KeyCode::W) {
        build_mode.tile_name = String::from("sand");
    } else if keys.just_pressed(KeyCode::G) {
        build_mode.tile_name = String::from("water");
    } else if keys.just_pressed(KeyCode::D) {
        build_mode.tile_name = String::from("dirt");
    } else if keys.just_pressed(KeyCode::E) {
        build_mode.tile_name = String::from("grass");
    } else if keys.just_pressed(KeyCode::Up) {
        build_mode.mode = (build_mode.mode + 1) % PLACE_MODES.len();
    } else if keys.just_pressed(KeyCode::Down) {
        build_mode.mode = if build_mode.mode == 0 {
            PLACE_MODES.len() - 1
        } else {
            build_mode.mode - 1
        };
    } else if keys.just_pressed(KeyCode::Key1) {
        build_mode.active_layer = 0u16;
    } else if keys.just_pressed(KeyCode::Key2) {
        build_mode.active_layer = 1u16;
    } else if keys.just_pressed(KeyCode::Key3) {
        build_mode.active_layer = 2u16;
    } else if keys.just_pressed(KeyCode::Comma) {
        saved.map = serializer.save_maps();
        println!(
            "{}",
            serde_json::to_string(&saved.map.as_ref().unwrap()).unwrap()
        );
    } else if keys.just_pressed(KeyCode::Period) {
        if let Some(map) = &saved.map {
            serializer.load_maps(map);
        }
    }
}

const PLACE_MODES: &[PlacementMode] = &[
    PlacementMode::Place,
    PlacementMode::TryPlace,
    PlacementMode::Toggle,
    PlacementMode::ToggleMatch,
    PlacementMode::Replace,
    PlacementMode::Remove,
];

//    _    _ _    _ _____
//   | |  | | |  | |  __ \
//   | |__| | |  | | |  | |
//   |  __  | |  | | |  | |
//   | |  | | |__| | |__| |
//   |_|  |_|\____/|_____/
//
//

// All HUD related things from this point onwards
// No need to scroll further (unless you want to...)
#[derive(Component)]
struct HudText;
fn update_text(
    mut query: Query<&mut Text, With<HudText>>,
    tilesets: Tilesets,
    build_mode: Res<BuildMode>,
    saved: Res<SavedMap>,
) {
    for mut text in query.iter_mut() {
        text.sections[1].value = format!("{}", tilesets.get_by_name(MY_TILESET).is_some());
        text.sections[4].value = build_mode.tile_name.to_string();
        text.sections[7].value = format!("{}", build_mode.active_layer + 1);
        text.sections[9].value = String::from("3");
        text.sections[12].value = format!("{:?}", PLACE_MODES[build_mode.mode]);
        text.sections[25].style.color = if saved.map.is_some() {
            Color::rgba(0.75, 0.75, 0.75, 0.65)
        } else {
            Color::rgba(0.65, 0.65, 0.65, 0.25)
        };
    }
}

fn setup_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");
    let font_bold = asset_server.load("fonts/FiraSans-Bold.ttf");

    let style_key = TextStyle {
        font: font_bold,
        font_size: 20.0,
        color: Color::rgba(0.15, 0.15, 0.15, 0.75),
    };
    let style_value = TextStyle {
        font: font.clone(),
        font_size: 18.0,
        color: Color::rgba(0.75, 0.75, 0.75, 0.75),
    };
    let style_small = TextStyle {
        font,
        font_size: 14.0,
        color: Color::rgba(0.75, 0.75, 0.75, 0.65),
    };

    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                justify_content: JustifyContent::FlexStart,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(15.0),
                    left: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Tileset Loaded : ".to_string(),
                        style: style_key.clone(),
                    },
                    TextSection {
                        value: "false".to_string(),
                        style: style_value.clone(),
                    },
                    TextSection {
                        value: "\n".to_string(),
                        style: style_value.clone(),
                    },
                    TextSection {
                        value: "Tile : ".to_string(),
                        style: style_key.clone(),
                    },
                    TextSection {
                        value: "-".to_string(),
                        style: style_value.clone(),
                    },
                    TextSection {
                        value: "\n".to_string(),
                        style: style_value.clone(),
                    },
                    TextSection {
                        value: "Layer : ".to_string(),
                        style: style_key.clone(),
                    },
                    TextSection {
                        value: "-".to_string(),
                        style: style_value.clone(),
                    },
                    TextSection {
                        value: " / ".to_string(),
                        style: style_value.clone(),
                    },
                    TextSection {
                        value: "-".to_string(),
                        style: style_value.clone(),
                    },
                    TextSection {
                        value: "\n".to_string(),
                        style: style_value.clone(),
                    },
                    TextSection {
                        value: "Tool : ".to_string(),
                        style: style_key.clone(),
                    },
                    TextSection {
                        value: "-".to_string(),
                        style: style_value.clone(),
                    },
                    TextSection {
                        value: "\n".to_string(),
                        style: style_value.clone(),
                    },
                    TextSection {
                        value: "Options :\n".to_string(),
                        style: style_key.clone(),
                    },
                    TextSection {
                        value: "  ( 1 ) Edit Layer 1\n".to_string(),
                        style: style_small.clone(),
                    },
                    TextSection {
                        value: "  ( 2 ) Edit Layer 2\n".to_string(),
                        style: style_small.clone(),
                    },
                    TextSection {
                        value: "  ( 3 ) Edit Layer 3\n".to_string(),
                        style: style_small.clone(),
                    },
                    TextSection {
                        value: "  ( w ) Set tile to 'Wall'\n".to_string(),
                        style: style_small.clone(),
                    },
                    TextSection {
                        value: "  ( g ) Set tile to 'Glass'\n".to_string(),
                        style: style_small.clone(),
                    },
                    TextSection {
                        value: "  ( d ) Set tile to 'Dirt'\n".to_string(),
                        style: style_small.clone(),
                    },
                    TextSection {
                        value: "  ( e ) Set tile to 'Empty'\n".to_string(),
                        style: style_small.clone(),
                    },
                    TextSection {
                        value: "  ( Up ) Next Tool\n".to_string(),
                        style: style_small.clone(),
                    },
                    TextSection {
                        value: "  ( Down ) Previous Tool\n".to_string(),
                        style: style_small.clone(),
                    },
                    TextSection {
                        value: "  ( , ) Save current tilemap\n".to_string(),
                        style: style_small.clone(),
                    },
                    TextSection {
                        value: "  ( . ) Load saved tilemap\n".to_string(),
                        style: style_small,
                    },
                    TextSection {
                        value: "\nClick to add/remove tiles".to_string(),
                        style: style_key,
                    },
                ],
                alignment: Default::default(),
            },
            ..Default::default()
        })
        .insert(HudText);
}

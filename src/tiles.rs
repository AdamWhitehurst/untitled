use bevy::prelude::Plugin as BevyPlugin;
use bevy::prelude::*;
use bevy::render::render_resource::TextureUsages;
use bevy::{asset::LoadState, render::render_resource::FilterMode};
use bevy_ecs_tilemap::prelude::*;
use bevy_tileset_map::prelude::Tileset;

pub struct Plugin;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AssetState {
    Initial,
    Loading,
    Loaded,
}

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_state(AssetState::Initial)
            .init_resource::<Vec<Handle<Image>>>()
            .add_system_set(SystemSet::on_enter(AssetState::Initial).with_system(setup_tiles))
            .add_system_set(SystemSet::on_update(AssetState::Loading).with_system(watch_load));
    }
}

fn setup_tiles(
    asset_server: Res<AssetServer>,
    mut asset_state: ResMut<State<AssetState>>,
    mut images: ResMut<Vec<Handle<Image>>>,
) {
    let imgs = vec![
        //
        // "tilesets/iso.png",
        "characters/basic/basic_idle_01.png",
        "characters/basic/basic_idle_02.png",
        "characters/basic/basic_idle_03.png",
        "characters/basic/basic_idle_04.png",
    ];
    *images = imgs.iter().map(|s| asset_server.load(*s)).collect();
    asset_state
        .set(AssetState::Loading)
        .expect("AssetState::Loading");
}

fn watch_load(
    asset_server: Res<AssetServer>,
    images: Res<Vec<Handle<Image>>>,
    mut asset_state: ResMut<State<AssetState>>,
    mut textures: ResMut<Assets<Image>>,
) {
    if let LoadState::Loaded = asset_server.get_group_load_state(images.iter().map(|s| s.id)) {
        for h in images.iter() {
            if let Some(mut texture) = textures.get_mut(h.clone()) {
                texture.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING
                    | TextureUsages::COPY_SRC
                    | TextureUsages::COPY_DST;
            }
        }
        asset_state
            .set(AssetState::Loaded)
            .expect("AssetState::Loaded");
    }
}

pub fn load_map(commands: &mut Commands, map_query: &mut MapQuery, tileset: &Tileset) {
    let t_s = tileset.size();
    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);

    let map_size = MapSize(1, 1);
    let chunk_size = ChunkSize(64, 64);
    let tile_size = TileSize(18.0, 20.0);
    let texture_size = TextureSize(t_s.x, t_s.y);
    let grid_size = Vec2::new(16.0, 8.);

    let mut map_settings = LayerSettings::new(map_size, chunk_size, tile_size, texture_size);

    map_settings.filter = FilterMode::Nearest;
    map_settings.cull = false;
    map_settings.grid_size = grid_size;
    map_settings.mesh_type = TilemapMeshType::Isometric(IsoType::Diamond);

    for z in 0..1 {
        let (mut layer_builder, layer_entity) =
            LayerBuilder::new(commands, map_settings.clone(), 0u16, z);
        map.add_layer(commands, z, layer_entity);

        for x in 0..64 {
            for y in 0..64 {
                let position = TilePos(x, y);
                let _ = layer_builder.set_tile(
                    position,
                    TileBundle {
                        tile: Tile {
                            texture_index: 1,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                );
            }
        }
        map_query.build_layer(commands, layer_builder, tileset.texture().clone());
    }

    // let size: Vec2 = Vec2::new(
    //     (map_size.0 * chunk_size.0 * grid_size.x as u32) as f32 / 2.,
    //     (map_size.1 * chunk_size.1 * grid_size.y as u32) as f32 / 2.,
    // );
    let t = Transform::from_xyz(0., 4., 0.);
    // info!("T: {:#?}", t);
    commands
        .entity(map_entity)
        .insert(map)
        // .insert(Transform::default())
        .insert(t)
        .insert(GlobalTransform::default());
}

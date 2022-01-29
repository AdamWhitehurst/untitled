use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy::render::render_resource::TextureUsages;
use bevy_ecs_tilemap::prelude::*;
// use rand::{thread_rng, Rng};
pub struct TilesPlugin;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum AssetState {
    Initial,
    Loading,
    Loaded,
}

impl Plugin for TilesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Vec<Handle<Image>>>()
            .add_plugin(TilemapPlugin)
            .add_state(AssetState::Initial)
            .add_system_set(SystemSet::on_enter(AssetState::Initial).with_system(setup_tiles))
            .add_system_set(SystemSet::on_update(AssetState::Loading).with_system(watch_load))
            .add_system_set(SystemSet::on_enter(AssetState::Loaded).with_system(load_map));
    }
}

fn setup_tiles(
    asset_server: Res<AssetServer>,
    mut asset_state: ResMut<State<AssetState>>,
    mut handles: ResMut<Vec<Handle<Image>>>,
) {
    let tiles = vec![
        //
        "grass_tile.png",
        "dirt_tile.png",
        "tiles.png",
    ];
    *handles = tiles.iter().map(|s| asset_server.load(*s)).collect();
    asset_state.set(AssetState::Loading).expect("Loading");
}

fn watch_load(
    asset_server: Res<AssetServer>,
    handles: ResMut<Vec<Handle<Image>>>,
    mut asset_state: ResMut<State<AssetState>>,
    mut textures: ResMut<Assets<Image>>,
) {
    if let LoadState::Loaded = asset_server.get_group_load_state(handles.iter().map(|s| s.id)) {
        for h in handles.iter() {
            if let Some(mut texture) = textures.get_mut(h.clone()) {
                texture.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING
                    | TextureUsages::COPY_SRC
                    | TextureUsages::COPY_DST;
            }
        }
        asset_state.set(AssetState::Loaded).expect("Loaded");
    }
}

fn load_map(
    mut commands: Commands,
    handles: ResMut<Vec<Handle<Image>>>,
    mut map_query: MapQuery,
    mut textures: ResMut<Assets<Image>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut tab = TextureAtlasBuilder::default();

    for handle in handles.iter() {
        if let Some(t) = textures.get(handle) {
            tab.add_texture(handle.clone(), t);
        }
    }

    let ta = tab.finish(&mut textures).expect("texture_atlas");

    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);

    let texture_size = TextureSize(ta.size.x, ta.size.y);
    let tile_size = TileSize(16.0, 16.0);
    let chunk_size = ChunkSize(8, 8);
    let map_size = MapSize(2, 2);

    let (mut layer_builder, _) = LayerBuilder::new(
        &mut commands,
        LayerSettings::new(map_size, chunk_size, tile_size, texture_size),
        0u16,
        0u16,
    );

    layer_builder.set_all(TileBundle {
        tile: Tile {
            texture_index: 3u16,
            ..Default::default()
        },
        ..Default::default()
    });

    let layer_entity =
        map_query.build_layer(&mut commands, layer_builder, ta.clone().texture.clone());

    map.add_layer(&mut commands, 0u16, layer_entity);

    let map_dims = tile_size.0 * chunk_size.0 as f32 * (map_size.0 as f32 * 0.5);
    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(-map_dims, -map_dims, 0.0))
        .insert(GlobalTransform::default());
}

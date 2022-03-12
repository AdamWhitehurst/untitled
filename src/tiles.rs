use bevy::prelude::*;
use bevy::render::render_resource::TextureUsages;
use bevy::{asset::LoadState, render::render_resource::FilterMode};
use bevy_ecs_tilemap::prelude::*;
use rand::{thread_rng, Rng};
pub struct TilesPlugin;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AssetState {
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
        "micro/tilesets/iso.png",
        "micro/characters/basic/basic_idle_01.png",
        "micro/characters/basic/basic_idle_02.png",
        "micro/characters/basic/basic_idle_03.png",
        "micro/characters/basic/basic_idle_04.png",
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
    asset_server: Res<AssetServer>,
) {
    let texture_handle = asset_server.load("micro/tilesets/iso.png");

    // Create map entity and component:
    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);

    let map_size = MapSize(1, 1);
    let chunk_size = ChunkSize(64, 64);
    let tile_size = TileSize(18.0, 20.0);
    let texture_size = TextureSize(126., 120.);
    let grid_size = Vec2::new(16.0, 8.);

    let mut map_settings = LayerSettings::new(map_size, chunk_size, tile_size, texture_size);

    map_settings.filter = FilterMode::Nearest;
    map_settings.cull = false;
    map_settings.grid_size = grid_size;
    map_settings.mesh_type = TilemapMeshType::Isometric(IsoType::Diamond);

    // let (mut layer_0, layer_0_entity) =
    //     LayerBuilder::<TileBundle>::new(&mut commands, map_settings.clone(), 0u16, 0u16);
    // map.add_layer(&mut commands, 0u16, layer_0_entity);

    // Make 2 layers on "top" of the base map.
    for z in 0..1 {
        let (mut layer_builder, layer_entity) =
            LayerBuilder::new(&mut commands, map_settings.clone(), 0u16, z);
        map.add_layer(&mut commands, z, layer_entity);

        let mut random = thread_rng();

        for x in 0..64 {
            for y in 0..64 {
                let position = TilePos(x, y);
                let _ = layer_builder.set_tile(
                    position,
                    TileBundle {
                        tile: Tile {
                            texture_index: 0,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                );
            }
        }
        // layer_builder.settings.grid_size = grid_size;
        map_query.build_layer(&mut commands, layer_builder, texture_handle.clone());
    }

    // map_query.build_layer(&mut commands, layer_0, texture_handle.clone());

    commands
        .entity(map_entity)
        .insert(map)
        // .insert(Transform::from_xyz(0., 0., 0.))
        .insert(Transform::from_xyz(0., 96., 1.))
        .insert(GlobalTransform::default());
}

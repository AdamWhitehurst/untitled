use bevy::asset::{HandleId, LoadState};
use bevy::prelude::*;

mod sprite;

#[derive(Default, Clone)]
struct ImageHandles {
    handles: Vec<HandleId>,
    atlas_loaded: bool,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<ImageHandles>()
        .add_startup_stage(
            "load asset",
            SystemStage::parallel().with_system(setup_tiles),
        )
        .add_startup_system(setup)
        .add_startup_system(setup_tiles)
        .add_system(animate_sprite_system)
        .run();
}

fn animate_sprite_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}
fn setup_tiles(mut commands: Commands, asset_server: Res<AssetServer>) {
    // let mut handles: Vec<Handle<Image>> = Vec<Handle<Image>>::new();
    let handles: Vec<HandleId> = sprite::tiles
        .iter()
        .map(|&s| asset_server.load::<Image, &str>(s).id)
        .collect();
}
fn watch_load() {
    if let LoadState::Loaded = asset_server.get_group_load_state(handles) {}
}
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("char/gabe/gabe-idle-run.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.1, true));
}

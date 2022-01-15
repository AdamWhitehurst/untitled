use bevy::asset::{LoadContext, LoadState};
use bevy::prelude::*;
use bevy::{
    log::prelude::*,
    prelude::{App as BevyApp, AssetServer, Commands, Res, ResMut},
    window::WindowDescriptor,
    DefaultPlugins,
};
use kayak_ui::bevy::{BevyContext, BevyKayakUIPlugin, FontMapping, UICameraBundle};
use kayak_ui::core::{
    render, rsx,
    styles::{Style, StyleProp, Units},
    use_state, widget, Color, Index,
};
use kayak_ui::widgets::{App, OnChange, Text, TextBox, Window};
mod sprite;

#[widget]
fn TextBoxExample(context: &mut KayakContext) {
    let (value, set_value, _) = use_state!("I started with a value!".to_string());
    let (empty_value, set_empty_value, _) = use_state!("".to_string());
    let (red_value, set_red_value, _) = use_state!("This text is red".to_string());

    let input_styles = Style {
        top: StyleProp::Value(Units::Pixels(10.0)),
        ..Default::default()
    };

    let red_text_styles = Style {
        color: StyleProp::Value(Color::new(1., 0., 0., 1.)),
        ..input_styles.clone()
    };

    let on_change = OnChange::new(move |event| {
        set_value(event.value);
    });

    let on_change_empty = OnChange::new(move |event| {
        set_empty_value(event.value);
    });

    let on_change_red = OnChange::new(move |event| {
        set_red_value(event.value);
    });

    rsx! {
        <Window position={(50.0, 50.0)} size={(300.0, 300.0)} title={"TextBox Example".to_string()}>
            <TextBox styles={Some(input_styles)} value={value} on_change={Some(on_change)} />
            <TextBox
                styles={Some(input_styles)}
                value={empty_value}
                on_change={Some(on_change_empty)}
                placeholder={Some("This is a placeholder".to_string())}
            />
            <TextBox styles={Some(red_text_styles)} value={red_value} on_change={Some(on_change_red)} />
        </Window>
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum AssetState {
    Initial,
    Loading,
    Loaded,
}

fn main() {
    let mut app = BevyApp::new();

    app.insert_resource(WindowDescriptor {
        width: 1270.0,
        height: 720.0,
        title: String::from(""),
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(BevyKayakUIPlugin)
    .init_resource::<Vec<Handle<Image>>>()
    .add_startup_system(startup_ui)
    .add_state(AssetState::Initial)
    .add_system_set(SystemSet::on_enter(AssetState::Initial).with_system(setup_tiles))
    .add_system_set(SystemSet::on_update(AssetState::Loading).with_system(watch_load))
    .add_system_set(SystemSet::on_enter(AssetState::Loaded).with_system(setup))
    .add_system_set(SystemSet::on_update(AssetState::Loaded).with_system(animate_sprite_system))
    .run();
}

fn startup_ui(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(UICameraBundle::new());

    font_mapping.add(asset_server.load("roboto.kayak_font"));

    let context = BevyContext::new(|context| {
        render! {
            <App>
            <TextBoxExample />
            </App>
        }
    });

    commands.insert_resource(context);
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

fn setup_tiles(
    asset_server: Res<AssetServer>,
    mut asset_state: ResMut<State<AssetState>>,
    mut handles: ResMut<Vec<Handle<Image>>>,
) {
    *handles = sprite::TILES
        .iter()
        .map(|&s| asset_server.load::<Image, &str>(s))
        .collect();

    asset_state.set(AssetState::Loading).expect("Loading");
}

fn watch_load(
    asset_server: Res<AssetServer>,
    handles: ResMut<Vec<Handle<Image>>>,
    mut asset_state: ResMut<State<AssetState>>,
) {
    if let LoadState::Loaded = asset_server.get_group_load_state(handles.iter().map(|s| s.id)) {
        asset_state.set(AssetState::Loaded).expect("Loaded");
    }
}
fn setup(
    mut commands: Commands,
    mut textures: ResMut<Assets<Image>>,
    handles: ResMut<Vec<Handle<Image>>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut texture_atlas = TextureAtlasBuilder::default();

    for handle in handles.iter() {
        if let Some(t) = textures.get(handle) {
            texture_atlas.add_texture(handle.clone(), t);
        }
    }

    let texture_atlas_handle =
        texture_atlases.add(texture_atlas.finish(&mut textures).expect("texture_atlas"));
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.1, true));
}

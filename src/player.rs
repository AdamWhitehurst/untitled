use std::time::Duration;

use bevy::core::FixedTimestep;
use bevy::prelude::Plugin as BevyPlugin;
use bevy::prelude::*;

use crate::camera::CameraFollow;
use crate::mouse::GlobalCursorPosition;
use crate::sprite::CharacterAnimation;

pub struct Plugin;
#[derive(Component, Debug, Default, Clone, Copy)]
pub struct PlayerCharacter;

pub enum PlayerState {
    Idle,
}

// pub type PlayerAnimation = Vec<>
//
// impl PlayerState {
//     fn to_animation(&self) -> Into
// }

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_stage_after(
            CoreStage::Update,
            "fixed_update",
            SystemStage::parallel()
                // Set the stage criteria to run the system at the target
                //  rate per seconds
                .with_run_criteria(FixedTimestep::steps_per_second(60.0))
                .with_system(player_movement)
                .with_system(animate_sprite),
        )
        // .add_startup_system(setup);
        .add_system_set(SystemSet::on_enter(crate::tiles::AssetState::Loaded).with_system(setup));
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
) {
    let texture_handles: Vec<Handle<Image>> = vec![
        asset_server.load("micro/characters/basic/basic_idle_01.png"),
        asset_server.load("micro/characters/basic/basic_idle_02.png"),
        asset_server.load("micro/characters/basic/basic_idle_03.png"),
        asset_server.load("micro/characters/basic/basic_idle_04.png"),
    ];
    let mut tab = TextureAtlasBuilder::default(); //::add_texture(&mut self, texture_handle, texture)//from_grid(texture_handle, Vec2::new(64., 64.), 13, 21);
    texture_handles.iter().for_each(|t| {
        tab.add_texture(t.clone(), textures.get(t).expect("character tex setup"));
    });
    let sprite_bundle = SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_xyz(0.0, 0.0, 100.0),
            ..Default::default()
        };
        sprite_bundle.
    let texture_atlas_handle =
        texture_atlases.add(tab.finish(&mut textures).expect("texture_atlas_builder"));
    commands
        .spawn_bundle(sprite_bundle)
        .insert(CharacterAnimation(
            Timer::new(Duration::new(1, 0), true),
            false,
            4,
        ))
        .insert(CameraFollow)
        .insert(PlayerCharacter);
}

// A simple camera system for moving and zooming the camera.
fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    cursor_tile: Res<GlobalCursorPosition>,
    mut query: Query<&mut Transform, With<PlayerCharacter>>,
) {
    for mut t in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::A) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::S) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
        }

        let z = t.translation.z;
        t.translation += direction * 1.0;
        t.translation = t.translation.round();
        // Important! We need to restore the Z values when moving the camera around.
        // Bevy has a specific camera setup and this can mess with how our layers are shown.
        t.translation.z = z;
    }
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&mut CharacterAnimation, &mut TextureAtlasSprite)>,
) {
    for (mut timer, mut sprite) in query.iter_mut() {
        info!("Yes");
        timer.0.tick(time.delta());
        if timer.1 && timer.0.just_finished() {
            // sprite.index = (timer.2 * 13) + ((sprite.index + 1) % (9 * timer.2));
            sprite.index = (sprite.index + 1) % timer.2;
        }
    }
}

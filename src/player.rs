use std::time::Duration;

use bevy::core::FixedTimestep;
use bevy::prelude::Plugin as BevyPlugin;
use bevy::prelude::*;

use crate::camera::CameraFollow;
use crate::sprite::CharacterAnimation;

pub struct Plugin;
#[derive(Component, Debug, Default, Clone, Copy)]
pub struct PlayerCharacter(Vec3);

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(player_input)
            .add_system(animate_sprite)
            .add_stage_after(
                CoreStage::Update,
                "player_move",
                SystemStage::parallel()
                    .with_run_criteria(FixedTimestep::steps_per_second(2.5))
                    .with_system(move_player),
            )
            .add_system_set(
                SystemSet::on_enter(crate::tiles::AssetState::Loaded).with_system(setup),
            );
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
) {
    let texture_handles: Vec<Handle<Image>> = vec![
        asset_server.load("characters/basic/basic_idle_01.png"),
        asset_server.load("characters/basic/basic_idle_02.png"),
        asset_server.load("characters/basic/basic_idle_03.png"),
        asset_server.load("characters/basic/basic_idle_04.png"),
    ];
    let mut tab = TextureAtlasBuilder::default(); //::add_texture(&mut self, texture_handle, texture)//from_grid(texture_handle, Vec2::new(64., 64.), 13, 21);
    texture_handles.iter().for_each(|t| {
        tab.add_texture(t.clone(), textures.get(t).expect("character tex setup"));
    });
    let texture_atlas_handle =
        texture_atlases.add(tab.finish(&mut textures).expect("texture_atlas_builder"));
    let sprite_bundle = SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_xyz(0., 0., 100.0),
        ..Default::default()
    };
    commands
        .spawn_bundle(sprite_bundle)
        .insert(Transform::from_xyz(0., 0., 100.))
        .insert(CharacterAnimation(
            Timer::new(Duration::from_secs_f32(0.25), true),
            true,
            4,
        ))
        .insert(CameraFollow)
        .insert(PlayerCharacter::default());
}

fn player_input(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut PlayerCharacter>) {
    if let Some(mut pc) = query.get_single_mut().ok() {
        let mut dir = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::A) {
            dir.x = -1.0;
        }

        if keyboard_input.pressed(KeyCode::D) {
            dir.x = 1.0;
        }

        if keyboard_input.pressed(KeyCode::W) {
            dir.y = 1.0;
        }

        if keyboard_input.pressed(KeyCode::X) {
            dir.y = -1.0;
        }

        if keyboard_input.pressed(KeyCode::E) {
            dir.x = 1.0;
            dir.y = 1.0;
        }

        if keyboard_input.pressed(KeyCode::Q) {
            dir.x = -1.0;
            dir.y = 1.0;
        }

        if keyboard_input.pressed(KeyCode::Z) {
            dir.x = -1.0;
            dir.y = -1.0;
        }

        if keyboard_input.pressed(KeyCode::C) {
            dir.x = 1.0;
            dir.y = -1.0;
        }

        if dir != Vec3::ZERO {
            dir.x *= 16.0;
            dir.y *= 8.0;

            if dir.x != 0.0 && dir.y != 0.0 {
                dir *= 0.5;
            }

            *pc = PlayerCharacter(dir);
        }
    }
}

fn move_player(
    mut query: Query<(Entity, &mut Transform, &mut PlayerCharacter)>,
    commands: Commands,
) {
    for (e, mut t, mut pc) in query.iter_mut() {
        t.translation += pc.0;
        *pc = PlayerCharacter(Vec3::ZERO);

        // commands
        // .entity(e)
        // .insert(CameraFollow(t.translation.clone()));
    }
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &mut CharacterAnimation,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    for (mut timer, mut sprite, ta) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.1 && timer.0.just_finished() {
            let taa = texture_atlases.get(ta).expect("texture atlas for anim");
            sprite.index = (sprite.index + 1) % taa.textures.len();
        }
    }
}

use std::time::Duration;

use bevy::core::FixedTimestep;
use bevy::prelude::Plugin as BevyPlugin;
use bevy::prelude::*;
use bevy_ecs_tilemap::TilePos;

use crate::camera::CameraFollow;
use crate::pathfinding::TilePath;
use crate::sprite::CharacterAnimation;
use crate::utils::*;

pub struct Plugin;

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct PlayerCharacter;

#[derive(Component, Debug, Clone, Copy)]
pub enum Animation {
    Idle,
    Walk,
    Dance,
    Jump,
}

impl Default for Animation {
    fn default() -> Self {
        Animation::Idle
    }
}

impl Animation {
    pub fn frames(&self) -> Vec<&'static str> {
        use Animation::*;
        match *self {
            Idle => vec![
                "characters/basic/basic_idle_01.png",
                "characters/basic/basic_idle_02.png",
                "characters/basic/basic_idle_03.png",
                "characters/basic/basic_idle_04.png",
            ],
            Walk => vec![
                "characters/basic/basic_run_01.png",
                "characters/basic/basic_run_02.png",
                "characters/basic/basic_run_03.png",
                "characters/basic/basic_run_04.png",
                "characters/basic/basic_run_05.png",
                "characters/basic/basic_run_06.png",
                "characters/basic/basic_run_07.png",
                "characters/basic/basic_run_08.png",
            ],
            Dance | Jump => vec![
                "characters/basic/basic_jump_01.png",
                "characters/basic/basic_jump_02.png",
                "characters/basic/basic_jump_03.png",
                "characters/basic/basic_jump_04.png",
                "characters/basic/basic_jump_05.png",
                "characters/basic/basic_jump_06.png",
                "characters/basic/basic_jump_07.png",
                "characters/basic/basic_jump_08.png",
                "characters/basic/basic_jump_09.png",
                "characters/basic/basic_jump_10.png",
            ],
        }
    }
    pub fn repeats(&self) -> bool {
        use Animation::*;
        match *self {
            Walk | Idle | Dance => true,
            Jump => false,
        }
    }
}

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(player_input)
            .add_system(animate_sprite)
            .add_system(tile_trans)
            .add_stage_after(
                CoreStage::Update,
                "player_move",
                SystemStage::parallel()
                    .with_run_criteria(FixedTimestep::steps_per_second(10.))
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
        }
    }
}

fn tile_trans(query: Query<(Entity, &TilePos, &GlobalTransform)>) {
    for (_, t, tr) in query.iter() {
        info!("{:?} {:?}", t, tr.translation);
    }
}

fn move_player(
    mut query: Query<(Entity, &mut Transform, &mut PlayerCharacter, &TilePath)>,
    mut commands: Commands,
) {
    for (e, mut t, _, path) in query.iter_mut() {
        let mut updated_path = path.0.clone();
        let opt_next = updated_path.pop();
        if let Some(pos) = opt_next {
            let wpos = iso_to_world(&Vec2::new(pos.0 as f32, pos.1 as f32));
            t.translation = wpos.extend(100.);
            commands.entity(e).insert(TilePath(updated_path));
        } else {
            commands.entity(e).remove::<TilePath>();
        }
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

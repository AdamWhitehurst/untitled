use bevy::core::FixedTimestep;
use bevy::prelude::Plugin as BevyPlugin;
use bevy::prelude::*;

use crate::camera::CameraFollow;
use crate::sprite::CharacterAnimation;

pub struct Plugin;
#[derive(Component, Debug, Default, Clone, Copy)]
pub struct PlayerCharacter;

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
        .add_startup_system(setup);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("micro/characters/basic/basic_idle_01.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64., 64.), 13, 21);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_xyz(0.0, 0.0, 100.0),
            ..Default::default()
        })
        .insert(CameraFollow)
        .insert(PlayerCharacter);
}

// A simple camera system for moving and zooming the camera.
fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
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
        timer.0.tick(time.delta());
        if timer.1 && timer.0.just_finished() {
            sprite.index = (timer.2 * 13) + ((sprite.index + 1) % (9 * timer.2));
            info!("{:?}", sprite.index);
        }
    }
}

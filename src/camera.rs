use std::time::Duration;

use bevy::prelude::Plugin as BevyPlugin;
use bevy::prelude::*;
use bevy::render::camera::Camera;
use bevy_tweening::lens::TransformPositionLens;
use bevy_tweening::*;

#[derive(Default, Debug, Component, Clone, Copy)]
pub struct CameraFollow;
pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TweeningPlugin)
            .add_startup_system(setup)
            .insert_resource(Msaa { samples: 4 })
            .add_system(follow_character);
    }
}

#[derive(Component)]
struct Cursor;

fn setup(mut commands: Commands) {
    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    camera_bundle.orthographic_projection.scale = 0.25;
    commands.spawn_bundle(camera_bundle);
}

fn follow_character(
    mut transforms: Query<&mut Transform>,
    q_cam: Query<Entity, With<Camera>>,
    q_follow: Query<Entity, With<CameraFollow>>,
    mut commands: Commands,
) {
    if let (Ok(src_e), Ok(flw_e)) = (q_follow.get_single(), q_cam.get_single()) {
        let mut tr = None;
        if let Ok(f_t) = transforms.get(src_e) {
            tr = Some(f_t.translation.clone());
        }

        if let (Ok(t), Some(tr)) = (transforms.get_mut(flw_e), tr) {
            let mut new_t = t.translation;
            new_t.x = tr.x;
            new_t.y = tr.y;
            new_t.z = t.translation.z;

            // Create a single animation (tween) to move an entity.
            let tween = Tween::new(
                // Use a quadratic easing on both endpoints.
                EaseFunction::SineInOut,
                // Loop animation back and forth.
                TweeningType::Once,
                // Animation time (one way only; for ping-pong it takes 2 seconds
                // to come back to start).
                Duration::from_millis(33),
                // The lens gives access to the Transform component of the Entity,
                // for the Animator to animate it. It also contains the start and
                // end values respectively associated with the progress ratios 0. and 1.
                TransformPositionLens {
                    start: t.translation,
                    end: new_t,
                },
            );

            let a = Animator::new(tween);
            commands.entity(flw_e).insert(a);
        }
    }
}

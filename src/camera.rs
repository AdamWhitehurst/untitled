use bevy::prelude::Plugin as BevyPlugin;
use bevy::prelude::*;
use bevy::render::camera::{Camera, DepthCalculation, ScalingMode};

#[derive(Default, Debug, Component, Clone, Copy)]
pub struct CameraFollow;
pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
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
) {
    if let (Ok(src_e), Ok(flw_e)) = (q_follow.get_single(), q_cam.get_single()) {
        let mut tr = None;
        if let Ok(f_t) = transforms.get(src_e) {
            tr = Some(f_t.translation.clone());
        }

        if let (Ok(mut t), Some(tr)) = (transforms.get_mut(flw_e), tr) {
            let z = t.translation.z;
            t.translation.x = tr.x;
            t.translation.y = tr.y;
            t.translation.z = z;
        }
    }
}

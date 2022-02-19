use bevy::prelude::Plugin as BevyPlugin;
use bevy::prelude::*;
use bevy::render::camera::Camera;
use bevy_tiled_camera::*;

#[derive(Default, Debug, Component, Clone, Copy)]
pub struct CameraFollow;
pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_plugin(TiledCameraPlugin)
            .insert_resource(Msaa { samples: 4 })
            .add_system(global_cursor)
            .add_system(follow_character);
    }
}

#[derive(Component)]
struct Cursor;

fn global_cursor(
    windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform, &TiledProjection)>,
    mut q_cursor: Query<&mut Transform, With<Cursor>>,
) {
    let window = windows.get_primary().unwrap();

    if let Some(pos) = window.cursor_position() {
        for (cam, cam_transform, proj) in q_camera.iter() {
            if let Some(p) = proj.screen_to_world(cam, &windows, cam_transform, pos) {
                let mut t = q_cursor.single_mut();
                *t.translation = *p;
            }
        }
    }
}

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
            t.translation.x = tr.x;
            t.translation.y = tr.y;
        }
    }
}

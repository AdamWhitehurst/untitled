use bevy::prelude::Plugin as BevyPlugin;
use bevy::prelude::*;
use bevy::render::camera::Camera;
use bevy_tiled_camera::*;
pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_plugin(TiledCameraPlugin)
            .insert_resource(Msaa { samples: 4 })
            .add_system(global_cursor);
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
    // commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let camera_bundle = TiledCameraBundle::new()
        .with_tile_count([1000, 800])
        .with_target_resolution(16, [1280, 720]);
    commands.spawn_bundle(camera_bundle);

    let col = Color::rgba(1.0, 1.0, 1.0, 0.35);
    let cursor = SpriteBundle {
        sprite: Sprite {
            color: col,
            custom_size: Some(Vec2::ONE),
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 2.0),
        ..Default::default()
    };
    commands.spawn_bundle(cursor).insert(Cursor);
}

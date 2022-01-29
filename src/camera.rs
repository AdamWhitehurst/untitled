use bevy::prelude::Plugin as BevyPlugin;
use bevy::prelude::*;
use bevy::{core::Time, input::Input, math::Vec3, render::camera::Camera};
use bevy_tiled_camera::*;
pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_plugin(TiledCameraPlugin)
            .add_system(cursor_system)
            .add_system(movement);
    }
}

#[derive(Component)]
struct Cursor;
fn cursor_system(
    input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform, &TiledProjection)>,
    mut q_cursor: Query<(&mut Transform, &mut Visibility), With<Cursor>>,
    // mut q_sprite: Query<&mut Sprite>,
) {
    let window = windows.get_primary().unwrap();

    if let Some(pos) = window.cursor_position() {
        for (cam, cam_transform, proj) in q_camera.iter() {
            if let Some(p) = proj.screen_to_world(cam, &windows, cam_transform, pos) {
                if let Some(mut p) = proj.world_to_tile_center(cam_transform, p) {
                    p.z = 2.0;

                    let (mut cursor_transform, mut v) = q_cursor.single_mut();
                    v.is_visible = true;

                    cursor_transform.translation = p;

                    if input.just_pressed(MouseButton::Left) {
                        let i = proj.world_to_tile(cam_transform, p).unwrap();
                        info!("{:?}", i);
                    }

                    return;
                }
            }
        }
    }

    let (_, mut v) = q_cursor.single_mut();
    v.is_visible = false;
}

fn setup(mut commands: Commands) {
    // commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let camera_bundle = TiledCameraBundle::new()
        .with_pixels_per_tile(16)
        .with_tile_count([640, 480]);

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

// A simple camera system for moving and zooming the camera.
pub fn movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut TiledProjection), With<Camera>>,
) {
    for (mut t, mut tp) in query.iter_mut() {
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

        if keyboard_input.pressed(KeyCode::Z) {
            let tc = tp.tile_count();
            tp.as_mut().set_tile_count([tc.x - 1, tc.y - 1]);
        }

        if keyboard_input.pressed(KeyCode::X) {
            let tc = tp.tile_count();
            tp.as_mut().set_tile_count([tc.x + 1, tc.y + 1]);
        }

        let z = t.translation.z;
        t.translation += time.delta_seconds() * direction * 10.;
        // Important! We need to restore the Z values when moving the camera around.
        // Bevy has a specific camera setup and this can mess with how our layers are shown.
        t.translation.z = z;
    }
}

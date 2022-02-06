use bevy::prelude::Plugin as BevyPlugin;
use bevy::prelude::*;
use bevy::core::FixedTimestep;

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
                .with_system(movement),
        );
    }
}

// A simple camera system for moving and zooming the camera.
pub fn movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform), With<PlayerCharacter>>,
) {
    for (mut t) in query.iter_mut() {
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

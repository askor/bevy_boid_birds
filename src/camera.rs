use bevy::prelude::*;
use crate::{GameState, actions::{Actions, self}};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup_camera))
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(rotate_camera))
        ;
    }
}

fn setup_camera(
    mut commands: Commands,
) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0., 2., 7.),
            ..default()
        },
        Name::new("Camera"),
    ));
}

fn rotate_camera(
    mut query: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let mut movement = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        movement = -1.0;
    } else if keyboard_input.pressed(KeyCode::Right) {
        movement = 1.0;
    } else {
        return;
    }

    let mut cam_transform = query.single_mut();
    let focus = Vec3::new(0., 1., 0.);

    cam_transform.translate_around(focus, Quat::from_rotation_y(time.delta_seconds() * movement ));
    cam_transform.look_at(focus, Vec3::Y)
}
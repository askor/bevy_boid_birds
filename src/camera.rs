use bevy::prelude::*;
use crate::GameState;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup_camera));
    }
}

fn setup_camera(
    mut commands: Commands,
) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0., 2., -4.),
            ..default()
        },
        Name::new("Camera"),
    ));
}
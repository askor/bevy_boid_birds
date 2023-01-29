use bevy::prelude::*;
use crate::GameState;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(setup_scene));
    }
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let box_length = 100.0;
    let box_height = 10.0;
    
    commands.spawn((
        PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(2.0 * box_length, 2.0 * box_height, 2.0 * box_length))),
                material: materials.add(Color::rgb_u8(100, 158, 100).into()),
                transform: Transform::from_xyz(0., -14., 0.),
                ..default()
        },
        Name::new("Ground")
    ));
}
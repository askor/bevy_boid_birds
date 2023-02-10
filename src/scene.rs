use bevy::prelude::*;
use bevy_atmosphere::prelude::AtmospherePlugin;
use crate::GameState;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(AtmospherePlugin)
        .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup_scene));
    }
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let box_length = 100.0;
    let box_height = 10.0;

    // Sun
    commands.spawn((
        DirectionalLightBundle::default(),
        Name::new("Sun"),
    ));
    
    // Ground
    commands.spawn((
        PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(2.0 * box_length, 2.0 * box_height, 2.0 * box_length))),
                material: materials.add(Color::rgb_u8(100, 158, 100).into()),
                transform: Transform::from_xyz(0., -40., 0.),
                ..default()
        },
        Name::new("Ground")
    ));

    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 1., 0.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(3.0, 4.0, -1.0),
        ..default()
    });
}
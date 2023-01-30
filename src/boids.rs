use bevy::prelude::*;
use crate::{GameState, loading::SceneAssets};

pub struct BoidsPlugin;

impl Plugin for BoidsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_boids))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(move_boids))
            ;
    }
}

#[derive(Bundle)]
struct BoidBundle {
    boid: Boid,
    velocity: Velocity,
    scene_bundle: SceneBundle,
}

#[derive(Component)]
struct Boid;

#[derive(Component)]
struct Velocity(Vec3);

fn spawn_boids (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    scenes: Res<SceneAssets>,
) {
    commands.spawn((BoidBundle {
        boid: Boid,
        velocity: Velocity(Vec3::new(1.0, 0., 0.)),
        scene_bundle: SceneBundle {
            scene: scenes.bird.clone(),
            transform: Transform::from_xyz(-10., 0., 0.).with_scale(Vec3::splat(0.02)),
            ..default()
        }},
        Name::new("Boid"),
    ));
}

fn move_boids (
    mut boid_query: Query<(&mut Transform, &Velocity), With<Boid>>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in boid_query.iter_mut() {
        let focus = transform.translation - velocity.0;
        let up = transform.local_y();
        transform.look_at(focus, up);
        transform.translation += velocity.0 * time.delta_seconds();
    }
}
use std::num;

use bevy::prelude::*;
use rand::Rng;
use crate::{GameState, loading::SceneAssets};

const SPEED: f32 = 6.0;

pub struct BoidsPlugin;

impl Plugin for BoidsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_boids))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(move_boids))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(stay_inside_bounds))
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
    let mut rng = rand::thread_rng();
    
    for i in 0..200 {
        // let random: f32 = rng.gen();
        commands.spawn((BoidBundle {
            boid: Boid,
            velocity: Velocity(Vec3::new(rng.gen_range(-10..10) as f32, rng.gen_range(-3..3) as f32, rng.gen_range(-10..10) as f32).normalize()),
            scene_bundle: SceneBundle {
                scene: scenes.bird.clone(),
                transform: Transform::from_xyz(rng.gen_range(-10..10) as f32, rng.gen_range(-10..10) as f32, rng.gen_range(-10..10) as f32).with_scale(Vec3::splat(0.02)),
                ..default()
            }},
            Name::new("Boid"),
        ));
    }
}

fn move_boids (
    mut boid_query: Query<(&mut Transform, &Velocity), With<Boid>>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in boid_query.iter_mut() {
        let focus = transform.translation - velocity.0;
        let up = transform.local_y();
        transform.look_at(focus, up);
        transform.translation += velocity.0 * time.delta_seconds() * SPEED;
    }
}

fn avoid_nearby (
    mut q_boid_trans: Query<&mut Transform, With<Boid>>,
) {
    for mut boid in q_boid_trans.iter_mut() {
        let mut entities: [Entity; 10] = [Entity::from_raw(0); 10];
    }
}

fn stay_inside_bounds (
    mut boid_query: Query<&mut Transform, With<Boid>>,
) {
    let bound_limit = 14.0;
    
    for mut transform in boid_query.iter_mut() {
        if transform.translation.x > bound_limit {
            transform.translation.x = -bound_limit;
        }
        else if transform.translation.x < -bound_limit {
            transform.translation.x = bound_limit;
        }

        if transform.translation.y > bound_limit {
            transform.translation.y = -bound_limit;
        }
        else if transform.translation.y < -bound_limit {
            transform.translation.y = bound_limit;
        }

        if transform.translation.z > bound_limit {
            transform.translation.z = -bound_limit;
        }
        else if transform.translation.z < -bound_limit {
            transform.translation.z = bound_limit;
        }
    }
}
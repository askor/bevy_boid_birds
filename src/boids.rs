use std::num;

use bevy::{prelude::*, utils::HashMap};
use rand::Rng;
use crate::{GameState, loading::SceneAssets};

const SPEED: f32 = 6.0;

pub struct BoidsPlugin;

impl Plugin for BoidsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(init_grid_map)
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
    mut grid_map: ResMut<GridMap>,
    scenes: Res<SceneAssets>,
) {
    let mut rng = rand::thread_rng();
    
    for i in 0..200 {
        let pos = Vec3::new(rng.gen_range(-10..10) as f32, rng.gen_range(-10..10) as f32, rng.gen_range(-10..10) as f32);

        let id = commands.spawn((BoidBundle {
            boid: Boid,
            velocity: Velocity(Vec3::new(rng.gen_range(-10..10) as f32, rng.gen_range(-3..3) as f32, rng.gen_range(-10..10) as f32).normalize()),
            scene_bundle: SceneBundle {
                scene: scenes.bird.clone(),
                transform: Transform::from_translation(pos).with_scale(Vec3::splat(0.02)),
                ..default()
            }},
            Name::new("Boid"),
        )).id();

        let index = get_cell_index(pos);

        let mut entities = match grid_map.map.get_mut(&get_key(index.0, index.1, index.2)) {
            Some(v) => v,
            None => panic!("Tried index {:?}", index),
        };
        entities.push(id);
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

const BOUNDS: f32 = 100.0;
const DIMENSIONS: i32 = 10;

#[derive(Resource)]
struct GridMap {
    map: HashMap<String, Vec<Entity>>
}

fn init_grid_map (
    mut commands: Commands
) {
    let mut map: HashMap<String, Vec<Entity>> = HashMap::new();
    for x in 0..DIMENSIONS {
        for y in 0..DIMENSIONS {
            for z in 0..DIMENSIONS {
                let key = get_key(x, y, z);
                map.insert(key, Vec::new());
                // println!("Index: {}:{}:{}", x, y, z);
            }
        }
    }
    println!("TEST: {:?}", get_cell_index(Vec3::new(-BOUNDS/2., 0., 0.)));
    commands.insert_resource(GridMap { map });
}

fn get_key (x: i32, y: i32, z: i32) -> String {
    return format!("{}{}{}", x, y, z);
}

fn get_cell_index (pos: Vec3) -> (i32, i32, i32) {
    let min = -BOUNDS/2.0;
    let max = BOUNDS/2.0;

    let x = (pos.x + BOUNDS/2.0).clamp(min, max).floor() as i32 / DIMENSIONS;
    let y = (pos.y + BOUNDS/2.0).clamp(min, max).floor() as i32 / DIMENSIONS;
    let z = (pos.z + BOUNDS/2.0).clamp(min, max).floor() as i32 / DIMENSIONS;
    return (x, y, z);
}

// fn avoid_nearby (
//     mut q_boid_trans: Query<&mut Transform, With<Boid>>,
// ) {
//     for mut boid in q_boid_trans.iter_mut() {
//         let mut entities: [Entity; 10] = [Entity::from_raw(0); 10];
//     }
// }

fn stay_inside_bounds (
    mut boid_query: Query<&mut Transform, With<Boid>>,
) {    
    for mut transform in boid_query.iter_mut() {
        if transform.translation.x > (BOUNDS/2.) {
            transform.translation.x = -(BOUNDS/2.);
        }
        else if transform.translation.x < -(BOUNDS/2.) {
            transform.translation.x = (BOUNDS/2.);
        }

        if transform.translation.y > (BOUNDS/2.) {
            transform.translation.y = -(BOUNDS/2.);
        }
        else if transform.translation.y < -(BOUNDS/2.) {
            transform.translation.y = (BOUNDS/2.);
        }

        if transform.translation.z > (BOUNDS/2.) {
            transform.translation.z = -(BOUNDS/2.);
        }
        else if transform.translation.z < -(BOUNDS/2.) {
            transform.translation.z = (BOUNDS/2.);
        }
    }
}
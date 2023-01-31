use std::num;

use bevy::{prelude::*, utils::HashMap};
use rand::Rng;
use crate::{GameState, loading::SceneAssets};

const SPEED: f32 = 1.0;

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
    
    for i in 0..1 {
        let pos = Vec3::new(rng.gen_range(-9..9) as f32, rng.gen_range(-9..9) as f32, rng.gen_range(-9..9) as f32);

        println!("Pos: {}", pos);
        println!("Calc index: {:?}", get_cell_index(pos));

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

        let mut entities = match grid_map.map.get_mut(&get_key(index)) {
            Some(v) => v,
            None => panic!("Tried index {:?}", index),
        };
        entities.push(id);
    }
}

fn move_boids (
    mut boid_query: Query<(&mut Transform, Entity, &Velocity), With<Boid>>,
    mut grid: ResMut<GridMap>,
    time: Res<Time>,
) {
    for (mut transform, entity, velocity) in boid_query.iter_mut() {
        let focus = transform.translation - velocity.0;
        let up = transform.local_y();
        transform.look_at(focus, up);
        
        let prev = transform.translation;
        let new_pos = velocity.0 * time.delta_seconds() * SPEED;

        update_grid(prev, new_pos, &mut grid, entity);
    }
}

fn update_grid(prev: Vec3, new_pos: Vec3, grid: &mut ResMut<GridMap>, entity: Entity) {
    let index0 = get_cell_index(prev);
    let index1 = get_cell_index(new_pos);
    println!("index: {:?}, pos: {}", index1, new_pos);
    if index0 != index1 {
        println!("Prev: {:?}, index: {:?}", index0, index1);
        let vec = grid.map.get_mut(&get_key(index0)).unwrap();
        vec.remove(
            vec.iter().position(|x| *x == entity)
            .expect(format!("No such entity found. Prev: {}, Current: {}", prev, new_pos).as_str())
        );
        grid.map.get_mut(&get_key(index1)).unwrap().push(entity);
    };
}

const BOUNDS: [Vec3; 2] = [Vec3::new(-10., -10., -10.), Vec3::new(10., 10., 10.)];
const DIMENSIONS: [i32; 3] = [10, 10, 10];

#[derive(Resource)]
struct GridMap {
    map: HashMap<String, Vec<Entity>>
}

fn init_grid_map (
    mut commands: Commands
) {
    let mut map: HashMap<String, Vec<Entity>> = HashMap::new();
    for x in 0..DIMENSIONS[0] {
        for y in 0..DIMENSIONS[1] {
            for z in 0..DIMENSIONS[2] {
                let key = get_key((x, y, z));
                map.insert(key, Vec::new());
                println!("Index: {}:{}:{}", x, y, z);
            }
        }
    }
    println!("TEST: {:?}", get_cell_index(Vec3::new(100., 0., 0.)));
    commands.insert_resource(GridMap { map });
}

fn get_key (index: (i32, i32,i32)) -> String {
    return format!("{}{}{}", index.0, index.1, index.2);
}

fn get_cell_index (pos: Vec3) -> (i32, i32, i32) {

    // let x = (((pos.x + max).clamp(0., BOUNDS) / BOUNDS) * DIMENSIONS as f32).floor() as i32;
    // let y = (((pos.y + max).clamp(0., BOUNDS) / BOUNDS) * DIMENSIONS as f32).floor() as i32;
    // let z = (((pos.z + max).clamp(0., BOUNDS) / BOUNDS) * DIMENSIONS as f32).floor() as i32;


    let x = ((pos.x - BOUNDS[0].x) / (BOUNDS[1].x - BOUNDS[0].x)).clamp(0., 1.0);
    let y = ((pos.y - BOUNDS[0].y) / (BOUNDS[1].y - BOUNDS[0].y)).clamp(0., 1.0);
    let z = ((pos.z - BOUNDS[0].z) / (BOUNDS[1].z - BOUNDS[0].z)).clamp(0., 1.0);

    let x_i = (x * (DIMENSIONS[0] as f32 - 1.0)).floor() as i32;
    let y_i = (y * (DIMENSIONS[1] as f32 - 1.0)).floor() as i32;
    let z_i = (z * (DIMENSIONS[2] as f32 - 1.0)).floor() as i32;

    return (x_i, y_i, z_i);
}

// fn avoid_nearby (
//     mut q_boid_trans: Query<&mut Transform, With<Boid>>,
// ) {
//     for mut boid in q_boid_trans.iter_mut() {
//         let mut entities: [Entity; 10] = [Entity::from_raw(0); 10];
//     }
// }

fn steer_towards_average_local_velocity (
    mut query: Query<&mut Velocity>,
    mut grid: ResMut<GridMap>
) {

}

fn stay_inside_bounds (
    mut boid_query: Query<(&mut Transform, Entity), With<Boid>>,
    mut grid: ResMut<GridMap>
) {
    let mut new_pos = Vec3::ZERO;
    
    for (mut transform, entity) in boid_query.iter_mut() {
        let old_pos = transform.translation;

        if transform.translation.x > BOUNDS[1].x {
            new_pos.x = BOUNDS[0].x;
        }
        else if transform.translation.x < BOUNDS[0].x {
            new_pos.x = BOUNDS[1].x;
        }

        if transform.translation.y > BOUNDS[1].y {
            new_pos.y = BOUNDS[0].y;
        }
        else if transform.translation.y < BOUNDS[0].y {
            new_pos.y = BOUNDS[1].y;
        }

        if transform.translation.z > BOUNDS[1].z {
            new_pos.z = BOUNDS[0].z;
        }
        else if transform.translation.z < BOUNDS[0].z {
            new_pos.z = BOUNDS[1].z;
        }

        
        transform.translation = new_pos;
        update_grid(old_pos, new_pos, &mut grid, entity);
    }
}
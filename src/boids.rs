use bevy::{prelude::*, utils::HashMap, math::vec3};
use rand::Rng;
use crate::{GameState, loading::SceneAssets};

const SPEED: f32 = 10.0;
const STEERING_FACTOR: f32 = 1.0;
const BIRD_COUNT: u32 = 1000;
const BOID_DIST_TOLERANCE_SQRD: f32 = 4.0;

const BOUNDS: [Vec3; 2] = [Vec3::new(-100., -100., -100.), Vec3::new(100., 100., 100.)];
const DIMENSIONS: [i32; 3] = [20, 20, 20];

pub struct BoidsPlugin;

impl Plugin for BoidsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(init_grid_map)
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_boids))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(move_boids))
            // .add_system_set(SystemSet::on_update(GameState::Playing).with_system(stay_inside_bounds))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(steer_towards_average_local_velocity))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(update_velocity))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(steer_towards_center))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(steer_horizontal))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(avoid_nearby))
            .register_type::<TargetVelocity>()
            ;
    }
}

#[derive(Bundle)]
struct BoidBundle {
    boid: Boid,
    velocity: Velocity,
    target: TargetVelocity,
    scene_bundle: SceneBundle,
}

#[derive(Component)]
struct Boid;

#[derive(Component)]
struct Velocity(Vec3);

#[derive(Component, Debug, Reflect)]
pub(crate) struct TargetVelocity(Vec3);

fn spawn_boids (
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    mut grid_map: ResMut<GridMap>,
    scenes: Res<SceneAssets>,
) {
    let mut rng = rand::thread_rng();
    
    for _ in 0..BIRD_COUNT {
        let pos = Vec3::new(rng.gen_range(BOUNDS[0].x..BOUNDS[1].x) as f32, rng.gen_range(BOUNDS[0].y..BOUNDS[1].y) as f32, rng.gen_range(BOUNDS[0].z..BOUNDS[1].z) as f32);

        // println!("Spawn pos: {}", pos);
        // println!("Calc index: {:?}", get_cell_index(pos));

        let vel = Vec3::new(rng.gen_range(-10..10) as f32, rng.gen_range(-3..3) as f32, rng.gen_range(-10..10) as f32).normalize();

        let id = commands.spawn((BoidBundle {
            boid: Boid,
            velocity: Velocity(vel),
            target: TargetVelocity(vel),
            scene_bundle: SceneBundle {
                scene: scenes.bird.clone(),
                transform: Transform::from_translation(pos).with_scale(Vec3::splat(0.02)),
                ..default()
            }},
            Name::new("Boid"),
        )).id();

        let index = get_cell_index(pos);

        let entities = match grid_map.map.get_mut(&get_key(index)) {
            Some(v) => {
                println!("Pushed to index {:?}", index);
                v
            },
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
        let prev_pos = transform.translation;
        let focus = transform.translation - velocity.0;
        let up = Vec3::Y;
        transform.look_at(focus, up);
        
        let new_pos = transform.translation + velocity.0 * time.delta_seconds() * SPEED;

        transform.translation = new_pos;

        update_grid(prev_pos, new_pos, &mut grid, entity);
    }
}

fn update_grid(prev: Vec3, new_pos: Vec3, grid: &mut ResMut<GridMap>, entity: Entity) {
    let index0 = get_cell_index(prev);
    let index1 = get_cell_index(new_pos);
    // println!("index: {:?}, pos: {}", index1, new_pos);
    if index0 != index1 {
        // println!("Prev: {:?}, index: {:?}", index0, index1);
        let vec = grid.map.get_mut(&get_key(index0)).unwrap();
        vec.remove(
            vec.iter().position(|x| *x == entity)
            .expect(format!("No such entity found. Prev: {}, Current: {}", prev, new_pos).as_str())
        );
        grid.map.get_mut(&get_key(index1)).unwrap().push(entity);
    };
}

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
                // println!("Index: {}:{}:{}", x, y, z);
            }
        }
    }
    commands.insert_resource(GridMap { map });
}

fn get_nearby (pos: Vec3, grid: &Res<GridMap>) -> Vec<Entity> {
    let index = get_cell_index(pos);
    let key = get_key(index);
    let nearby = grid.map.get(&key).unwrap();
    nearby.to_owned()
}

fn get_key (index: (i32, i32,i32)) -> String {
    return format!("{}{}{}", index.0, index.1, index.2);
}

fn get_cell_index (pos: Vec3) -> (i32, i32, i32) {
    let x = ((pos.x - BOUNDS[0].x) / (BOUNDS[1].x - BOUNDS[0].x)).clamp(0., 0.9999999);
    let y = ((pos.y - BOUNDS[0].y) / (BOUNDS[1].y - BOUNDS[0].y)).clamp(0., 0.9999999);
    let z = ((pos.z - BOUNDS[0].z) / (BOUNDS[1].z - BOUNDS[0].z)).clamp(0., 0.9999999);

    // println!("x: {:?}", x);

    let x_f = (x * (DIMENSIONS[0] as f32)).floor();
    let y_f = (y * (DIMENSIONS[1] as f32)).floor();
    let z_f = (z * (DIMENSIONS[2] as f32)).floor();

    // println!("x_f: {:?}", x_f);

    let z_i = z_f as i32;
    let y_i = y_f as i32;
    let x_i = x_f as i32;

    // println!("x_i: {:?}", x_i);

    return (x_i, y_i, z_i);
}

fn avoid_nearby (
    mut q_target_v: Query<(&mut TargetVelocity, &Transform)>,
    q_boid_trans: Query<&Transform, With<Boid>>,
    grid: Res<GridMap>,
) {
    for (mut target, trans) in q_target_v.iter_mut() {
        let nearby = get_nearby(trans.translation, &grid);
        let mut avoidance_vec: Vec3 = Vec3::ZERO;

        for entity in nearby {
            let pos = q_boid_trans.get(entity).expect("Boid pos not found from entity. ").translation;
            let offset_vec = pos - trans.translation;
            let dist_sqrd = offset_vec.length_squared();
            if dist_sqrd < BOID_DIST_TOLERANCE_SQRD {
                avoidance_vec += -offset_vec.normalize_or_zero() * (BOID_DIST_TOLERANCE_SQRD.sqrt() - dist_sqrd.sqrt());
            }
        }

        if let Some(nomalized_avoidance_vec) = avoidance_vec.try_normalize() {
            target.0 = nomalized_avoidance_vec;
        }
    }
}

fn steer_towards_average_local_velocity (
    mut query: Query<(&Transform, &mut TargetVelocity)>,
    q_velocity: Query<&Velocity>,
    grid: Res<GridMap>,
) {
    for (trans, mut target) in query.iter_mut() {
        let nearby = get_nearby(trans.translation, &grid);

        let list_len = nearby.len();
        // Skip if no neighbours
        if list_len == 0 { continue; }
        
        let mut average_v = Vec3::ZERO;

        for near_e in nearby {
            let near_vel = q_velocity.get(near_e).unwrap().0;
            average_v += near_vel / list_len as f32;
        }

        target.0 = target.0.lerp(average_v, 0.5).normalize_or_zero();
    }
}

fn steer_towards_center (
    mut query: Query<(&Transform, &mut TargetVelocity)>,
) {
    for (trans, mut target) in query.iter_mut() {
        if trans.translation.x.abs() > BOUNDS[1].x || trans.translation.y.abs() > BOUNDS[1].y || trans.translation.z.abs() > BOUNDS[1].z {
            target.0 = target.0.lerp(-trans.translation.normalize(), 0.3);
        }
    }
}

fn steer_horizontal (
    mut query: Query<(&Transform, &mut TargetVelocity)>,
) {
    for (trans, mut target) in query.iter_mut() {
        target.0 = target.0.lerp(vec3(target.0.x, target.0.y.clamp(-0.1, 0.1), target.0.z), 1.);
    }
}

fn update_velocity (
    mut q_vel: Query<(&mut Velocity, &TargetVelocity), Changed<TargetVelocity>>,
    time: Res<Time>,
) {

    for (mut vel, target) in q_vel.iter_mut() {
        vel.0 = vel.0.lerp(target.0, STEERING_FACTOR * time.delta_seconds()).normalize();
    }
}

fn _stay_inside_bounds (
    mut boid_query: Query<(&mut Transform, Entity), With<Boid>>,
    mut grid: ResMut<GridMap>
) {
    
    for (mut transform, entity) in boid_query.iter_mut() {
        let mut new_pos = Vec3::ZERO;
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

        if new_pos != Vec3::ZERO {
            transform.translation = new_pos;
            update_grid(old_pos, new_pos, &mut grid, entity);
        }
    }
}
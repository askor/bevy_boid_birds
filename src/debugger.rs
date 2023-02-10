use bevy::{prelude::*, time};

use crate::{GameState, boids::TargetVelocity};
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
        // .add_system_set(SystemSet::on_update(GameState::Playing).with_system(print_target_vel))
        // .add_system_set(SystemSet::on_update(GameState::Playing).with_system(stop_after_timer))
        .insert_resource(StopTimer(Timer::from_seconds(0.2, TimerMode::Once)))
        ;
    }
}

fn print_target_vel (
    query: Query<&TargetVelocity>
) {
    for tv in query.iter() {
        info!("TARGET VEL: {:?}", tv);
    }
}

#[derive(Resource)]
struct StopTimer(Timer);

fn stop_after_timer (
    mut timer: ResMut<StopTimer>,
    time: Res<Time>,
    mut state: ResMut<State<GameState>>
) {
    if timer.0.tick(time.delta()).just_finished() {
        state.set(GameState::Pause).unwrap();
    }
}
mod actions;
mod audio;
mod loading;
mod menu;
mod player;
mod scene;
mod camera;
mod boids;
mod debugger;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;
use crate::scene::ScenePlugin;
use crate::camera::CameraPlugin;
use crate::boids::BoidsPlugin;
use crate::debugger::DebugPlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
    Pause,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Loading)
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(ScenePlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(BoidsPlugin)
            .add_plugin(DebugPlugin)
            
            // External
            .add_plugin(bevy_inspector_egui::quick::WorldInspectorPlugin)
            ;

        #[cfg(debug_assertions)]
        {
            // app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            //     .add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}

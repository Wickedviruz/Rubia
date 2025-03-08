mod messages;
mod player;
mod network;
mod world_renderer;

use bevy::prelude::*;
use player::CameraSettings;
use network::NetworkPlugin;

#[derive(Resource, Default)]
pub struct PendingChunks(pub Vec<(i32, i32, Vec<u8>)>);

fn main() {
    App::new()
        .insert_resource(PendingChunks::default())
        .insert_resource(CameraSettings::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(NetworkPlugin)
        .add_systems(Startup, (
            player::spawn_player,
            world_renderer::spawn_basic_scene,
        ))
        .add_systems(Update, (
            player::player_movement,
            player::camera_follow_and_control,
            world_renderer::render_pending_chunks,
        ))
        .run();
}

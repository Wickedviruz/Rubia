mod gui;
mod world;
mod camera;
mod io; 

use bevy::prelude::*;
use camera::camera_controls;
use gui::{gui_system, CurrentTool};
use world::{setup_scene, WorldMap};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(CurrentTool::default())
        .insert_resource(WorldMap::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_egui::EguiPlugin)
        .add_systems(Startup, setup_scene) // <--- se punkt 3 nedan!
        .add_systems(Update, (gui_system, camera_controls, world::world_rendering))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(10.0, 15.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

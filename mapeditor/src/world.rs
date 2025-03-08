use bevy::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Resource, Serialize, Deserialize, Default)]
pub struct WorldMap {
    pub blocks: Vec<((i32, i32, i32), BlockType)>,
}

#[derive(Clone, Copy, Serialize, Deserialize, Default, PartialEq)]
pub enum BlockType {
    #[default]
    Empty,
    Grass,
    Dirt,
    Stone,
}

pub fn setup_scene(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

pub fn world_rendering(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    world: Res<WorldMap>,
) {
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(10.0, 15.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    for (pos, block_type) in &world.blocks {
        commands.spawn(PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(StandardMaterial {
                base_color: match block_type {
                    BlockType::Grass => Color::rgb(0.2, 0.8, 0.2),
                    BlockType::Stone => Color::GRAY,
                    BlockType::Dirt => Color::rgb(0.6, 0.4, 0.2),
                    _ => Color::WHITE,
                },
                ..default()
            }),
            transform: Transform::from_xyz(pos.0 as f32, pos.1 as f32, pos.2 as f32),
            ..default()
        });
    }
}


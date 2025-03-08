use bevy::prelude::*;
use crate::PendingChunks;

pub fn spawn_basic_scene(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(16.0 - 10.0, 10.0, 16.0 - 10.0)
            .looking_at(Vec3::new(16.0, 1.0, 16.0), Vec3::Y),
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

pub fn render_pending_chunks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut pending_chunks: ResMut<PendingChunks>,
) {
    for (chunk_x, chunk_y, blocks) in pending_chunks.0.drain(..) {
        render_chunk(&mut commands, &mut meshes, &mut materials, chunk_x, chunk_y, &blocks);
    }
}

fn render_chunk(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    chunk_x: i32,
    chunk_y: i32,
    blocks: &[u8],
) {
    for x in 0..32 {
        for y in 0..32 {
            let height = blocks[y * 32 + x];
            if height > 0 {
                // Rendera block staplade ovanpå varandra baserat på höjden
                for h in 0..height {
                    commands.spawn(PbrBundle {
                        mesh: meshes.add(Mesh::from(Cuboid::new(1.0, 1.0, 1.0))),
                        material: materials.add(StandardMaterial {
                            base_color: if h == height - 1 { Color::GREEN } else { Color::rgb(0.6, 0.3, 0.1) },
                            ..default()
                        }),
                        transform: Transform::from_xyz(
                            (chunk_x * 32 + x as i32) as f32,
                            h as f32, // höjden här!
                            (chunk_y * 32 + y as i32) as f32,
                        ),
                        ..default()
                    });
                }
            }
        }
    }
}

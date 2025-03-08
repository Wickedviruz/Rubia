use bevy::prelude::*;
use bevy::prelude::Cuboid;
use bevy::input::mouse::{MouseMotion, MouseWheel};

#[derive(Component)]
pub struct Player;

#[derive(Resource)]
pub struct CameraSettings {
    pub distance: f32,
    pub yaw: f32,
    pub pitch: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            distance: 15.0,
            yaw: std::f32::consts::PI / 2.0,
            pitch: 0.5,
        }
    }
}

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Player,
        PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(0.8, 0.8, 0.8))),
            material: materials.add(StandardMaterial {
                base_color: Color::YELLOW,
                ..default()
            }),
            transform: Transform::from_xyz(16.0, 1.0, 16.0),
            ..default()
        },
    ));
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let speed = 5.0;
    for mut transform in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) { direction.z -= 1.0; }
        if keyboard_input.pressed(KeyCode::KeyS) { direction.z += 1.0; }
        if keyboard_input.pressed(KeyCode::KeyA) { direction.x -= 1.0; }
        if keyboard_input.pressed(KeyCode::KeyD) { direction.x += 1.0; }

        transform.translation += direction.normalize_or_zero() * speed * time.delta_seconds();
    }
}

pub fn camera_follow_and_control(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
    mut settings: ResMut<CameraSettings>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut scroll_events: EventReader<MouseWheel>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for event in scroll_events.read() {
            settings.distance = (settings.distance - event.y * 2.0).clamp(5.0, 50.0);
        }

        if mouse_input.pressed(MouseButton::Right) {
            for event in mouse_motion_events.read() {
                settings.yaw -= event.delta.x * 0.01;
                settings.pitch = (settings.pitch + event.delta.y * 0.01).clamp(-1.5, 1.5);
            }
        }

        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            let offset = Vec3::new(
                settings.yaw.cos() * settings.pitch.cos(),
                settings.pitch.sin(),
                settings.yaw.sin() * settings.pitch.cos(),
            ) * settings.distance;

            camera_transform.translation = player_transform.translation + offset;
            camera_transform.look_at(player_transform.translation, Vec3::Y);
        }
    }
}

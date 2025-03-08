use bevy::prelude::*;

pub fn camera_controls(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    let speed = 10.0;
    for mut transform in camera_query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowUp) { direction.z -= 1.0; }
        if keyboard_input.pressed(KeyCode::ArrowDown) { direction.z += 1.0; }
        if keyboard_input.pressed(KeyCode::ArrowLeft) { direction.x -= 1.0; }
        if keyboard_input.pressed(KeyCode::ArrowRight) { direction.x += 1.0; }

        transform.translation += direction.normalize_or_zero() * speed * time.delta_seconds();
    }
}

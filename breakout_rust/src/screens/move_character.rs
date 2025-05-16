use bevy::prelude::*;

const CHARACTER_SPEED: f32 = 500.0;
pub const CHARACTER_SIZE: Vec2 = Vec2::new(300.0, 256.0);
const CHARACTER_PADDING: f32 = 10.0;

#[derive(Component)]
enum Direction{
    Left,
    Right,
}

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
pub fn sprite_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut character_transform: Single<&mut Transform, With<crate::screens::character::Character>>,
    time: Res<Time>,
) {
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::KeyA) {
        direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyD) {
        direction += 1.0;
    }

    // Calculate the new horizontal paddle position based on player input
    let new_character_position =
        character_transform.translation.x + direction * CHARACTER_SPEED * time.delta_secs();

    // Update the paddle position,
    // making sure it doesn't cause the paddle to leave the arena
    let left_bound = crate::screens::game::LEFT_WALL + crate::screens::game::WALL_THICKNESS / 2.0 + CHARACTER_SIZE.x / 2.0 + CHARACTER_PADDING;
    let right_bound = crate::screens::game::RIGHT_WALL - crate::screens::game::WALL_THICKNESS / 2.0 - CHARACTER_SIZE.x / 2.0 - CHARACTER_PADDING;

    character_transform.translation.x = new_character_position.clamp(left_bound, right_bound);
}
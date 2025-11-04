use crate::{
    messages::{NetworkAction, PlayerFrameInput},
    players::{
        collision::check_player_collision,
        constants::{FLY_SPEED_MULTIPLIER, GRAVITY, JUMP_VELOCITY, MAX_VERTICAL_SPEED, SPEED},
    },
    world::WorldMap,
};
use bevy::prelude::*;

use super::Player;

pub fn simulate_player_movement(
    player: &mut Player,
    world_map: &impl WorldMap,
    action: &PlayerFrameInput,
) {
    // let's check if the 9 chunks around the player are loaded
    let chunks = world_map.get_surrounding_chunks(player.position, 1);
    if chunks.len() < 9 {
        log::debug!("Not enough chunks loaded, skipping movement simulation");
        return;
    }

    let delta = action.delta_ms as f32 / 1000.0;

    if action.is_pressed(NetworkAction::ToggleFlyMode) {
        player.is_flying = !player.is_flying;
    }

    player.camera_transform = action.camera;

    let direction = get_desired_direction(player, action);
    let is_jumping = action.is_pressed(NetworkAction::JumpOrFlyUp);

    if player.is_flying {
        fly(player, direction, delta);
    } else {
        fly_not(player, is_jumping, direction, delta, world_map);
    }

    // If the player is below the world, reset their position
    const FALL_LIMIT: f32 = -50.0;
    if player.position.y < FALL_LIMIT {
        player.position = Vec3::new(0.0, 100.0, 0.0);
        player.velocity.y = 0.0;
    }
}

fn get_desired_direction(player: &mut Player, action: &PlayerFrameInput) -> Vec3 {
    let mut direction = Vec3::ZERO;

    // Calculate movement directions relative to the camera, in the horizontal plane
    let forward = player
        .camera_transform
        .forward()
        .xyz()
        .with_y(0.0)
        .normalize();

    let right = player
        .camera_transform
        .right()
        .xyz()
        .with_y(0.0)
        .normalize();

    // Adjust direction based on key presses
    if action.is_pressed(NetworkAction::MoveBackward) {
        direction -= forward;
    }
    if action.is_pressed(NetworkAction::MoveForward) {
        direction += forward;
    }
    if action.is_pressed(NetworkAction::MoveLeft) {
        direction -= right;
    }
    if action.is_pressed(NetworkAction::MoveRight) {
        direction += right;
    }

    // Normalize direction to prevent faster movement with diagonals
    if direction != Vec3::ZERO {
        direction = direction.normalize();
    }

    if action.is_pressed(NetworkAction::JumpOrFlyUp) {
        direction += Vec3::Y;
    }
    if action.is_pressed(NetworkAction::SneakOrFlyDown) {
        direction -= Vec3::Y;
    }

    direction
}

fn fly(player: &mut Player, direction: Vec3, delta: f32) {
    player.position += direction * (SPEED * FLY_SPEED_MULTIPLIER * delta);
    player.velocity.y = 0.0;
    player.on_ground = false;
}

fn fly_not(
    player: &mut Player,
    is_jumping: bool,
    direction: Vec3,
    delta: f32,
    world_map: &impl WorldMap,
) {
    let displacement = SPEED * delta;

    // Attempt to move the player by the calculated direction
    let new_x = player.position.x + direction.x * displacement;
    let new_z = player.position.z + direction.z * displacement;

    player.velocity.y = player
        .velocity
        .y
        .max(-MAX_VERTICAL_SPEED)
        .min(MAX_VERTICAL_SPEED);
    let new_y = player.position.y + player.velocity.y * delta;

    let new_vec_x = &player.position.with_x(new_x);
    let new_vec_y = &player.position.with_y(new_y);
    let new_vec_z = &player.position.with_z(new_z);

    // If a block is detected in the new position, don't move the player on this axis
    if !check_player_collision(new_vec_x, player, world_map) {
        player.position.x = new_x;
    }

    if check_player_collision(new_vec_y, player, world_map) {
        player.on_ground = true;
        player.velocity.y = 0.0;
    } else {
        player.position.y = new_y;
        player.on_ground = false;
    }

    if !check_player_collision(new_vec_z, player, world_map) {
        player.position.z = new_z;
    }

    // TODO: short-hops
    // Handle jumping (if on the ground) and gravity, only if not flying
    if player.on_ground && is_jumping {
        // Player can jump only when grounded
        player.velocity.y = JUMP_VELOCITY;
        player.on_ground = false;
    } else if !player.on_ground {
        // Apply gravity when the player is in the air
        player.velocity.y += GRAVITY * delta;
    }
}

trait IsPressed {
    fn is_pressed(&self, action: NetworkAction) -> bool;
}

impl IsPressed for PlayerFrameInput {
    fn is_pressed(&self, action: NetworkAction) -> bool {
        self.inputs.contains(&action)
    }
}

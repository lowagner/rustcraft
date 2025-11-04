use crate::{
    constants::DEFAULT_CHUNK_RENDER_DISTANCE_RADIUS,
    input::{data::GameAction, keyboard::is_action_just_pressed},
    player::CurrentPlayerMarker,
    world::WorldRenderRequestUpdateEvent,
    KeyMap,
};
use bevy::prelude::*;
use shared::CHUNK_SIZE;

#[derive(Resource, Default, Reflect)]
pub struct RenderDistance {
    pub distance: u32,
}

pub fn render_distance_update_system(
    player_transform: Query<&Transform, With<CurrentPlayerMarker>>,
    mut ev_writer: EventWriter<WorldRenderRequestUpdateEvent>,
    mut render_distance: ResMut<RenderDistance>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    key_map: Res<KeyMap>,
) {
    if render_distance.distance == 0 {
        render_distance.distance = DEFAULT_CHUNK_RENDER_DISTANCE_RADIUS;
    }

    if is_action_just_pressed(GameAction::RenderDistanceMinus, &keyboard_input, &key_map) {
        render_distance.distance = 1.max(render_distance.distance - 1);
        info!("Reducing render distance to {}", render_distance.distance);
        // Far-away chunks will be despawned via `entities::stack::stack_update_system`.
    }

    if is_action_just_pressed(GameAction::RenderDistancePlus, &keyboard_input, &key_map) {
        let old_distance = render_distance.distance as f32;
        render_distance.distance += 1;
        let new_distance = render_distance.distance as f32;
        info!("Increasing render distance to {}", render_distance.distance);
        if let Ok(player_transform) = player_transform.single() {
            ask_for_chunks(
                ev_writer,
                player_transform.translation,
                old_distance,
                new_distance,
            );
        } else {
            debug!("Player position not found");
        }
    }
}

fn ask_for_chunks(
    mut ev_writer: EventWriter<WorldRenderRequestUpdateEvent>,
    player_position: Vec3,
    old_distance: f32,
    new_distance: f32,
) {
    info!("Adding chunks around player position {}", player_position);
    warn!("TODO");
    _ = ev_writer;
    _ = old_distance;
    _ = new_distance;
}

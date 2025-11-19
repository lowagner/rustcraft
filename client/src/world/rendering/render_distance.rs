use crate::{
    constants::DEFAULT_CHUNK_RENDER_DISTANCE_RADIUS,
    input::{data::GameAction, keyboard::is_action_just_pressed},
    player::CurrentPlayerMarker,
    world::WorldRenderRequestUpdateEvent,
    KeyMap,
};
use bevy::prelude::*;
use shared::world::{chunk_index_to_world_position, world_position_to_chunk_position};
use shared::CHUNK_SIZE;

// TODO: implement Default using DEFAULT_CHUNK_RENDER_DISTANCE_RADIUS
#[derive(Resource, Default, Reflect)]
pub struct RenderDistance {
    chunks: u32,
}

impl RenderDistance {
    pub fn close_enough(&self, v1: &Vec3, v2: &Vec3) -> bool {
        v1.distance(*v2) < self.distance()
    }

    pub fn too_far(&self, v1: &Vec3, v2: &Vec3) -> bool {
        !self.close_enough(v1, v2)
    }

    fn distance(&self) -> f32 {
        self.chunks as f32 * CHUNK_SIZE as f32
    }
}

pub fn render_distance_update_system(
    player_transform: Query<&Transform, With<CurrentPlayerMarker>>,
    mut ev_writer: EventWriter<WorldRenderRequestUpdateEvent>,
    mut render_distance: ResMut<RenderDistance>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    key_map: Res<KeyMap>,
) {
    if render_distance.chunks == 0 {
        render_distance.chunks = DEFAULT_CHUNK_RENDER_DISTANCE_RADIUS;
    }

    if is_action_just_pressed(GameAction::RenderDistanceMinus, &keyboard_input, &key_map) {
        render_distance.chunks = 1.max(render_distance.chunks - 1);
        info!("Reducing render distance to {}", render_distance.chunks);
        // Far-away chunks will be despawned via `entities::stack::stack_update_system`.
    }

    if is_action_just_pressed(GameAction::RenderDistancePlus, &keyboard_input, &key_map) {
        let old_distance = render_distance.distance();
        render_distance.chunks += 1;
        let new_distance = render_distance.distance();
        info!("Increasing render distance to {}", render_distance.chunks);
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
    info!(
        "Adding chunks around player position {} between {} and {}",
        player_position, old_distance, new_distance
    );
    let center_chunk_index = world_position_to_chunk_position(player_position);
    let max_delta: i32 = (new_distance / CHUNK_SIZE as f32).floor() as i32;
    for i in -max_delta..=max_delta {
        for j in -max_delta..=max_delta {
            for k in -max_delta..=max_delta {
                let chunk_index = center_chunk_index + IVec3 { x: i, y: j, z: k };
                let chunk_position = chunk_index_to_world_position(&chunk_index);
                let chunk_distance = player_position.distance(chunk_position);
                // Some distances to diagonal chunks will be > new_distance;
                // we could be smarter about the i,j,k loop but being lazy for now.
                if chunk_distance <= new_distance && chunk_distance > old_distance {
                    info!("Requesting reload of chunk at index {}", chunk_index);
                    // TODO: this doesn't actually work.
                    ev_writer.write(WorldRenderRequestUpdateEvent::ChunkToReload(chunk_index));
                }
            }
        }
    }
}

mod collision;
mod end_turn;
mod entity_render;
mod map_render;
mod movement;
mod player_input;
mod rand_move;

use crate::prelude::*;

pub fn build_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .build()
}

pub fn build_player_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(movement::movement_system())
        .flush()
        .add_system(collision::collision_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(end_turn::end_turn_system())
        .build()
}

pub fn build_enemy_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(rand_move::rand_movement_system())
        .flush()
        .add_system(movement::movement_system())
        .flush()
        .add_system(collision::collision_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(end_turn::end_turn_system())
        .build()
}
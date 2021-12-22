use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
pub fn movement(
    entity: &Entity,
    wants_move: &WantsToMove,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    sub_world: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if map.can_enter_tile(wants_move.destination) {
        commands.add_component(wants_move.entity, wants_move.destination);
        // Checking if the entity is the Player
        if sub_world
            .entry_ref(wants_move.entity)
            .unwrap()
            .get_component::<Player>()
            .is_ok()
        {
            camera.on_player_move(wants_move.destination);
        }
    }
    commands.remove(*entity);
}

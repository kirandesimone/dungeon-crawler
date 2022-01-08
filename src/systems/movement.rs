use crate::prelude::*;
use std::collections::HashSet;

#[system(for_each)]
#[read_component(Player)]
#[read_component(FieldOfView)]
#[read_component(Point)]
pub fn movement(
    entity: &Entity,
    wants_move: &WantsToMove,
    #[resource] map: &mut Map,
    #[resource] camera: &mut Camera,
    sub_world: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    let mut points = <(Entity, &Point)>::query();
    let entities_pos: HashSet<&Point> = points
        .iter(sub_world)
        .filter(|(entity, _)| **entity != wants_move.entity)
        .map(|(_, pos)| pos)
        .collect();

    if let Ok(entry) = sub_world.entry_ref(wants_move.entity) {
        if let Ok(fov) = entry.get_component::<FieldOfView>() {
            commands.add_component(wants_move.entity, fov.clone_dirty());

            if entry.get_component::<Player>().is_ok() {
                camera.on_player_move(wants_move.destination);
                fov.visible_tiles.iter().for_each(|pos| {
                    map.revealed_tiles[map_idx(pos.x, pos.y)] = true;
                })
            }
        }
    }

    if map.can_enter_tile(wants_move.destination) && !entities_pos.contains(&wants_move.destination)
    {
        commands.add_component(wants_move.entity, wants_move.destination);
    }
    commands.remove(*entity);
}

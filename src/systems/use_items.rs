use crate::prelude::*;

#[system]
#[read_component(ProvidesHealing)]
#[read_component(ProvidesDungeonMap)]
#[write_component(Health)]
#[read_component(ActivateItem)]
pub fn use_items(
    sub_world: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] map: &mut Map,
) {
    let mut healing_queue: Vec<(Entity, i32)> = Vec::new();

    <(Entity, &ActivateItem)>::query()
        .iter(sub_world)
        .for_each(|(entity, active_item)| {
            let item = sub_world.entry_ref(active_item.item);
            if let Ok(item) = item {
                if let Ok(healing) = item.get_component::<ProvidesHealing>() {
                    healing_queue.push((active_item.used_by, healing.amount));
                }

                if let Ok(_dungeon_map) = item.get_component::<ProvidesDungeonMap>() {
                    map.revealed_tiles.iter_mut().for_each(|t| *t = true);
                }
            }

            commands.remove(active_item.item);
            commands.remove(*entity);
        });

    for heals in healing_queue.iter() {
        if let Ok(mut target) = sub_world.entry_mut(heals.0) {
            if let Ok(health) = target.get_component_mut::<Health>() {
                health.current = i32::min(health.max, heals.1 + health.current);
            }
        }
    }
}

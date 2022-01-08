use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[write_component(Health)]
#[read_component(Item)]
#[read_component(Carried)]
#[read_component(Weapon)]
pub fn player_input(
    sub_world: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::W => Point::new(0, -1),
            VirtualKeyCode::D => Point::new(1, 0),
            VirtualKeyCode::S => Point::new(0, 1),
            VirtualKeyCode::A => Point::new(-1, 0),
            VirtualKeyCode::G => {
                let (player, player_pos) = players
                    .iter(sub_world)
                    .find_map(|(entity, pos)| Some((*entity, *pos)))
                    .unwrap();
                let mut items = <(Entity, &Point, &Item)>::query();
                items
                    .iter(sub_world)
                    .filter(|(_entity, &item_pos, _item)| item_pos == player_pos)
                    .for_each(|(entity, _item_pos, _item)| {
                        commands.remove_component::<Point>(*entity);
                        commands.add_component(*entity, Carried(player));

                        if let Ok(e) = sub_world.entry_ref(*entity) {
                            if e.get_component::<Weapon>().is_ok() {
                                <(Entity, &Carried, &Weapon)>::query()
                                    .iter(sub_world)
                                    .filter(|(_, carried, _)| carried.0 == player)
                                    .for_each(|(e, _c, _w)| commands.remove(*e));
                            }
                        }
                    });
                Point::new(0, 0)
            }
            VirtualKeyCode::Key1 => use_item(sub_world, 0, commands),
            VirtualKeyCode::Key2 => use_item(sub_world, 1, commands),
            VirtualKeyCode::Key3 => use_item(sub_world, 2, commands),
            VirtualKeyCode::Key4 => use_item(sub_world, 3, commands),
            VirtualKeyCode::Key5 => use_item(sub_world, 4, commands),
            VirtualKeyCode::Key6 => use_item(sub_world, 5, commands),
            VirtualKeyCode::Key7 => use_item(sub_world, 6, commands),
            VirtualKeyCode::Key8 => use_item(sub_world, 7, commands),
            VirtualKeyCode::Key9 => use_item(sub_world, 8, commands),
            _ => Point::zero(),
        };

        let (player_entity, destination) = players
            .iter(sub_world)
            .find_map(|(entity, pos)| Some((*entity, *pos + delta)))
            .unwrap();
        let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
        let mut did_something = false;
        if delta.x != 0 || delta.y != 0 {
            let mut hit_something = false;
            enemies
                .iter(sub_world)
                .filter(|(_, pos)| **pos == destination)
                .for_each(|(entity, _)| {
                    hit_something = true;
                    did_something = true;
                    commands.push((
                        (),
                        WantsToAttack {
                            attacker: player_entity,
                            victim: *entity,
                        },
                    ));
                });

            if !hit_something {
                commands.push((
                    (),
                    WantsToMove {
                        entity: player_entity,
                        destination,
                    },
                ));
            }
            *turn_state = TurnState::PlayerTurn;
        }
    }
}

fn use_item(sub_world: &mut SubWorld, n: usize, commands: &mut CommandBuffer) -> Point {
    let player_entity = <(Entity, &Player)>::query()
        .iter(sub_world)
        .find_map(|(entity, _player)| Some(*entity))
        .unwrap();

    let carried_items = <(Entity, &Carried, &Item)>::query()
        .iter(sub_world)
        .filter(|(_, carried, _)| carried.0 == player_entity)
        .enumerate()
        .filter(|(item_pos, (_, _, _))| *item_pos == n)
        .find_map(|(_, (entity, _, _))| Some(*entity));

    if let Some(item) = carried_items {
        commands.push((
            (),
            ActivateItem {
                used_by: player_entity,
                item,
            },
        ));
    }

    Point::zero()
}

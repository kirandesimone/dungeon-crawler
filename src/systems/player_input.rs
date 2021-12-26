use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[write_component(Health)]
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
            _ => Point::zero()
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
                did_something = true;
                commands.push((
                    (),
                    WantsToMove {
                        entity: player_entity,
                        destination,
                    },
                ));
            }

            if !did_something {
                if let Ok(mut player_health) = sub_world
                    .entry_mut(player_entity)
                    .unwrap()
                    .get_component_mut::<Health>()
                {
                    player_health.current = i32::min(player_health.max, player_health.current + 1);
                }
            }
            *turn_state = TurnState::PlayerTurn;
        }
    }
}

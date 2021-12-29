use crate::prelude::*;

#[system]
#[write_component(Health)]
#[read_component(WantsToAttack)]
#[read_component(Player)]
pub fn combat(sub_world: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();
    let victims: Vec<(Entity, Entity)> = attackers
        .iter(sub_world)
        .map(|(entity, attack)| (*entity, attack.victim))
        .collect();

    victims.iter().for_each(|(message, victim)| {
        let is_player = sub_world
            .entry_ref(*victim)
            .unwrap()
            .get_component::<Player>()
            .is_ok();
        if let Ok(mut health) = sub_world
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            println!("Health before attack: {}", health.current);
            health.current -= 1;
            if health.current < 1 && !is_player {
                commands.remove(*victim);
            }
            println!("Health after attack: {}", health.current);
        }
        commands.remove(*message);
    });
}

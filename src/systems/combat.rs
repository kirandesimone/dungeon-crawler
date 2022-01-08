use crate::prelude::*;

#[system]
#[write_component(Health)]
#[read_component(WantsToAttack)]
#[read_component(Player)]
#[read_component(Damage)]
#[read_component(Carried)]
pub fn combat(sub_world: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();
    let victims: Vec<(Entity, Entity, Entity)> = attackers
        .iter(sub_world)
        .map(|(entity, attack)| (*entity, attack.attacker, attack.victim))
        .collect();

    victims.iter().for_each(|(message, attacker, victim)| {
        let is_player = sub_world
            .entry_ref(*victim)
            .unwrap()
            .get_component::<Player>()
            .is_ok();
        let base_damage = if let Ok(v) = sub_world.entry_ref(*attacker) {
            if let Ok(damage) = v.get_component::<Damage>() {
                damage.0
            } else {
                0
            }
        } else {
            0
        };
        let weapon_dmg: i32 = <(&Carried, &Damage)>::query()
            .iter(sub_world)
            .filter(|(carried, _)| carried.0 == *attacker)
            .map(|(_, dmg)| dmg.0)
            .sum();

        let combined_dmg = base_damage + weapon_dmg;

        if let Ok(mut health) = sub_world
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            println!("Health before attack: {}", health.current);
            health.current -= combined_dmg;
            if health.current < 1 && !is_player {
                commands.remove(*victim);
            }
            println!("Health after attack: {}", health.current);
        }
        commands.remove(*message);
    });
}

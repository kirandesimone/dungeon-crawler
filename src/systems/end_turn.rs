use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Point)]
#[read_component(AmuletOfYala)]
pub fn end_turn(sub_world: &SubWorld, #[resource] turn_state: &mut TurnState) {
    let mut player_health = <(&Health, &Point)>::query().filter(component::<Player>());
    let mut amulet_yala_pos = <&Point>::query().filter(component::<AmuletOfYala>());
    let current_state = turn_state.clone();
    let mut new_state = match turn_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::EnemyTurn,
        TurnState::EnemyTurn => TurnState::AwaitingInput,
        _ => current_state,
    };

    let amulet_pos = amulet_yala_pos.iter(sub_world).nth(0).unwrap();

    player_health.iter(sub_world).for_each(|(hp, pos)| {
        if hp.current < 1 {
            new_state = TurnState::GameOver;
        }

        if pos == amulet_pos {
            new_state = TurnState::Victory;
        }
    });

    *turn_state = new_state;
}

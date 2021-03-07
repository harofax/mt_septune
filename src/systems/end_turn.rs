use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(CosmicEgg)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TurnState) {
    let mut player_hp = <(&Health, &Point)>::query().filter(component::<Player>());

    let mut cosmic_egg = <&Point>::query().filter(component::<CosmicEgg>());

    let current_state = turn_state.clone();

    let mut new_state = current_state;
    // let mut new_state = match current_state {
    //     TurnState::AwaitingInput => return,
    //     TurnState::PlayerTurn => TurnState::MonsterTurn,
    //     TurnState::MonsterTurn => TurnState::AwaitingInput,
    //     _ => current_state
    // };

    let cosmic_egg_pos = cosmic_egg.iter(ecs).nth(0).unwrap();

    player_hp.iter(ecs).for_each(|(hp, pos)| {
        if hp.current < 1 {
            new_state = TurnState::GameOver;
        }
        if pos == cosmic_egg_pos {
            new_state = TurnState::Victory;
        }
    });

    *turn_state = new_state;
}

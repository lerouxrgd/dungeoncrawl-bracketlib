use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    let delta = match key {
        Some(VirtualKeyCode::Left) => Point::new(-1, 0),
        Some(VirtualKeyCode::Right) => Point::new(1, 0),
        Some(VirtualKeyCode::Up) => Point::new(0, -1),
        Some(VirtualKeyCode::Down) => Point::new(0, 1),
        _ => return,
    };

    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
    let (player_entity, destination) = players
        .iter(ecs)
        .find_map(|(&entity, &pos)| Some((entity, pos + delta)))
        .unwrap();

    let mut hit_something = false;
    let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
    enemies
        .iter(ecs)
        .filter(|(_, &pos)| pos == destination)
        .for_each(|(&entity, _)| {
            hit_something = true;

            commands.push((WantsToAttack {
                attacker: player_entity,
                victim: entity,
            },));
        });

    if !hit_something {
        commands.push((WantsToMove {
            entity: player_entity,
            destination,
        },));
    }

    *turn_state = TurnState::PlayerTurn;
}

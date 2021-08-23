use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[write_component(Health)]
#[read_component(Item)]
#[read_component(Carried)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());

    let delta = match key {
        // movements
        Some(VirtualKeyCode::Left) => Point::new(-1, 0),
        Some(VirtualKeyCode::Right) => Point::new(1, 0),
        Some(VirtualKeyCode::Up) => Point::new(0, -1),
        Some(VirtualKeyCode::Down) => Point::new(0, 1),

        // pick up item
        Some(VirtualKeyCode::G) => {
            let (player, player_pos) = players
                .iter(ecs)
                .find_map(|(&entity, &pos)| Some((entity, pos)))
                .unwrap();

            let mut items = <(Entity, &Item, &Point)>::query();
            items
                .iter(ecs)
                .filter(|(_item, _, &item_pos)| item_pos == player_pos)
                .for_each(|(&item, _, _item_pos)| {
                    commands.remove_component::<Point>(item);
                    commands.add_component(item, Carried(player));
                });

            Point::zero()
        }

        // ignore other keys
        Some(_) => Point::zero(),
        // no key were pressed
        _ => return,
    };

    let (player, destination) = players
        .iter(ecs)
        .find_map(|(&player, &player_pos)| Some((player, player_pos + delta)))
        .unwrap();

    // No movement => heal
    if delta == Point::zero() {
        ecs.entry_mut(player)
            .unwrap()
            .get_component_mut::<Health>()
            .map(|mut health| {
                health.current = i32::min(health.max, health.current + 1);
            })
            .ok();
    } else
    // Move or attack
    {
        let mut hit_something = false;
        let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
        enemies
            .iter(ecs)
            .filter(|(_, &pos)| pos == destination)
            .for_each(|(&enemy, _)| {
                hit_something = true;

                commands.push((WantsToAttack {
                    attacker: player,
                    victim: enemy,
                },));
            });

        if !hit_something {
            commands.push((WantsToMove {
                entity: player,
                destination,
            },));
        }
    }

    *turn_state = TurnState::PlayerTurn;
}

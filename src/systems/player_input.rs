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

                    if let Ok(item_ref) = ecs.entry_ref(item) {
                        // picked up a weapon
                        if item_ref.get_component::<Weapon>().is_ok() {
                            <(Entity, &Carried, &Weapon)>::query()
                                .iter(ecs)
                                .filter(|(_, c, _)| c.0 == player)
                                .for_each(|(&previous_weapon, _, _)| {
                                    commands.remove(previous_weapon);
                                })
                        }
                    }
                });

            Point::zero()
        }

        // use item
        Some(VirtualKeyCode::Key1) => use_item(0, ecs, commands),
        Some(VirtualKeyCode::Key2) => use_item(1, ecs, commands),
        Some(VirtualKeyCode::Key3) => use_item(2, ecs, commands),
        Some(VirtualKeyCode::Key4) => use_item(3, ecs, commands),
        Some(VirtualKeyCode::Key5) => use_item(4, ecs, commands),
        Some(VirtualKeyCode::Key6) => use_item(5, ecs, commands),
        Some(VirtualKeyCode::Key7) => use_item(6, ecs, commands),
        Some(VirtualKeyCode::Key8) => use_item(7, ecs, commands),
        Some(VirtualKeyCode::Key9) => use_item(8, ecs, commands),

        // ignore other keys
        Some(_) => Point::zero(),
        // no key were pressed
        _ => return,
    };

    let (player, destination) = players
        .iter(ecs)
        .find_map(|(&player, &player_pos)| Some((player, player_pos + delta)))
        .unwrap();

    // Move or attack
    if delta != Point::zero() {
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

fn use_item(n: usize, ecs: &mut SubWorld, commands: &mut CommandBuffer) -> Point {
    let player = <(Entity, &Player)>::query()
        .iter(ecs)
        .find_map(|(&player, _)| Some(player))
        .unwrap();

    let item = <(Entity, &Item, &Carried)>::query()
        .iter(ecs)
        .filter(|(_, _, carried)| carried.0 == player)
        .enumerate()
        .filter(|&(item_idx, (_, _, _))| item_idx == n)
        .find_map(|(_, (&item, _, _))| Some(item));

    if let Some(item) = item {
        commands.push((ActivateItem {
            used_by: player,
            item,
        },));
    }

    Point::zero()
}

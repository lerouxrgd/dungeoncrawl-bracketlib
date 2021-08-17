use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(ChasingPlayer)]
#[read_component(Health)]
#[read_component(Player)]
pub fn chasing(#[resource] map: &Map, ecs: &SubWorld, commands: &mut CommandBuffer) {
    let mut movers = <(Entity, &Point, &ChasingPlayer)>::query();
    let mut positions = <(Entity, &Point, &Health)>::query();
    let mut player = <(&Point, &Player)>::query();

    let &player_pos = player.iter(ecs).nth(0).unwrap().0;
    let player_idx = map_idx(player_pos.x, player_pos.y);

    let search_targets = vec![player_idx];
    let dijkstra_map = DijkstraMap::new(SCREEN_WIDTH, SCREEN_HEIGHT, &search_targets, map, 1024.0);

    movers.iter(ecs).for_each(|(&mover, &source_pos, _)| {
        let idx = map_idx(source_pos.x, source_pos.y);
        let destination = match DijkstraMap::find_lowest_exit(&dijkstra_map, idx, map) {
            Some(destination) => destination,
            None => return,
        };

        // 1.2 is smaller than 1.4 which is approx. the diag dist => prevent diag attacks
        let distance = DistanceAlg::Pythagoras.distance2d(source_pos, player_pos);
        let destination = if distance > 1.2 {
            map.index_to_point2d(destination)
        } else {
            player_pos
        };

        let mut attacked = false;
        positions
            .iter(ecs)
            .filter(|(_, &target_pos, _)| target_pos == destination)
            .for_each(|(&victim, _, _)| {
                if ecs
                    .entry_ref(victim)
                    .unwrap()
                    .get_component::<Player>()
                    .is_ok()
                {
                    commands.push((WantsToAttack {
                        attacker: mover,
                        victim,
                    },));
                }
                attacked = true;
            });

        if !attacked {
            commands.push((WantsToMove {
                entity: mover,
                destination,
            },));
        }
    });
}

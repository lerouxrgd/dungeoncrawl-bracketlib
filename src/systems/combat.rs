use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[read_component(Player)]
#[write_component(Health)]
#[read_component(Damage)]
#[read_component(Carried)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();

    let victims: Vec<(Entity, Entity, Entity)> = attackers
        .iter(ecs)
        .map(|(&message, attack)| (message, attack.attacker, attack.victim))
        .collect();

    victims.into_iter().for_each(|(message, attacker, victim)| {
        let is_player = ecs
            .entry_ref(victim)
            .unwrap()
            .get_component::<Player>()
            .is_ok();

        let base_damage = ecs
            .entry_ref(attacker)
            .map(|attack| {
                attack
                    .get_component::<Damage>()
                    .map(|dmg| dmg.0)
                    .unwrap_or(0)
            })
            .unwrap_or(0);

        let weapon_damage = <(&Carried, &Damage)>::query()
            .iter(ecs)
            .filter(|(carried, _)| carried.0 == attacker)
            .map(|(_, dmg)| dmg.0)
            .sum::<i32>();

        let final_damage = base_damage + weapon_damage;

        if let Ok(mut health) = ecs.entry_mut(victim).unwrap().get_component_mut::<Health>() {
            health.current -= final_damage;
            if health.current < 1 && !is_player {
                commands.remove(victim);
            }
        }

        commands.remove(message);
    });
}

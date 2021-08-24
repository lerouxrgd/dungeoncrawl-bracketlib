use crate::prelude::*;

#[system]
#[read_component(ActivateItem)]
#[read_component(ProvidesHealing)]
#[write_component(Health)]
#[read_component(ProvidesDungeonMap)]
pub fn use_items(ecs: &mut SubWorld, commands: &mut CommandBuffer, #[resource] map: &mut Map) {
    let mut activations = <(Entity, &ActivateItem)>::query();

    let mut healing_to_apply = Vec::<(Entity, i32)>::new();
    activations
        .iter(ecs)
        .for_each(|(&msg, &ActivateItem { item, used_by })| {
            if let Ok(item) = ecs.entry_ref(item) {
                if let Ok(healing) = item.get_component::<ProvidesHealing>() {
                    healing_to_apply.push((used_by, healing.amount));
                }

                if let Ok(_mapper) = item.get_component::<ProvidesDungeonMap>() {
                    map.revealed_tiles.iter_mut().for_each(|t| *t = true);
                }
            }

            commands.remove(item);
            commands.remove(msg);
        });

    for (entity, heal) in healing_to_apply.into_iter() {
        if let Ok(mut target) = ecs.entry_mut(entity) {
            if let Ok(health) = target.get_component_mut::<Health>() {
                health.current = i32::min(health.max, health.current + heal);
            }
        }
    }
}

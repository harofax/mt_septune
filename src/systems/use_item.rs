use crate::prelude::*;

#[system]
#[read_component(ActivateItem)]
#[read_component(ProvidesHealing)]
#[write_component(Health)]
#[read_component(ProvidesDungeonMap)]
pub fn use_items(ecs: &mut SubWorld, commands: &mut CommandBuffer, #[resource] map: &mut Map) {
    // add our mutable changes to a list to be executed all at once
    let mut healing_to_apply = Vec::<(Entity, i32)>::new();

    <(Entity, &ActivateItem)>::query()
        .iter(ecs)
        .for_each(|(entity, activate)| {
            // entry_ref allows accessing an entity that's not returned in a query,
            // returns a reference to a single entity
            let item = ecs.entry_ref(activate.item);
            if let Ok(item) = item {
                // if let Ok => lets us only run code if item/etc exists
                //  get_component allows access to components assigned to the entity, but might be None
                if let Ok(healing) = item.get_component::<ProvidesHealing>() {
                    // add healing to our list of healing events to process
                    healing_to_apply.push((activate.used_by, healing.amount));
                }

                if let Ok(_mapper) = item.get_component::<ProvidesDungeonMap>() {
                    // go through map tiles and make them all revealed
                    map.revealed_tiles.iter_mut().for_each(|t| *t = true);
                }
            }

            commands.remove(activate.item);
            commands.remove(*entity);
        });

    for heal in healing_to_apply.iter() {
        if let Ok(mut target) = ecs.entry_mut(heal.0) {
            if let Ok(health) = target.get_component_mut::<Health>() {
                // to prevent overhealing, clamp to max
                health.current = i32::min(health.max, health.current + heal.1);
            }
        }
    }
}

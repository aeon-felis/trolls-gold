use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[allow(unused)]
pub fn collision_started_events_both_ways<'a>(
    reader: &'a mut MessageReader<CollisionEvent>,
) -> impl 'a + Iterator<Item = (Entity, Entity)> {
    reader
        .read()
        .filter_map(|e| {
            if let CollisionEvent::Started(e1, e2, _) = e {
                Some((*e1, *e2))
            } else {
                None
            }
        })
        .flat_map(|(e1, e2)| [(e1, e2), (e2, e1)])
}

use crate::prelude::*;

pub fn spawn_player(entity_component_system: &mut World, position: Point) {
    entity_component_system.push((
        Player,
        position,
        Render { color: ColorPair::new(WHITE, BLACK), glyph: to_cp437('@') }
    ));
}

use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(entity_component_system: &mut SubWorld, #[resource] map: &Map, #[resource] key: &Option<VirtualKeyCode>, #[resource] camera: &mut Camera) {
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::new(0, 0),
        };

        if delta.x != 0 || delta.y != 0 {
            <&mut Point>::query()
                .filter(component::<Player>())
                .iter_mut(entity_component_system)
                .for_each(|position| {
                    let destination = *position + delta;
                    if map.can_enter_tile(destination) {
                        *position = destination;
                        camera.on_player_move(destination);
                    }
                })
        }
    }
}

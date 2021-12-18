use crate::prelude::*;

const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_start: Point,
}

impl MapBuilder {
    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|map_tile| *map_tile = tile)
    }

    fn build_random_rooms(&mut self, random_number_generator: &mut RandomNumberGenerator) {
        while self.rooms.len() < NUM_ROOMS {
            let new_room = Rect::with_size(
                random_number_generator.range(1, SCREEN_WIDTH - 10),
                random_number_generator.range(1, SCREEN_HEIGHT - 10),
                random_number_generator.range(2, 10),
                random_number_generator.range(2, 10),
            );
            let mut overlap = false;
            for room in self.rooms.iter() {
                if room.intersect(&new_room) {
                    overlap = true
                }
            }

            if !overlap {
                new_room.for_each(|point| {
                    if point.x > 0 && point.x < SCREEN_WIDTH && point.y > 0 && point.y < SCREEN_HEIGHT {
                        let index = map_index(point.x, point.y);
                        self.map.tiles[index] = TileType::Floor;
                    }
                });
                self.rooms.push(new_room)
            }
        }
    }
}

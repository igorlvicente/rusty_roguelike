use crate::prelude::*;

const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_start: Point,
}

impl MapBuilder {
    pub fn new(random_number_generator: &mut RandomNumberGenerator) -> Self {
        let mut map_builder = Self {
            map: Map::new(),
            rooms: Vec::new(),
            player_start: Point::zero(),
        };

        map_builder.fill(TileType::Wall);
        map_builder.build_random_rooms(random_number_generator);
        map_builder.build_corridors(random_number_generator);
        map_builder.player_start = map_builder.rooms[0].center();
        map_builder
    }

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

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{min, max};

        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(index) = self.map.try_index(Point::new(x, y)) {
                self.map.tiles[index as usize] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{min, max};

        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(index) = self.map.try_index(Point::new(x, y)) {
                self.map.tiles[index as usize] = TileType::Floor;
            }
        }
    }

    fn build_corridors(&mut self, random_number_generator: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|room_a, room_b| room_a.center().x.cmp(&room_b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let previous_room = rooms[i-1].center();
            let current_room = room.center();

            if random_number_generator.range(0, 2) == 1 {
                self.apply_horizontal_tunnel(previous_room.x, current_room.x, previous_room.y);
                self.apply_vertical_tunnel(previous_room.y, current_room.y, current_room.x);
            } else {
                self.apply_horizontal_tunnel(previous_room.x, current_room.x, current_room.y);
                self.apply_vertical_tunnel(previous_room.y, current_room.y, previous_room.x);
            }
        }
    }
}

use bracket_lib::prelude::*;

mod map;
mod player;
mod map_builder;

mod prelude {
    pub use bracket_lib::prelude::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;

    pub use crate::map::*;
    pub use crate::player::*;
    pub use crate::map_builder::*;
}

use prelude::*;

struct State {
    map: Map,
    player: Player,
}

impl State {
    fn new() -> Self {
        let mut random_number_generator = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut random_number_generator);
        Self {
            map: map_builder.map,
            player: Player::new(map_builder.player_start),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, context: &mut BTerm) {
        context.cls();
        self.player.update(context, &self.map);
        self.map.render(context);
        self.player.render(context);
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple(SCREEN_WIDTH, SCREEN_HEIGHT)
        .expect("Error when trying to create the screen")
        .with_title("Rusty Roguelike")
        .with_fps_cap(30.0)
        .build()?;

    main_loop(context, State::new())
}

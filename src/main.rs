use bracket_lib::prelude::*;

mod map;
mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
}

use prelude::*;

struct State {}

impl GameState for State {
    fn tick(&mut self, context: &mut BTerm) {
        context.print_centered((SCREEN_HEIGHT as f32 / 2.0) as i32, "Hello, world!")
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple(SCREEN_WIDTH, SCREEN_HEIGHT)
        .expect("Error when trying to create the screen")
        .with_title("Rusty Roguelike")
        .build()?;

    main_loop(context, State {})
}

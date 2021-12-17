use bracket_lib::prelude::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

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

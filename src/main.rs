mod map;
mod player;
mod map_builder;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 90;
    pub const SCREEN_HEIGHT: i32 = 60;
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
        let mut rng = RandomNumberGenerator::new();
        let map_builder =  MapBuilder::new(&mut rng);
        Self {
            map: map_builder.map,
            player: Player::new(map_builder.player_start),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.player.update(ctx, &self.map);
        self.map.render(ctx);
        self.player.render(ctx);
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_font("haro_16x16.png", 16, 16)
        .with_tile_dimensions(16, 16)
        .with_automatic_console_resize(false)
        .with_simple_console(SCREEN_WIDTH, SCREEN_HEIGHT, "haro_16x16.png")
        .with_title("Mt. Septune")
        .with_fps_cap(30.0)
        .build()?;

    main_loop(context, State::new())
}

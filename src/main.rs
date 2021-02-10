mod map;
//mod player;
mod map_builder;
mod camera;
mod components;
mod spawner;
mod systems;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 45;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;

    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;

    pub use legion::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;
}

use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    systems: Schedule,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder =  MapBuilder::new(&mut rng);

        spawn_player(&mut ecs, map_builder.player_start);

        map_builder.rooms
            .iter()
            .skip(1)
            .map(|room| room.center())
            .for_each(|pos| spawn_monster(&mut ecs, &mut rng, pos));

        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));

        Self {
            ecs,
            resources,
            systems: build_scheduler()
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // Set layer to map console
        ctx.set_active_console(0);
        ctx.cls();

        // Set layer to entity console
        ctx.set_active_console(1);
        ctx.cls();

        // -- Execute systems
        self.resources.insert(ctx.key);
        self.systems.execute(&mut self.ecs, &mut self.resources);

        // -- Render Draw Buffer
        render_draw_buffer(ctx).expect("Render Draw Buffer ERROR");

    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Mt. Septune")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("haro_32x32.png", 32, 32)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "haro_32x32.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "haro_32x32.png") // no bg => transparency, new layer basically
        .with_automatic_console_resize(false)
        .build()?;

    main_loop(context, State::new())
}

#![warn(clippy::pedantic)]

mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod systems;
mod turn_state;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 100;
    pub const SCREEN_HEIGHT: i32 = 60;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub const TURN_TIME: f32 = 200.0; // frame duration in millisecs

    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;

    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
}

use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    frame_time: f32,
    monster_systems: Schedule,
    realtime_systems: Schedule,
    render_systems: Schedule,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);

        spawn_player(&mut ecs, map_builder.player_start);
        spawn_cosmic_egg(&mut ecs, map_builder.egg_start);

        map_builder
            .monster_spawns
            .iter()
            .for_each(|pos| spawn_entity(&mut ecs, &mut rng, *pos));

        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        // resources.insert(TurnState::AwaitingInput);
        resources.insert(TurnState::GamePlay);
        resources.insert(map_builder.theme);

        Self {
            ecs,
            resources,
            frame_time: 0.0,
            monster_systems: build_monster_scheduler(),
            realtime_systems: build_realtime_scheduler(),
            render_systems: build_render_scheduler(),
        }
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(2, RED, BLACK, "DEATH APPROACHES");
        ctx.print_color_centered(3, VIOLETRED2, BLACK, "----------------");
        ctx.print_color_centered(
            5,
            WHITE,
            BLACK,
            "The infinite void of the cosmos starts closing in.",
        );
        ctx.print_color(SCREEN_WIDTH / 2 - 12, 6, WHITE, BLACK, "------------------");
        ctx.print_color_centered(
            9,
            GOLD,
            BLACK,
            "The mystery of SEPTUNE will remain undiscovered.",
        );
        ctx.print_color(
            SCREEN_WIDTH / 2 - 20,
            10,
            MAGENTA,
            BLACK,
            "------------------",
        );

        ctx.print_color_centered(SCREEN_HEIGHT - 4, GREEN, BLACK, "Press ESC to try again.");

        if let Some(VirtualKeyCode::Escape) = ctx.key {
            self.reset_game_state();
        }
    }

    fn victory(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(2, GREEN, BLACK, "THE EGG HAS BEEN RETRIEVED");
        ctx.print_color_centered(3, GOLD, BLACK, "--------------------------");
        ctx.print_color_centered(
            5,
            WHITE,
            BLACK,
            "The surface of the Egg is impossibly detailed,",
        );
        ctx.print_color_centered(
            6,
            WHITE,
            BLACK,
            "and you feel the vibrations of creation itself coursing",
        );
        ctx.print_color_centered(7, WHITE, BLACK, "through every fiber of your being.");

        ctx.print_color_centered(SCREEN_HEIGHT - 5, GREEN, BLACK, "Press ESC to start again.");

        if let Some(VirtualKeyCode::Escape) = ctx.key {
            self.reset_game_state();
        }
    }

    fn reset_game_state(&mut self) {
        self.ecs = World::default();
        self.resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);

        spawn_player(&mut self.ecs, map_builder.player_start);
        spawn_cosmic_egg(&mut self.ecs, map_builder.egg_start);

        map_builder
            .monster_spawns
            .iter()
            .for_each(|pos| spawn_entity(&mut self.ecs, &mut rng, *pos));

        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(map_builder.player_start));
        self.resources.insert(TurnState::GamePlay);
        self.resources.insert(map_builder.theme);
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

        ctx.set_active_console(2);
        ctx.cls();

        self.frame_time += ctx.frame_time_ms;
        // -- Execute systems
        self.resources.insert(ctx.key);

        ctx.set_active_console(0);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));

        let current_state = self.resources.get::<TurnState>().unwrap().clone();

        match current_state {
            TurnState::GamePlay => {
                self.realtime_systems
                    .execute(&mut self.ecs, &mut self.resources);
                self.render_systems
                    .execute(&mut self.ecs, &mut self.resources);
                if self.frame_time > TURN_TIME {
                    self.frame_time = 0.0;
                    self.monster_systems
                        .execute(&mut self.ecs, &mut self.resources);
                }
            }
            TurnState::GameOver => {
                self.game_over(ctx);
            }
            TurnState::Victory => {
                self.victory(ctx);
            }
            _ => {}
        }

        // self.realtime_systems
        //     .execute(&mut self.ecs, &mut self.resources);
        /*
        match current_state {
            TurnState::AwaitingInput => self
                .input_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::PlayerTurn => {
                self.player_systems
                    .execute(&mut self.ecs, &mut self.resources);
            }
            TurnState::MonsterTurn => self
                .monster_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::GameOver => {
                self.game_over(ctx);
            }
            TurnState::Victory => {
                self.victory(ctx);
            }
        }*/

        // -- Render Draw Buffer
        render_draw_buffer(ctx).expect("Render Draw Buffer ERROR");
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Mt. Septune")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(16, 16)
        .with_resource_path("resources/")
        .with_font("haro_16x16.png", 16, 16)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "haro_16x16.png") // map
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "haro_16x16.png") // creatures
        .with_simple_console_no_bg(SCREEN_WIDTH, SCREEN_HEIGHT, "haro_16x16.png") // hud
        .build()?;

    main_loop(context, State::new())
}

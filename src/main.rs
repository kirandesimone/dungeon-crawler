mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod systems;
mod turn_state;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::systems::*;
    pub use legion::world::*;
    pub use legion::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
}

use prelude::*;
pub struct State {
    world: World,
    resources: Resources,
    input_schedule: Schedule,
    player_schedule: Schedule,
    enemy_schedule: Schedule,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_active_console(2);
        ctx.cls();
        self.resources.insert(ctx.key);
        ctx.set_active_console(0);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));
        let current_state = self.resources.get::<TurnState>().unwrap().clone();
        match current_state {
            TurnState::AwaitingInput => self
                .input_schedule
                .execute(&mut self.world, &mut self.resources),
            TurnState::PlayerTurn => self
                .player_schedule
                .execute(&mut self.world, &mut self.resources),
            TurnState::EnemyTurn => self
                .enemy_schedule
                .execute(&mut self.world, &mut self.resources),
            TurnState::GameOver => self.game_over(ctx),
            TurnState::Victory => self.victory(ctx),
        };
        render_draw_buffer(ctx).expect("Render Error");
    }
}

impl State {
    fn new() -> Self {
        let mut world = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let mb = MapBuilder::new(&mut rng);
        spawn_player(&mut world, mb.player_start);
        spawn_amulet_yala(&mut world, mb.amulet_start);
        mb.rooms.iter().skip(1).map(|r| r.center()).for_each(|pos| {
            spawn_enemy(&mut world, pos, &mut rng);
        });
        resources.insert(mb.map);
        resources.insert(Camera::new(mb.player_start));
        resources.insert(TurnState::AwaitingInput);
        Self {
            world,
            resources,
            input_schedule: build_input_scheduler(),
            player_schedule: build_player_scheduler(),
            enemy_schedule: build_enemy_scheduler(),
        }
    }

    fn reset(&mut self) {
        self.world = World::default();
        self.resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        spawn_player(&mut self.world, map_builder.player_start);
        spawn_amulet_yala(&mut self.world, map_builder.amulet_start);
        map_builder
            .rooms
            .iter()
            .skip(1)
            .map(|room| room.center())
            .for_each(|room_center| {
                spawn_enemy(&mut self.world, room_center, &mut rng);
            });
        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(map_builder.player_start));
        self.resources.insert(TurnState::AwaitingInput);
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(2, RED, BLACK, "GAME OVER");
        ctx.print_color_centered(4, WHITE, BLACK, "Slain by a monster");
        ctx.print_color_centered(5, WHITE, BLACK, "The amulet was not recovered");
        ctx.print_color_centered(9, GREEN, BLACK, "Press 1 to Play Again");
        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.reset();
        }
    }

    fn victory(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(2, GREEN, BLACK, "VICTOY");
        ctx.print_color_centered(4, WHITE, BLACK, "You have found the Amulet of Yala");
        ctx.print_color_centered(9, GREEN, BLACK, "Press 1 to Play Again");
        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.reset();
        }
    }
}

fn main() -> BError {
    let ctx = BTermBuilder::new()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("src/resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(SCREEN_WIDTH * 2, SCREEN_HEIGHT * 2, "terminal8x8.png")
        .build()?;

    let gamestate = State::new();
    main_loop(ctx, gamestate)
}

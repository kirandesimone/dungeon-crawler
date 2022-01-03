// FOR TESTING PURPOSES

use super::MapArchitect;
use crate::prelude::*;

pub struct EmptyArchitect {}

impl MapArchitect for EmptyArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
            monster_spawn: Vec::new(),
            theme: super::themes::DungeonTheme::new(),
        };
        mb.fill(TileType::Floor);
        mb.player_start = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        mb.amulet_start = mb.find_most_distant();
        for _ in 0..50 {
            mb.monster_spawn.push(Point::new(
                rng.range(1, SCREEN_WIDTH),
                rng.range(1, SCREEN_WIDTH),
            ));
        }
        mb
    }
}
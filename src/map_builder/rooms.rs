use super::MapArchitect;
use crate::prelude::*;

pub struct RoomArchitect {}

impl MapArchitect for RoomArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut map_builder = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
            monster_spawn: Vec::new(),
            theme: super::themes::DungeonTheme::new(),
        };

        map_builder.fill(TileType::Wall);
        map_builder.random_room_builder(rng);
        map_builder.build_hallways(rng);
        map_builder.player_start = map_builder.rooms[0].center();
        map_builder.amulet_start = map_builder.find_most_distant();
        for room in map_builder.rooms.iter().skip(1) {
            map_builder.monster_spawn.push(room.center());
        }

        map_builder
    }
}

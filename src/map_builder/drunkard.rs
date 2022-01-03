use super::MapArchitect;
use crate::prelude::*;

const STUMBLE_DIST: usize = 400;
const SCREEN_SIZE: usize = (SCREEN_HEIGHT * SCREEN_WIDTH) as usize;
const DESIRED_TILES: usize = SCREEN_SIZE / 3;
pub struct DrunkardsWalkArchitect {}

impl MapArchitect for DrunkardsWalkArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut map_builder = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            monster_spawn: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
            theme: super::themes::DungeonTheme::new(),
        };

        map_builder.fill(TileType::Wall);
        let center = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        self.drunkard(rng, &mut map_builder.map, &center);
        while map_builder
            .map
            .tiles
            .iter()
            .filter(|tile| **tile == TileType::Floor)
            .count()
            < DESIRED_TILES
        {
            let random_pos_x = rng.range(0, SCREEN_WIDTH);
            let random_pos_y = rng.range(0, SCREEN_HEIGHT);
            self.drunkard(
                rng,
                &mut map_builder.map,
                &Point::new(random_pos_x, random_pos_y),
            );

            let dijkstra_map = DijkstraMap::new(
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
                &vec![map_builder.map.point2d_to_index(center)],
                &map_builder.map,
                1024.0,
            );

            dijkstra_map
                .map
                .iter()
                .enumerate()
                .filter(|(_, distance)| *distance > &2000.0)
                .for_each(|(idx, _)| {
                    map_builder.map.tiles[idx] = TileType::Wall;
                });
        }

        map_builder.monster_spawn = map_builder.monster_spawn_points(&center, rng);
        map_builder.player_start = center;
        map_builder.amulet_start = map_builder.find_most_distant();

        map_builder
    }
}

impl DrunkardsWalkArchitect {
    fn drunkard(&mut self, rng: &mut RandomNumberGenerator, map: &mut Map, start: &Point) {
        let mut drunkard_pos = start.clone();
        let mut drunkard_dist = 0;

        loop {
            let drunkard_idx = map.point2d_to_index(drunkard_pos);
            map.tiles[drunkard_idx] = TileType::Floor;
            match rng.range(0, 4) {
                0 => drunkard_pos.x -= 1,
                1 => drunkard_pos.x += 1,
                2 => drunkard_pos.y -= 1,
                _ => drunkard_pos.y += 1,
            }
            if !map.in_bounds(drunkard_pos) {
                break;
            }
            drunkard_dist += 1;
            if drunkard_dist > STUMBLE_DIST {
                break;
            }
        }
    }
}

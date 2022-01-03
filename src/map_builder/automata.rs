use super::MapArchitect;
use crate::prelude::*;

pub struct CellularAutomataArchitect {}

impl MapArchitect for CellularAutomataArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut map_builder = MapBuilder {
            map: Map::new(),
            monster_spawn: Vec::new(),
            rooms: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
            theme: super::themes::DungeonTheme::new(),
        };
        self.random_noise_map(rng, &mut map_builder.map);
        for _ in 0..10 {
            self.iteration(&mut map_builder.map);
        }
        let start = self.find_player_start(&map_builder.map);
        map_builder.player_start = start;
        map_builder.amulet_start = map_builder.find_most_distant();
        map_builder.monster_spawn = map_builder.monster_spawn_points(&start, rng);

        map_builder
    }
}

impl CellularAutomataArchitect {
    fn random_noise_map(&mut self, rng: &mut RandomNumberGenerator, map: &mut Map) {
        map.tiles.iter_mut().for_each(|tile| {
            let roll = rng.range(0, 99);
            if roll > 55 {
                *tile = TileType::Floor;
            } else {
                *tile = TileType::Wall;
            }
        });
    }

    fn count_neighbors(&mut self, x: i32, y: i32, map: &Map) -> usize {
        let mut neighbors = 0;
        for iy in -1..=1 {
            for ix in -1..=1 {
                if !(iy == 0 && ix == 0) && map.tiles[map_idx(x + ix, y + iy)] == TileType::Wall {
                    neighbors += 1;
                }
            }
        }
        neighbors
    }

    fn iteration(&mut self, map: &mut Map) {
        let mut tiles = map.tiles.clone();
        for y in 1..SCREEN_HEIGHT - 1 {
            for x in 1..SCREEN_WIDTH - 1 {
                let neighbors = self.count_neighbors(x, y, map);
                let idx = map_idx(x, y);
                if neighbors > 4 || neighbors == 0 {
                    tiles[idx] = TileType::Wall;
                } else {
                    tiles[idx] = TileType::Floor;
                }
            }
        }
        map.tiles = tiles;
    }

    fn find_player_start(&mut self, map: &Map) -> Point {
        let center = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        let closest_point = map
            .tiles
            .iter()
            .enumerate()
            .filter(|(_, tile)| **tile == TileType::Floor)
            .map(|(idx, _)| {
                (
                    idx,
                    DistanceAlg::Pythagoras.distance2d(center, map.index_to_point2d(idx)),
                )
            })
            .min_by(|(_, distance_a), (_, distance_b)| distance_a.partial_cmp(distance_b).unwrap())
            .map(|(idx, _)| idx)
            .unwrap();

        map.index_to_point2d(closest_point)
    }
}

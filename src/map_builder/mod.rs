mod automata;
mod drunkard;
mod empty;
mod prefab;
mod rooms;
mod themes;

// use empty::EmptyArchitect;
use crate::prelude::*;
use automata::CellularAutomataArchitect;
use drunkard::DrunkardsWalkArchitect;
use prefab::place_prefab;
use rooms::RoomArchitect;
use themes::*;
trait MapArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder;
}

pub trait MapTheme: Sync + Send {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType;
}

const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_start: Point,
    pub amulet_start: Point,
    pub monster_spawn: Vec<Point>,
    pub theme: Box<dyn MapTheme>,
}

impl MapBuilder {
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut architect: Box<dyn MapArchitect> = match rng.range(0, 3) {
            0 => Box::new(RoomArchitect {}),
            1 => Box::new(CellularAutomataArchitect {}),
            _ => Box::new(DrunkardsWalkArchitect {}),
        };
        let mut map_builder = architect.new(rng);
        place_prefab(&mut map_builder, rng);
        map_builder.theme = match rng.range(0, 2) {
            0 => DungeonTheme::new(),
            _ => ForestTheme::new(),
        };

        map_builder
    }

    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn find_most_distant(&mut self) -> Point {
        let dijkstra_map = DijkstraMap::new(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            &vec![self.map.point2d_to_index(self.player_start)],
            &self.map,
            1024.0,
        );

        const UNREACHABLE: &f32 = &f32::MAX;
        self.map.index_to_point2d(
            dijkstra_map
                .map
                .iter()
                .enumerate()
                .filter(|(_, dist)| *dist < UNREACHABLE)
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .unwrap()
                .0,
        )
    }

    //carve out random dungeon rooms
    fn random_room_builder(&mut self, rng: &mut RandomNumberGenerator) {
        while self.rooms.len() < NUM_ROOMS {
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10),
                rng.range(1, SCREEN_HEIGHT - 10),
                rng.range(2, 10),
                rng.range(2, 10),
            );

            let mut overlap = false;
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                }
            }

            // if overlap is false then this will run
            if !overlap {
                room.for_each(|p| {
                    if p.x > 0 && p.x < SCREEN_WIDTH && p.y > 0 && p.y < SCREEN_HEIGHT {
                        let idx = map_idx(p.x, p.y); // get the specific tile in map
                        self.map.tiles[idx] = TileType::Floor;
                    }
                });

                self.rooms.push(room);
            }
        }
    }

    fn vertical_hallways(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{max, min};
        for y in min(y1, y2)..=max(y1, y2) {
            // if in_bound then return tile
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn horizontal_hallways(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{max, min};
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn build_hallways(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();

            if rng.range(0, 2) == 1 {
                self.horizontal_hallways(prev.x, new.x, prev.y);
                self.vertical_hallways(prev.y, new.y, new.x);
            } else {
                self.vertical_hallways(prev.y, new.y, prev.x);
                self.horizontal_hallways(prev.x, new.x, new.y);
            }
        }
    }

    fn monster_spawn_points(&self, start: &Point, rng: &mut RandomNumberGenerator) -> Vec<Point> {
        const NUM_MONSTERS: usize = 50;
        let mut spawn_points: Vec<Point> = self
            .map
            .tiles
            .iter()
            .enumerate()
            .filter(|(idx, tile)| {
                **tile == TileType::Floor
                    && DistanceAlg::Pythagoras.distance2d(*start, self.map.index_to_point2d(*idx))
                        > 10.0
            })
            .map(|(idx, _)| self.map.index_to_point2d(idx))
            .collect();
        let mut spawns = Vec::new();
        for _ in 0..NUM_MONSTERS {
            let target_index = rng.random_slice_index(&spawn_points).unwrap();
            spawns.push(spawn_points[target_index].clone());
            spawn_points.remove(target_index);
        }

        spawns
    }
}

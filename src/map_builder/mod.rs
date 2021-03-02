use crate::prelude::*;

mod automata;
mod drunkard;
mod dungeon_rooms;
mod empty;
mod labyruary;
mod prefabs;
mod themes;

use automata::CellularAutomataArchitect;
use drunkard::DrunkardsWalkArchitect;
use dungeon_rooms::DungeonRoomsArchitect;
use empty::EmptyArchitect;
use labyruary::LabyruaryArchitect;
use prefabs::apply_prefab;
use themes::*;

pub trait MapTheme: Sync + Send {
    fn tile_to_render(&self, tile_type: TileType) -> (FontCharType, ColorPair);
}

trait MapArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder;
}

const NUM_ROOMS: usize = 30;
pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub monster_spawns: Vec<Point>,
    pub player_start: Point,
    pub egg_start: Point,
    pub theme: Box<dyn MapTheme>,
}

impl MapBuilder {
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut architect: Box<dyn MapArchitect> = match rng.range(0, 100) {
            0..=20 => Box::new(DrunkardsWalkArchitect {}),
            21..=45 => Box::new(DungeonRoomsArchitect {}),
            46..=70 => Box::new(LabyruaryArchitect {}),
            _ => Box::new(CellularAutomataArchitect {}),
        };

        let mut mb = architect.new(rng);
        apply_prefab(&mut mb, rng);

        // So now we create new themes in the constructors of the mb:s,
        // consider a better solution since we're essentially doing it twice
        // (not that it matters for performance really)
        mb.theme = match rng.range(0, 2) {
            0 => DungeonTheme::new(),
            _ => ForestTheme::new(),
        };

        mb
    }

    fn spawn_monsters(&self, start: &Point, rng: &mut RandomNumberGenerator) -> Vec<Point> {
        const NUM_MONSTERS: usize = 50;
        let mut spawnable_tiles: Vec<Point> = self
            .map
            .tiles
            .iter()
            .enumerate()
            .filter(|(idx, t)| {
                **t == TileType::Ground
                    && DistanceAlg::Pythagoras.distance2d(*start, self.map.index_to_point2d(*idx))
                        > 10.0
            })
            .map(|(idx, _)| self.map.index_to_point2d(idx))
            .collect();

        let mut spawns = Vec::new();
        for _ in 0..NUM_MONSTERS {
            let target_index = rng.random_slice_index(&spawnable_tiles).unwrap();
            spawns.push(spawnable_tiles[target_index].clone());
            spawnable_tiles.remove(target_index);
        }
        spawns
    }

    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn find_most_distant(&self, start_point: Point) -> Point {
        let dijkstra_map = DijkstraMap::new(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            &vec![self.map.point2d_to_index(start_point)],
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

    fn apply_tiles_to_map(&mut self, rect: &Rect, tile: TileType) {
        for y in rect.y1..=rect.y2 {
            for x in rect.x1..=rect.x2 {
                let idx = map_idx(x, y);
                self.map.tiles[idx] = tile;
            }
        }
    }

    fn carve_random_rooms(
        &mut self,
        rng: &mut RandomNumberGenerator,
        fill_tile: TileType,
        (min_width, max_width): (i32, i32),
        (min_height, max_height): (i32, i32),
        allow_overlap: bool,
    ) {
        while self.rooms.len() < NUM_ROOMS {
            let w = rng.range(min_width, max_width);
            let h = rng.range(min_height, max_height);
            let x = rng.range(1, SCREEN_WIDTH - w - 1) - 1;
            let y = rng.range(1, SCREEN_HEIGHT - h - 1) - 1;

            let room = Rect::with_size(x, y, w, h);

            let mut overlap = false;
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                }
            }

            if !overlap || allow_overlap == true {
                // room.for_each(|p| {
                //     if p.x > 0 && p.x < SCREEN_WIDTH && p.y > 0 && p.y < SCREEN_HEIGHT {
                //         let idx = map_idx(p.x, p.y);
                //         self.map.tiles[idx] = fill_tile;
                //     }
                // });
                self.apply_tiles_to_map(&room, fill_tile);

                self.rooms.push(room)
            }
        }
    }

    fn place_random_buildings(
        &mut self,
        num_of_buildings: usize,
        rng: &mut RandomNumberGenerator,
        wall_tile: TileType,
        inner_tile: TileType,
        (min_width, max_width): (i32, i32),
        (min_height, max_height): (i32, i32),
        allow_overlap: bool,
    ) {
        for _ in 0..num_of_buildings {
            let w = rng.range(min_width, max_width);
            let h = rng.range(min_height, max_height);
            let x = rng.range(1, SCREEN_WIDTH - w - 1) - 1;
            let y = rng.range(1, SCREEN_HEIGHT - h - 1) - 1;

            let building = Rect::with_size(x, y, w, h);
            let cavity = Rect::with_size(x + 1, y + 1, w - 2, h - 2);

            let mut overlap = false;
            for r in self.rooms.iter() {
                if r.intersect(&building) {
                    overlap = true;
                }
            }

            if !overlap || allow_overlap == true {
                self.apply_tiles_to_map(&building, wall_tile);
                self.apply_tiles_to_map(&cavity, inner_tile);

                self.rooms.push(building);
            }
        }
    }

    fn apply_doors(&mut self, rng: &mut RandomNumberGenerator) {
        for room in self.rooms.clone() {
            for door_dir in 0..4 {
                let (door_y, door_x) = match door_dir {
                    // top
                    1 => (room.y1, rng.range(room.x1 + 1, room.x2)),
                    // right
                    2 => (rng.range(room.y1 + 1, room.y2), room.x2),
                    //bottom
                    3 => (room.y2, rng.range(room.x1 + 1, room.x2)),
                    //left
                    _ => (rng.range(room.y1 + 1, room.y2), room.x1),
                };

                let door_idx = map_idx(door_x, door_y);

                self.map.tiles[door_idx] = TileType::Door;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{max, min};
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx as usize] = TileType::Ground;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{max, min};
        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx as usize] = TileType::Ground;
            }
        }
    }

    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();

            if rng.range(0, 2) == 1 {
                self.apply_horizontal_tunnel(prev.x, new.x, prev.y);
                self.apply_vertical_tunnel(prev.y, new.y, new.x);
            } else {
                self.apply_vertical_tunnel(prev.y, new.y, prev.x);
                self.apply_horizontal_tunnel(prev.x, new.x, new.y);
            }
        }
    }
}

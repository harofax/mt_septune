use crate::prelude::*;
use super::MapArchitect;

pub struct CellularAutomataArchitect {}

impl MapArchitect for CellularAutomataArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder{
            map: Map::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            egg_start: Point::zero()
        };

        self.random_noise_map(rng, &mut mb.map);

        //for _ in 0..1 {
        //    self.smooth(&mut mb.map);
        //}
        let player_start = self.find_start(&mb.map);

        /* ---- cool winding tunnel caves */

        self.smooth(&mut mb.map);


        mb.apply_horizontal_tunnel(1, SCREEN_WIDTH, player_start.y);
        mb.apply_horizontal_tunnel(1, SCREEN_WIDTH, player_start.y + 1);
        mb.apply_horizontal_tunnel(1, SCREEN_WIDTH, player_start.y + 2);

        let vert_tunnel_x = rng.range(1, SCREEN_WIDTH-10);

        mb.apply_vertical_tunnel(10, SCREEN_HEIGHT - 10, vert_tunnel_x);
        mb.apply_vertical_tunnel(10, SCREEN_HEIGHT - 10, vert_tunnel_x + 1);
        mb.apply_vertical_tunnel(10, SCREEN_HEIGHT - 10, vert_tunnel_x + 2);

        for _ in 0..3 {
            self.smooth(&mut mb.map);
        }

        let dijkstra_map = DijkstraMap::new(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            &vec![mb.map.point2d_to_index(player_start)],
            &mb.map,
            1024.0
        );
        dijkstra_map.map
            .iter()
            .enumerate()
            .filter(|(_, distance)| *distance > &2000.0)
            .for_each(|(idx, _)| mb.map.tiles[idx] = TileType::Wall);


        //for _ in 0..4 {
        //    self.iterate(&mut mb.map);
        //}
        // */


        mb.monster_spawns = mb.spawn_monsters(&player_start, rng);
        mb.player_start = player_start;
        mb.egg_start = mb.find_most_distant(player_start);

        mb
    }
}

impl CellularAutomataArchitect {
    fn random_noise_map(
        &mut self,
        rng: &mut RandomNumberGenerator,
        map: &mut Map)
    {
        map.tiles.iter_mut().for_each(|t| {
            let roll = rng.range(0, 100);
            if roll > 50 {
                *t = TileType::Asphalt;
            } else {
                *t = TileType::Wall;
            }
        });
    }

    fn count_neighbours(&self, x: i32, y: i32, map: &Map) -> usize {
        let mut neighbours = 0;

        for iy in -1..= 1 {
            for ix in -1 ..= 1 {
                if !(ix == 0 && iy == 0) &&
                    map.tiles[map_idx(x+ix, y+iy)] == TileType::Wall
                {
                    neighbours += 1;
                }
            }
        }
        neighbours
    }

    fn smooth(&mut self, map: &mut Map) {
        let mut new_tiles = map.tiles.clone();

        for y in 1 .. SCREEN_HEIGHT -1 {
            for x in 1 .. SCREEN_WIDTH -1 {
                let mut floor_tiles = 0;
                let mut wall_tiles = 0;

                for ox in -1..2 {
                    for oy in -1..2 {
                        if map.tiles[map_idx(x + ox, y + oy)] == TileType::Asphalt {
                            floor_tiles += 1;
                        } else {
                            wall_tiles += 1;
                        }
                    }
                }

                new_tiles[map_idx(x, y)] = if floor_tiles >= wall_tiles {
                    TileType::Asphalt
                } else {
                    TileType::Wall
                };

                /*
                let neighbours = self.count_neighbours(x, y, map);
                let idx = map_idx(x, y);
                if neighbours > 4 || neighbours == 0 {
                    new_tiles[idx] = TileType::Wall;
                } else {
                    new_tiles[idx] = TileType::Asphalt;
                } */
            }
        }
        map.tiles = new_tiles;
    }

    fn iterate(&mut self, map: &mut Map) {
        let mut new_tiles = map.tiles.clone();

        for y in 1 .. SCREEN_HEIGHT -1 {
            for x in 1 .. SCREEN_WIDTH -1 {
                let neighbours = self.count_neighbours(x, y, map);
                let idx = map_idx(x, y);
                if neighbours > 4 || neighbours == 0 {
                    new_tiles[idx] = TileType::Wall;
                } else {
                    new_tiles[idx] = TileType::Asphalt;
                }
            }
        }
        map.tiles = new_tiles;
    }

    fn find_start(&self, map: &Map) -> Point {
        let center = Point::new(SCREEN_WIDTH/2, SCREEN_HEIGHT/2);
        let closest_point = map.tiles
            .iter()
            .enumerate()
            .filter(|(_, t)| **t == TileType::Asphalt)
            .map(|(idx, _)| (idx, DistanceAlg::Pythagoras.distance2d(
                center,
                map.index_to_point2d(idx)
            )))
            .min_by(|(_, distance), (_, distance2)|
            distance.partial_cmp(&distance2).unwrap()
            )
            .map(|(idx, _)| idx)
            .unwrap();

        map.index_to_point2d(closest_point)
    }
}
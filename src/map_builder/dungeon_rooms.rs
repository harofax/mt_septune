use super::MapArchitect;
use crate::prelude::*;

pub struct DungeonRoomsArchitect {}

impl MapArchitect for DungeonRoomsArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            egg_start: Point::zero(),
        };

        mb.fill(TileType::Wall);
        mb.carve_random_rooms(rng, TileType::Asphalt, (8, 14), (6, 13), false);
        mb.build_corridors(rng);

        mb.player_start = mb.rooms[0].center();
        mb.egg_start = mb.find_most_distant(mb.player_start);

        for room in mb.rooms.iter().skip(1) {
            mb.monster_spawns.push(room.center());
        }

        mb
    }
}

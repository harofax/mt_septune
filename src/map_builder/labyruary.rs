use super::MapArchitect;
use crate::prelude::*;

pub struct LabyruaryArchitect {}

const NUM_BUILDINGS: usize = 90;

impl MapArchitect for LabyruaryArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            egg_start: Point::zero(),
            theme: super::themes::DungeonTheme::new(),
        };

        mb.fill(TileType::Grass);
        mb.place_random_buildings(
            NUM_BUILDINGS,
            rng,
            TileType::Wall,
            TileType::Floorboard,
            (6, 14),
            (6, 12),
            true,
        );
        mb.apply_doors(rng);
        mb.place_random_buildings(
            20,
            rng,
            TileType::Floorboard,
            TileType::Grass,
            (2, 12),
            (2, 12),
            true,
        );

        mb.player_start = mb.rooms[0].center();
        mb.egg_start = mb.find_most_distant(mb.player_start);

        for room in mb.rooms.iter().skip(NUM_BUILDINGS) {
            mb.monster_spawns.push(room.center());
        }

        mb
    }
}

// impl LabyruaryArchitect {
//     fn place_rooms(
//         &mut self,
//         rng: &mut RandomNumberGenerator,
//         map: &mut Map,
//         rooms: &mut Vec<Rect>,
//     ) {
//         while rooms.len() < NUM_BUILDINGS {
//             let w = rng.range(6, 13);
//             let h = rng.range(6, 13);
//             let x = rng.range(1, SCREEN_WIDTH - w - 1) - 1;
//             let y = rng.range(1, SCREEN_HEIGHT - h - 1) - 1;

//             let building = Rect::with_size(x, y, w, h);
//             let cavity = Rect::with_size(x + 1, y + 1, w - 2, h - 2);

//             map.apply_tiles(building)
//         }
//     }
// }

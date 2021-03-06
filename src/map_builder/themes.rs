use crate::prelude::*;

pub struct DungeonTheme {}

impl DungeonTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}

impl MapTheme for DungeonTheme {
    fn tile_to_render(&self, tile_type: TileType) -> (FontCharType, ColorPair) {
        match tile_type {
            TileType::Ground => (
                to_cp437('░'),
                ColorPair::new(RGB::from_u8(26, 26, 32), RGB::from_u8(11, 11, 15)),
            ),
            TileType::Wall => (
                to_cp437('┬'),
                ColorPair::new(RGB::from_u8(179, 75, 84), RGB::from_u8(214, 110, 105)),
            ),
            TileType::Floorboard => (
                to_cp437('-'),
                ColorPair::new(RGB::from_u8(102, 73, 53), RGB::from_u8(150, 100, 72)),
            ),
            TileType::Grass => (
                to_cp437('"'),
                ColorPair::new(RGB::from_u8(2, 219, 158), RGB::from_u8(2, 168, 129)),
            ),
            TileType::Door => (
                to_cp437('▬'),
                ColorPair::new(RGB::from_u8(214, 110, 105), RGB::from_u8(150, 100, 72)),
            ),
        }
    }
}

pub struct ForestTheme {}

impl MapTheme for ForestTheme {
    fn tile_to_render(&self, tile_type: TileType) -> (FontCharType, ColorPair) {
        match tile_type {
            TileType::Ground => (
                to_cp437(','),
                ColorPair::new(RGB::from_u8(0, 140, 90), RGB::from_u8(0, 64, 64)),
            ),
            TileType::Wall => (
                to_cp437('♠'),
                ColorPair::new(RGB::from_u8(0, 178, 89), RGB::from_u8(0, 64, 64)),
            ),
            TileType::Floorboard => (
                to_cp437('≈'),
                ColorPair::new(RGB::from_u8(26, 20, 13), RGB::from_u8(51, 41, 26)),
            ),
            TileType::Grass => (
                to_cp437('`'),
                ColorPair::new(RGB::from_u8(0, 140, 105), RGB::from_u8(0, 64, 64)),
            ),
            TileType::Door => (
                to_cp437('∩'),
                ColorPair::new(RGB::from_u8(128, 128, 128), RGB::from_u8(0, 64, 64)),
            ),
        }
    }
}

impl ForestTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}

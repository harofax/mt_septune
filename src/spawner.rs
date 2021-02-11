use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push(
        (
            Player,
            pos,
            Render {
                color: ColorPair::new(RGB::from_u8(255, 204, 51), BLACK),
                glyph: to_cp437('@')
            }
        )
    );
}

pub fn spawn_monster(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    pos: Point
) {
    ecs.push(
        (Enemy,
        pos,
        Render {
            color: ColorPair::new(RGB::from_u8(9, 170, 129), BLACK),
            glyph : match rng.range(0, 4) {
                0 => to_cp437('Å'),
                1 => to_cp437('o'),
                2 => to_cp437('Ω'),
                3 => to_cp437('☻'),
                _ => to_cp437('r'),
            }
        },
        MovingRandomly{},
        )
    );
}
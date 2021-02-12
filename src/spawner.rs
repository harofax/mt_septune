use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push(
        (
            Player,
            pos,
            Render {
                color: ColorPair::new(RGB::from_u8(255, 204, 51), BLACK),
                glyph: to_cp437('@')
            },
            Health {current: 10, max: 10}
        )
    );
}

fn rat() -> (i32, String, FontCharType, ColorPair) {
    (1, "Rat".to_string(), to_cp437('r'), ColorPair::new(GRAY, BLACK))
}

fn ombolonian() -> (i32, String, FontCharType, ColorPair) {
    (2, "Ombolonian".to_string(), to_cp437('o'), ColorPair::new(MAGENTA, BLACK))
}

pub fn spawn_monster(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    pos: Point
) {
    let (hp, name, glyph, color) = match rng.roll_dice(1, 10) {
        1..=8 => rat(),
        _ => ombolonian(),
    };

    ecs.push(
        (Enemy,
         pos,
         Render {
             color,
             glyph,
         },
         ChasingPlayer,
         Health {current: hp, max: hp},
         Name(name)
        )
    );
}
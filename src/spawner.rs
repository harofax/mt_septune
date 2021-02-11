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
            Health {current: 20, max: 20}
        )
    );
}

fn rat() -> (i32, String, FontCharType) {
    (1, "Rat".to_string(), to_cp437('r'))
}

fn ombolonian() -> (i32, String, FontCharType) {
    (2, "Ombolonian".to_string(), to_cp437('o'))
}

pub fn spawn_monster(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    pos: Point
) {
    let (hp, name, glyph) = match rng.roll_dice(1, 10) {
        1..=8 => rat(),
        _ => ombolonian()
    };

    ecs.push(
        (Enemy,
         pos,
         Render {
             color: ColorPair::new(RGB::from_u8(9, 170, 129), BLACK),
             glyph,
         },
         MovingRandomly{},
         Health {current: hp, max: hp},
         Name(name)
        )
    );
}
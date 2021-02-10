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
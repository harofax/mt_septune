use crate::prelude::*;

#[system]
pub fn map_render(#[resource] map: &Map, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);

    for y in camera.top_y ..= camera.bottom_y {
        for x in camera.left_x .. camera.right_x {
            let pt = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);
            if map.in_bounds(pt) {
                let idx = map_idx(x, y);
                let (glyph, colors) = match map.tiles[idx] {
                    TileType::Asphalt => (to_cp437('░'),
                                          ColorPair::new(
                                              RGB::from_u8(26, 26, 32),
                                              RGB::from_u8(11, 11, 15)
                                          )
                    ),
                    TileType::Wall => (to_cp437('╪'),
                                       ColorPair::new(
                                           RGB::from_u8(140, 123, 157),
                                           RGB::from_u8(165, 152, 179)
                                       )
                    ),
                    _ => (to_cp437('!'),
                        ColorPair::new(
                            RED,
                            PINK
                        )
                    ),
                };


                draw_batch.set(
                    pt - offset,
                    colors,
                    glyph
                );
            }
        }
    }
    //                v--- sort order for drawing, 0 is first
    draw_batch.submit(0).expect("Batch Submit Error!");
}
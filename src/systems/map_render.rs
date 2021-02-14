use crate::prelude::*;
use std::ops::Mul;

#[system]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn map_render(
    #[resource] map: &Map,
    #[resource] camera: &Camera,
    ecs: &SubWorld
) {
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);

    let player_fov = fov.iter(ecs).nth(0).unwrap();

    for y in camera.top_y ..= camera.bottom_y {
        for x in camera.left_x .. camera.right_x {
            let pt = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);
            let idx = map_idx(x, y);
            if map.in_bounds(pt) && (player_fov.visible_tiles.contains(&pt)
                | map.revealed_tiles[idx]) {

                let (glyph, mut colors) = match map.tiles[idx] {
                    TileType::Asphalt => (to_cp437('░'),
                                          ColorPair::new(
                                              RGB::from_u8(26, 26, 32),
                                              RGB::from_u8(11, 11, 15)
                                          )
                    ),
                    TileType::Wall => (to_cp437('┬'),
                                       ColorPair::new(
                                           RGB::from_u8(179, 75, 84),
                                           RGB::from_u8(214, 110, 105)
                                       )
                    ),
                    _ => (to_cp437('!'),
                        ColorPair::new(
                            RED,
                            PINK
                        )
                    ),
                };

                if !player_fov.visible_tiles.contains(&pt) {
                    colors.fg = colors.fg.to_greyscale().mul(0.6);
                    colors.bg = colors.bg.to_greyscale().mul(0.8);
                }


                draw_batch.set(
                    pt - offset,
                    colors,
                    glyph
                );
            }
        }
    }
    draw_batch.submit(0).expect("Batch Submit Error!");
    //               ^^^----- draw console, 0 is our map console
}
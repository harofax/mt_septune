use crate::prelude::*;

#[system] // adds _system to fn name, so entity_render() becomes entity_render_system. also does other stuff, under da hood
#[read_component(Point)]
#[read_component(Render)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn entity_render(
    #[resource] camera: &Camera,
    ecs: &SubWorld,
) {
    let mut renderables = <(&Point, &Render)>::query();
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1); // target console 1: entity layer
    let offset = Point::new(camera.left_x, camera.top_y);

    let player_fov = fov.iter(ecs).nth(0).unwrap();

    renderables
        .iter(ecs)
        .filter(|(pos, _)| player_fov.visible_tiles.contains(&pos))
        .for_each(|(pos, render)| {
            draw_batch.set(
                *pos - offset,
                render.color,
                render.glyph
            );
        }
    );

    draw_batch.submit(5000).expect("Batch error");
}

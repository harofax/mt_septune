use crate::prelude::*;

#[system] // adds _system to fn name, so entity_render() becomes entity_render_system. also does other stuff, under da hood
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1); // target console
    let offset = Point::new(camera.left_x, camera.top_y);

    <(&Point, &Render)>::query()
        .iter(ecs)
        .for_each(|(pos, render)| {
            draw_batch.set(
                *pos - offset,
                render.color,
                render.glyph
            );
        }
        );
    draw_batch.submit(5000).expect("Draw Batch Submit error, entity_render");
}
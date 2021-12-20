use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);
    let offset = Point::new(camera.left_x, camera.right_x);

    <(&Point, &Render)>::query()
        .iter(ecs)
        .for_each(|(position, render)| {
            draw_batch.set(*position - offset, render.color, render.glyph);
        });

    draw_batch.submit(5000).expect("Batch error");
}

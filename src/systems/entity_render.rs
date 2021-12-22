use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(sub_world: &SubWorld, #[resource] camera: &Camera) {
    let mut render_batch = DrawBatch::new();
    render_batch.target(1);
    let offset = Point::new(camera.left_x, camera.top_y);

    <(&Point, &Render)>::query()
        .iter(sub_world)
        .for_each(|(pos, rend)| {
            render_batch.set(*pos - offset, rend.color, rend.glyph);
        });
    render_batch
        .submit(5000)
        .expect("Entities Failed to Render");
}

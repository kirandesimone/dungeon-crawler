use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn entity_render(sub_world: &SubWorld, #[resource] camera: &Camera) {
    let mut renderables = <(&Point, &Render)>::query();
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = fov.iter(sub_world).nth(0).unwrap();

    let mut render_batch = DrawBatch::new();
    render_batch.target(1);
    let offset = Point::new(camera.left_x, camera.top_y);

    renderables
        .iter(sub_world)
        .filter(|(pos, _)| player_fov.visible_tiles.contains(&pos))
        .for_each(|(pos, rend)| {
            render_batch.set(*pos - offset, rend.color, rend.glyph);
        });
    render_batch
        .submit(5000)
        .expect("Entities Failed to Render");
}

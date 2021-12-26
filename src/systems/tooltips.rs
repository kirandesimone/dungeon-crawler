use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Health)]
#[read_component(Name)]
pub fn tooltips(sub_world: &SubWorld, #[resource] mouse_pos: &Point, #[resource] camera: &Camera) {
    let mut positions = <(Entity, &Point, &Name)>::query();
    let offset = Point::new(camera.left_x, camera.top_y);
    let map_pos = *mouse_pos + offset;
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    positions.iter(sub_world).for_each(|(entity, pos, name)| {
        if *pos == map_pos {
            let screen_pos = *mouse_pos * 4;
            let display = if let Ok(health) = sub_world
                .entry_ref(*entity)
                .unwrap()
                .get_component::<Health>()
            {
                format!("{} : hp {}", &name.0, health.current)
            } else {
                name.0.clone()
            };
            draw_batch.print(screen_pos, &display);
        }
    });
    draw_batch.submit(10100).expect("Tooltips failed to load");
}

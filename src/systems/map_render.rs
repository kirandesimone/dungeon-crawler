use crate::prelude::*;

#[system]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn map_render(
    sub_world: &SubWorld,
    #[resource] map: &Map,
    #[resource] camera: &Camera,
    #[resource] theme: &Box<dyn MapTheme>,
) {
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = fov.iter(sub_world).nth(0).unwrap();
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);
    let offset = Point::new(camera.left_x, camera.top_y);
    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..=camera.right_x {
            let p = Point::new(x, y);
            let idx = map_idx(p.x, p.y);
            if map.in_bounds(p) && (player_fov.visible_tiles.contains(&p) | map.revealed_tiles[idx])
            {
                let tint = if player_fov.visible_tiles.contains(&p) {
                    WHITE
                } else {
                    DARK_GRAY
                };

                let glyph = theme.tile_to_render(map.tiles[idx]);

                draw_batch.set(p - offset, ColorPair::new(tint, BLACK), glyph);
            }
        }
    }
    draw_batch.submit(0).expect("Map Render Failed");
}

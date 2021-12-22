use crate::prelude::*;

#[system]
pub fn map_render(#[resource] map: &Map, #[resource] camera: &Camera) {
    let mut render_batch = DrawBatch::new();
    render_batch.target(0);
    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..=camera.right_x {
            let p = Point::new(x, y);
            if map.in_bounds(p) {
                let idx = map_idx(p.x, p.y);
                let glyph = match map.tiles[idx] {
                    TileType::Floor => to_cp437('.'),
                    TileType::Wall => to_cp437('#'),
                };
                let offset = Point::new(camera.left_x, camera.top_y);
                render_batch.set(p - offset, ColorPair::new(WHITE, BLACK), glyph);
            }
        }
    }
    render_batch.submit(0).expect("Map Render Failed");
}

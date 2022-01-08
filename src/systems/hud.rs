use crate::prelude::*;

/// JUST FOR DISPLAYING HUD
#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Name)]
#[read_component(Item)]
#[read_component(Carried)]
pub fn display_hud(sub_world: &SubWorld) {
    let mut health_query = <&Health>::query().filter(component::<Player>());
    let player_health = health_query.iter(sub_world).nth(0).unwrap();
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    draw_batch.print_centered(1, "Explore the dungeon with WASD");
    let green_health = HSV {
        h: 149.0,
        s: 67.0,
        v: 76.0,
    };

    draw_batch.bar_horizontal(
        Point::zero(),
        SCREEN_WIDTH * 2,
        player_health.current,
        player_health.max,
        ColorPair::new(green_health.to_rgba(1.0), BLACK),
    );
    draw_batch.print_color_centered(
        0,
        format!("Health: {} / {}", player_health.current, player_health.max),
        ColorPair::new(WHITE, RED),
    );

    let (_player, map_level) = <(Entity, &Player)>::query()
        .iter(sub_world)
        .find_map(|(entity, player)| Some((*entity, player.map_level)))
        .unwrap();
    draw_batch.print_color_right(
        Point::new(SCREEN_WIDTH * 2, 1),
        format!("Dungeon Level: {}", map_level + 1),
        ColorPair::new(YELLOW, BLACK),
    );

    let player = <(Entity, &Player)>::query()
        .iter(sub_world)
        .find_map(|(entity, _player)| Some(*entity))
        .unwrap();
    let mut items = <(&Item, &Carried, &Name)>::query();
    let mut y = 3;
    items
        .iter(sub_world)
        .filter(|(_, carried, _)| carried.0 == player)
        .for_each(|(_, _, name)| {
            draw_batch.print(Point::new(3, y), format!("{} : {}", y - 2, &name.0));
            y += 1;
        });
    if y > 3 {
        draw_batch.print_color(Point::new(3, 2), "Inventory", ColorPair::new(YELLOW, BLACK));
    }
    draw_batch.submit(10000).expect("FAILD LOADING HUD");
}

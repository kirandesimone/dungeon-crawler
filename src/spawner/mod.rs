mod template;

use crate::prelude::*;
use template::Templates;

/*
    This is no longer applicable for all entites
    X - This file spawn entities by pushing components

    now loading data from RON file
*/

pub fn spawn_level(
    world: &mut World,
    rng: &mut RandomNumberGenerator,
    map_level: usize,
    spawn_points: &[Point],
    resources: &mut Resources,
) {
    let template = Templates::load();
    template.spawn_entities(world, rng, map_level, spawn_points, resources);
}

pub fn spawn_player(world: &mut World, pos: Point) {
    // this will create an entity with these components
    world.push((
        Player { map_level: 0 },
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
        Health {
            current: 10,
            max: 10,
        },
        FieldOfView::new(8),
        Damage(1),
    ));
}

pub fn spawn_amulet_yala(world: &mut World, pos: Point) {
    world.push((
        Item,
        AmuletOfYala,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('|'),
        },
        Name("Amulet of YALA".to_string()),
    ));
}

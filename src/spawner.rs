use crate::prelude::*;

/*
    This file spawn entities by pushing components
*/

pub fn spawn_player(world: &mut World, pos: Point) {
    // this will create an entity with these components
    world.push((
        Player,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
    ));
}

pub fn spawn_enemy(world: &mut World, pos: Point, rng: &mut RandomNumberGenerator) {
    world.push((
        Enemy,
        pos,
        MoveRandomly,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: match rng.range(0, 4) {
                0 => to_cp437('E'),
                1 => to_cp437('O'),
                2 => to_cp437('o'),
                _ => to_cp437('g'),
            },
        },
    ));
}

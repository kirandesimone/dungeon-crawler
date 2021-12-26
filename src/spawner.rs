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
        Health {
            current: 20,
            max: 20,
        },
    ));
}

fn goblin() -> (i32, String, FontCharType) {
    (1, "Goblin".to_string(), to_cp437('g'))
}

fn orc() -> (i32, String, FontCharType) {
    (2, "Orc".to_string(), to_cp437('o'))
}

pub fn spawn_enemy(world: &mut World, pos: Point, rng: &mut RandomNumberGenerator) {
    let (hp, name, glyph) = match rng.roll_dice(1, 10) {
        1..=8 => goblin(),
        _ => orc(),
    };
    world.push((
        Enemy,
        pos,
        MoveRandomly,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph,
        },
        Health {
            current: hp,
            max: hp,
        },
        Name(name),
    ));
}

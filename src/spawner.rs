use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player,
        pos,
        Render {
            color: ColorPair::new(RGB::from_u8(255, 204, 51), BLACK),
            glyph: to_cp437('@'),
        },
        Health {
            current: 10,
            max: 10,
        },
        FieldOfView::new(8),
    ));
}

pub fn spawn_entity(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    let roll = rng.roll_dice(1, 6);
    match roll {
        1 => spawn_healing_potion(ecs, pos),
        2 => spawn_magic_mapper(ecs, pos),
        _ => spawn_monster(ecs, rng, pos),
    }
}

pub fn spawn_healing_potion(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        pos,
        Render {
            color: ColorPair::new(RED, BLACK),
            glyph: to_cp437('!'),
        },
        Name("Healing potion".to_string()),
        ProvidesHealing { amount: 6 },
    ));
}

pub fn spawn_magic_mapper(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        pos,
        Render {
            color: ColorPair::new(BEIGE, BLACK),
            glyph: to_cp437('{'),
        },
        Name("Magic Scroll".to_string()),
        ProvidesDungeonMap {},
    ));
}

pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    let (hp, name, glyph, color) = match rng.roll_dice(1, 10) {
        1..=8 => rat(),
        _ => ombolonian(),
    };

    ecs.push((
        Enemy,
        pos,
        Render { color, glyph },
        ChasingPlayer,
        Health {
            current: hp,
            max: hp,
        },
        Name(name),
        FieldOfView::new(6),
    ));
}

fn rat() -> (i32, String, FontCharType, ColorPair) {
    (
        1,
        "Rat".to_string(),
        to_cp437('r'),
        ColorPair::new(GRAY, BLACK),
    )
}

fn ombolonian() -> (i32, String, FontCharType, ColorPair) {
    (
        2,
        "Ombolonian".to_string(),
        to_cp437('o'),
        ColorPair::new(MAGENTA, BLACK),
    )
}

pub fn spawn_cosmic_egg(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        CosmicEgg,
        pos,
        Render {
            color: ColorPair::new(MAGENTA, BLACK),
            glyph: to_cp437('Φ'),
        },
        Name("The Cosmic Egg of Eternity".to_string()),
    ));
}

use crate::prelude::*;
use legion::systems::CommandBuffer;
use ron::de::from_reader;
use serde::Deserialize;
use std::collections::HashSet;
use std::fs::File;

#[derive(Deserialize, Clone, Debug)]
pub struct Template {
    pub entity_type: EntityType,
    pub name: String,
    pub glyph: char,
    pub levels: HashSet<usize>,
    pub provides: Option<Vec<(String, i32)>>,
    pub hp: Option<i32>,
    pub frequency: i32,
    pub base_damage: Option<i32>,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub enum EntityType {
    Enemy,
    Item,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Templates {
    pub entities: Vec<Template>,
}

impl Templates {
    pub fn load() -> Self {
        let file = File::open("./src/resources/template.ron")
            .expect("Could not open file in resources directoy");
        from_reader(file).expect("Could not deserialize file")
    }

    pub fn spawn_entities(
        &self,
        world: &mut World,
        rng: &mut RandomNumberGenerator,
        level: usize,
        spawn_points: &[Point],
        resources: &mut Resources,
    ) {
        let mut available_entities = Vec::new();
        self.entities
            .iter()
            .filter(|entity| entity.levels.contains(&level))
            .for_each(|e| {
                for _ in 0..e.frequency {
                    available_entities.push(e);
                }
            });

        let mut commands = CommandBuffer::new(world);
        spawn_points.iter().for_each(|pos| {
            if let Some(entity) = rng.random_slice_entry(&available_entities) {
                self.spawn_entity(pos, entity, &mut commands);
            }
        });

        commands.flush(world, resources);
    }

    fn spawn_entity(
        &self,
        pos: &Point,
        template: &Template,
        commands: &mut legion::systems::CommandBuffer,
    ) {
        let entity = commands.push((
            Name(template.name.clone()),
            pos.clone(),
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437(template.glyph),
            },
        ));

        match template.entity_type {
            EntityType::Item => commands.add_component(entity, Item),
            EntityType::Enemy => {
                commands.add_component(entity, Enemy);
                commands.add_component(entity, FieldOfView::new(6));
                commands.add_component(entity, ChasingPlayer);
                commands.add_component(
                    entity,
                    Health {
                        current: template.hp.unwrap(),
                        max: template.hp.unwrap(),
                    },
                );
            }
        }

        if let Some(effects) = &template.provides {
            effects
                .iter()
                .for_each(|(effect, n)| match effect.as_str() {
                    "Healing" => commands.add_component(entity, ProvidesHealing { amount: *n }),
                    "Magic Map" => commands.add_component(entity, ProvidesDungeonMap),
                    _ => println!("There is no action for {}", effect),
                })
        }

        if let Some(damage) = &template.base_damage {
            commands.add_component(entity, Damage(*damage));
            if template.entity_type == EntityType::Item {
                commands.add_component(entity, Weapon);
            }
        }
    }
}

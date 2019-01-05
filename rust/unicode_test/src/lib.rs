#![cfg_attr(test, feature(test))]

use std::cmp::Ordering;
extern crate criterion;
extern crate rand;

pub static ASCII_TEXT: &'static str = "Attend to hear 6 stellar #mobile #startups at #OF12 Entrepreneur Idol show 2day,  http://t.co/HtzEMgAC @TiEcon @sv_entrepreneur @500!";
pub static UNICODE_TEXT: &'static str = "Attend \u{20000}\u{20000} hear 6 stellar #mobile #startups at #OF12 Entrepreneur Idol show 2day,  http://t.co/HtzEMgAC @TiEcon @sv_entrepreneur @500!";

mod benchmark_base;
pub use benchmark_base::*;

#[derive(Clone, PartialEq, Hash, Eq)]
pub struct Entity<T> {
    start: usize,
    end: usize,
    html: T,
}

impl Ord for Entity<String> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start.cmp(&other.start)
    }
}

impl PartialOrd for Entity<String> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub type DecodedEntity = Entity<Vec<char>>;

impl Ord for DecodedEntity {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start.cmp(&other.start)
    }
}

impl PartialOrd for DecodedEntity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn render(text: &str, entities: &Vec<Entity<String>>) -> String {
    let mut sb = String::with_capacity(text.len() * 2);
    let mut my_entities = entities.clone();
    my_entities.sort_by(|e1, e2| e1.start.cmp(&e2.start));

    let mut pos = 0 as usize;
    for entity in my_entities {
        sb.extend(text.chars().skip(pos).take(entity.start - pos));
        sb.push_str(entity.html.as_str());
        pos = entity.end;
    }
    for c in text.chars().skip(pos).take(text.chars().count() - pos) {
        sb.push(c);
    }
    sb
}

pub fn render_chars(text: &Vec<char>, entities: &Vec<DecodedEntity>) -> String {
    let mut sb: Vec<char> = Vec::with_capacity(text.len() * 2);
    let mut my_entities = entities.clone();
    my_entities.sort_by(|e1, e2| e1.start.cmp(&e2.start));

    let mut pos = 0 as usize;
    for entity in my_entities {
        sb.extend_from_slice(&text[pos..entity.start]);
        sb.extend_from_slice(&entity.html);
        pos = entity.end;
    }
    sb.extend_from_slice(&text[pos..text.len()]);
    sb.into_iter().collect() // <-- UTF-8 encoding
}

pub fn render_chars2(text: &Vec<char>, entities: &Vec<Entity<String>>) -> String {
    let mut my_entities = entities.clone();
    my_entities.sort();
    let mut sb = String::with_capacity(text.len() * 2);
    let mut pos = 0 as usize;
    for entity in my_entities {
        for i in pos..entity.start {
            sb.push(text[i]);
        }
        sb.push_str(&entity.html);
        pos = entity.end;
    }
    for i in pos..text.len() {
        sb.push(text[i]);
    }
    sb
}

pub fn render_chars_entity_references(text: &Vec<char>, entities: &Vec<&Entity<String>>) -> String {
    let mut my_entities: Vec<&Entity<String>> = Vec::with_capacity(entities.len());
    for e in entities {
        my_entities.push(e);
    }
    my_entities.sort();

    let mut sb = String::with_capacity(text.len() * 2);
    let mut pos = 0 as usize;
    for entity in my_entities {
        for i in pos..entity.start {
            sb.push(text[i]);
        }
        sb.push_str(&entity.html);
        pos = entity.end;
    }
    for i in pos..text.len() {
        sb.push(text[i]);
    }
    sb
}

pub fn render_chars_entity_references_to_chars(
    text: &Vec<char>,
    entities: &Vec<&DecodedEntity>,
) -> Vec<char> {
    let mut my_entities: Vec<&DecodedEntity> = Vec::with_capacity(entities.len());
    for e in entities {
        my_entities.push(e);
    }
    my_entities.sort_unstable();

    let mut sb: Vec<char> = Vec::with_capacity(text.len() * 2);
    let mut pos = 0 as usize;
    for entity in my_entities {
        sb.extend_from_slice(&text[pos..entity.start]);
        sb.extend_from_slice(&entity.html);
        pos = entity.end;
    }
    sb.extend_from_slice(&text[pos..text.len()]);
    sb
}

#[derive(Copy, Clone, Debug)]
pub struct Coord {
    start: usize,
    end: usize,
}

pub fn render_coords(
    coordinates: &mut Vec<Coord>,
    text: &Vec<char>,
    entities: &Vec<&DecodedEntity>,
) {
    let mut pos = 0 as usize;
    for entity in entities {
        coordinates.push(Coord {
            start: pos,
            end: entity.start,
        });
        coordinates.push(Coord {
            start: 0,
            end: entity.html.len(),
        });
        pos = entity.end;
    }
    coordinates.push(Coord {
        start: pos,
        end: text.len(),
    });
}

pub fn coordinates_to_utf8(
    coordinates: &Vec<Coord>,
    text: &Vec<char>,
    entities: &Vec<&DecodedEntity>,
) -> String {
    let mut sb = String::with_capacity(text.len() * 2);
    let mut in_entity = false;
    let mut entity_index = 0;

    let mut source: &Vec<char> = text;

    for coord in coordinates {
        if in_entity {
            source = &entities[entity_index].html;
            entity_index += 1;
        } else {
            source = text;
        }

        for i in coord.start..coord.end {
            sb.push(source[i]);
        }
        in_entity = !in_entity;
    }

    sb
}

fn main() {
    let result = render(&ASCII_TEXT, &mut entities());
    println!("Result: {}", result);
}

pub fn entities() -> Vec<Entity<String>> {
    let entities = vec![
        Entity {
            start: 82,
            end: 102,
            html: "<http://t.co/HtzEMgAC>".to_string(),
        },
        Entity {
            start: 128,
            end: 132,
            html: "<@500>".to_string(),
        },
        Entity {
            start: 25,
            end: 32,
            html: "<#mobile>".to_string(),
        },
        Entity {
            start: 33,
            end: 42,
            html: "<#startups>".to_string(),
        },
        Entity {
            start: 111,
            end: 127,
            html: "<@sv_entrepreneur>".to_string(),
        },
        Entity {
            start: 46,
            end: 51,
            html: "<#OF12>".to_string(),
        },
        Entity {
            start: 103,
            end: 110,
            html: "<@TiEcon>".to_string(),
        },
    ];
    entities
}

pub fn decoded_entities(entities: Vec<Entity<String>>) -> Vec<DecodedEntity> {
    entities
        .iter()
        .map(|e| DecodedEntity {
            start: e.start,
            end: e.end,
            html: e.html.chars().collect(),
        })
        .collect()
}

pub fn entity_refs<'a, T>(entities: &'a Vec<T>) -> Vec<&'a T> {
    entities.into_iter().map(|e| e).collect()
}

#[cfg(test)]
extern crate test;
mod rendertest {
    use super::*;
    use rand::{self, Rng};

    #[test]
    fn correctness_chars() {
        let result = "Attend \u{20000}\u{20000} hear 6 stellar <#mobile> <#startups> at <#OF12> Entrepreneur Idol show 2day,  <http://t.co/HtzEMgAC> <@TiEcon> <@sv_entrepreneur> <@500>!";
        assert_eq!(
            result,
            render_chars(
                &UNICODE_TEXT.chars().collect(),
                &decoded_entities(entities())
            )
        )
    }

    #[test]
    fn correctness() {
        let result = "Attend \u{20000}\u{20000} hear 6 stellar <#mobile> <#startups> at <#OF12> Entrepreneur Idol show 2day,  <http://t.co/HtzEMgAC> <@TiEcon> <@sv_entrepreneur> <@500>!";
        assert_eq!(result, render(&UNICODE_TEXT, &entities()))
    }

    #[test]
    fn correctness_chars2() {
        let result = "Attend \u{20000}\u{20000} hear 6 stellar <#mobile> <#startups> at <#OF12> Entrepreneur Idol show 2day,  <http://t.co/HtzEMgAC> <@TiEcon> <@sv_entrepreneur> <@500>!";
        assert_eq!(
            result,
            render_chars2(&UNICODE_TEXT.chars().collect(), &entities())
        )
    }

    #[test]
    fn correctness_chars_entity_references() {
        let result = "Attend \u{20000}\u{20000} hear 6 stellar <#mobile> <#startups> at <#OF12> Entrepreneur Idol show 2day,  <http://t.co/HtzEMgAC> <@TiEcon> <@sv_entrepreneur> <@500>!";
        assert_eq!(
            result,
            render_chars_entity_references(
                &UNICODE_TEXT.chars().collect(),
                &entity_refs(&entities())
            )
        )
    }

    #[test]
    fn correctness_chars_entity_reference_to_chars() {
        let result = "Attend \u{20000}\u{20000} hear 6 stellar <#mobile> <#startups> at <#OF12> Entrepreneur Idol show 2day,  <http://t.co/HtzEMgAC> <@TiEcon> <@sv_entrepreneur> <@500>!";
        let chars = render_chars_entity_references_to_chars(
            &UNICODE_TEXT.chars().collect(),
            &entity_refs(&decoded_entities(entities())),
        );
        let s: String = chars.iter().collect();
        assert_eq!(result, s);
    }

    #[test]
    fn correctness_render_steps() {
        let result = "Attend \u{20000}\u{20000} hear 6 stellar <#mobile> <#startups> at <#OF12> Entrepreneur Idol show 2day,  <http://t.co/HtzEMgAC> <@TiEcon> <@sv_entrepreneur> <@500>!";

        // Decode from UTF-8
        let decoded = &decoded_entities(entities());
        let refs = &entity_refs(decoded);
        let text = &UNICODE_TEXT.chars().collect();

        // Sort entities
        let mut sorted: Vec<&DecodedEntity> = Vec::with_capacity(refs.len());
        for e in refs {
            sorted.push(e);
        }
        sorted.sort_unstable();

        // Render coordinates
        let mut ht = Vec::with_capacity(64);
        render_coords(&mut ht, text, &sorted);

        // Encode to UTF-8
        let s = coordinates_to_utf8(&ht, text, &sorted);

        assert_eq!(result, s);
    }
}

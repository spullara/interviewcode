#![cfg_attr(test, feature(test))]

extern crate jemallocator;
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

use std::cmp::Ordering;
use std::collections::BTreeSet;

pub static ASCII_TEXT: &'static str = "Attend to hear 6 stellar #mobile #startups at #OF12 Entrepreneur Idol show 2day,  http://t.co/HtzEMgAC @TiEcon @sv_entrepreneur @500!";
pub static UNICODE_TEXT: &'static str = "Attend \u{20000}\u{20000} hear 6 stellar #mobile #startups at #OF12 Entrepreneur Idol show 2day,  http://t.co/HtzEMgAC @TiEcon @sv_entrepreneur @500!";

#[derive(Clone, Eq, PartialEq)]
pub struct Entity {
    start: usize,
    end: usize,
    html: String
}

impl Ord for Entity {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start.cmp(&other.start)
    }
}

impl PartialOrd for Entity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct DecodedEntity {
    start: usize,
    end: usize,
    html: Vec<char>,
}

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

impl Entity {
    fn decode(&self) -> DecodedEntity {
        DecodedEntity {start: self.start, end: self.end, html: self.html.chars().collect()}
    }
}

fn render(text: &str, entities: &BTreeSet<Entity>) -> String {
    let mut sb = String::with_capacity(text.len()*2);
    let mut pos = 0 as usize;
    for entity in entities {
        sb.extend(text.chars().skip(pos).take(entity.start - pos));
        sb.push_str(entity.html.as_str());
        pos = entity.end;
    }
    for c in text.chars().skip(pos).take(text.chars().count() - pos) {
        sb.push(c);
    }
    sb
}

fn render_chars(text: &Vec<char>, entities: &BTreeSet<DecodedEntity>) -> String {
    let mut sb: Vec<char> = Vec::with_capacity(text.len()*2);
    let mut pos = 0 as usize;
    for entity in entities {
        sb.extend_from_slice(&text[pos..entity.start]);
        sb.extend_from_slice(&entity.html);
        pos = entity.end;
    }
    sb.extend_from_slice(&text[pos..text.len()]);
    sb.into_iter().collect()
}

fn render_chars_2(text: &Vec<char>, entities: &BTreeSet<Entity>) -> String {
    let mut sb = String::with_capacity(text.len()*2);
    let mut pos = 0 as usize;
    for entity in entities {
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

fn main() {
    let result = classic(&ASCII_TEXT, &entities());
    println!("Result: {}", result);
}


pub fn classic(text: &str, entities: &BTreeSet<Entity>) -> String {
    render(&text, entities)
}

pub fn classic_chars(text: &Vec<char>, entities: &BTreeSet<DecodedEntity>) -> String {
    render_chars(&text, entities)
}

pub fn classic_chars_2(text: &Vec<char>, entities: &BTreeSet<Entity>) -> String {
    render_chars_2(&text, entities)
}

pub fn entities() -> BTreeSet<Entity> {
    let mut entities = BTreeSet::new();
    entities.insert(Entity {start: 82, end: 102, html:"<http://t.co/HtzEMgAC>".to_string()});
    entities.insert(Entity {start: 128, end: 132, html:"<@500>".to_string()});
    entities.insert(Entity {start: 25, end: 32, html:"<#mobile>".to_string()});
    entities.insert(Entity {start: 33, end: 42, html:"<#startups>".to_string()});
    entities.insert(Entity {start: 111, end: 127, html:"<@sv_entrepreneur>".to_string()});
    entities.insert(Entity {start: 46, end: 51, html:"<#OF12>".to_string()});
    entities.insert(Entity {start: 103, end: 110, html:"<@TiEcon>".to_string()});

    entities
}

pub fn decoded_entities() -> BTreeSet<DecodedEntity> {
    let mut decoded_entities = BTreeSet::new();
    for entity in entities() {
        decoded_entities.insert(entity.decode());
    }
    decoded_entities
}


#[cfg(test)] extern crate rand;
#[cfg(test)] extern crate test;

#[cfg(test)]
mod rendertest {
    use super::*;
    use rand::{self,Rng};
    use test::Bencher;

    fn generate_entities() -> Vec<BTreeSet<Entity>> {
        let mut rng = rand::thread_rng();
        let mut entities_list: Vec<BTreeSet<Entity>> = Vec::with_capacity(1000);

        for _ in 0..1000 {
            let total = rng.gen::<usize>() % 10;
            let mut indices = Vec::with_capacity(total);
            for _ in 0..(total*2) {
                loop {
                    let index = rng.gen::<usize>() % ASCII_TEXT.len();
                    if !indices.contains(&index) {
                        indices.push(index);
                        break;
                    }
                }
            }

            indices.sort();
            let entities = indices.chunks(2).map(|chunk| {
                let (start, end) = (chunk[0], chunk[1]);
                let length = end - start;
                Entity {start: start, end: end, html: (0..length).map(|_| "X").collect()}
            });
            let mut set = BTreeSet::new();
            for entity in entities {
                set.insert(entity);
            }
            entities_list.push(set);
        }

        entities_list
    }

    fn generate_decoded_entities() -> Vec<BTreeSet<DecodedEntity>> {
        generate_entities().into_iter().map(|entries| {
            entries.into_iter().map(|e| { e.decode() } ).collect()
        }).collect()
    }

    #[test]
    fn correctness_chars_2() {
        let result = "Attend \u{20000}\u{20000} hear 6 stellar <#mobile> <#startups> at <#OF12> Entrepreneur Idol show 2day,  <http://t.co/HtzEMgAC> <@TiEcon> <@sv_entrepreneur> <@500>!";
        assert_eq!(result, classic_chars_2(&UNICODE_TEXT.chars().collect(), &entities()))
    }

    #[test]
    fn correctness_chars() {
        let result = "Attend \u{20000}\u{20000} hear 6 stellar <#mobile> <#startups> at <#OF12> Entrepreneur Idol show 2day,  <http://t.co/HtzEMgAC> <@TiEcon> <@sv_entrepreneur> <@500>!";
        assert_eq!(result, classic_chars(&UNICODE_TEXT.chars().collect(), &decoded_entities()))
    }

    #[test]
    fn correctness() {
        let result = "Attend \u{20000}\u{20000} hear 6 stellar <#mobile> <#startups> at <#OF12> Entrepreneur Idol show 2day,  <http://t.co/HtzEMgAC> <@TiEcon> <@sv_entrepreneur> <@500>!";
        assert_eq!(result, classic(&UNICODE_TEXT, &entities()))
    }

    #[bench]
    fn bench_replacement(b: &mut Bencher) {
        let entities_list = generate_entities();
        let mut index_iter = (0..1000).into_iter().cycle();
        b.iter(|| {
            classic(UNICODE_TEXT, &entities_list[index_iter.next().unwrap()])
        });
    }

    #[bench]
    fn bench_replacement_chars(b: &mut Bencher) {
        let entities_list = generate_decoded_entities();
        let mut index_iter = (0..1000).into_iter().cycle();
        let decoded_text = UNICODE_TEXT.chars().collect();
        b.iter(|| {
            let option = index_iter.next();
            classic_chars(&decoded_text, &entities_list[option.unwrap()])
        });
    }

    #[bench]
    fn bench_replacement_chars_2(b: &mut Bencher) {
        let entities_list = generate_entities();
        let mut index_iter = (0..1000).into_iter().cycle();
        let decoded_text = UNICODE_TEXT.chars().collect();
        b.iter(|| {
            let option = index_iter.next();
            classic_chars_2(&decoded_text, &entities_list[option.unwrap()])
        });
    }
}

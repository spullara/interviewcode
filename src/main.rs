#![feature(convert, test, core, core_str_ext, vec_push_all)]
extern crate test;
extern crate core;

use core::str::StrExt;

pub static ASCII_TEXT: &'static str = "Attend to hear 6 stellar #mobile #startups at #OF12 Entrepreneur Idol show 2day,  http://t.co/HtzEMgAC @TiEcon @sv_entrepreneur @500!";
pub static UNICODE_TEXT: &'static str = "Attend 次次 hear 6 stellar #mobile #startups at #OF12 Entrepreneur Idol show 2day,  http://t.co/HtzEMgAC @TiEcon @sv_entrepreneur @500!";

#[derive(Clone)]
pub struct Entity {
    start: usize,
    end: usize,
    html: String
}

#[derive(Clone)]
pub struct DecodedEntity {
    start: usize,
    end: usize,
    html: Vec<char>,
}

unsafe fn render_ascii(text: &str, entities: &mut Vec<Entity>) -> String {
    let mut sb = String::new();
    entities.sort_by(|e1, e2| e1.start.cmp(&e2.start) );

    let mut pos = 0 as usize;
    for entity in entities {
        sb.push_str(text.slice_unchecked(pos, entity.start));
        sb.push_str(entity.html.as_str());
        pos = entity.end;
    }
    sb.push_str(text.slice_unchecked(pos, text.len()));
    sb
}

fn render(text: &Vec<char>, entities: &mut Vec<DecodedEntity>) -> String {
    // Initial capacity based on observation that entities tend to add just 2 chars
    let mut sb: Vec<char> = Vec::with_capacity(text.len()+entities.len()*2);
    entities.sort_by(|e1, e2| e1.start.cmp(&e2.start) );

    let mut pos = 0 as usize;
    for entity in entities {
        sb.push_all(&text[pos..entity.start]);
        sb.push_all(&*entity.html);
        pos = entity.end;
    }
    sb.push_all(&text[pos..text.len()]);
    sb.into_iter().collect()
}


fn main() {
    let result = classic(&ASCII_TEXT.chars().collect(), &mut decoded_entities());
    println!("Result: {}", result);
}


pub fn classic_ascii(text: &str, entities: &mut Vec<Entity>) -> String {
    unsafe {
        render_ascii(text, entities)
    }
}

pub fn classic(text: &Vec<char>, entities: &mut Vec<DecodedEntity>) -> String {
    render(&text, entities)
}

pub fn entities() -> Vec<Entity> {
    let entities = vec![
    Entity {start: 82, end: 102, html:"<http://t.co/HtzEMgAC>".to_string()},
    Entity {start: 128, end: 132, html:"<@500>".to_string()},
    Entity {start: 25, end: 32, html:"<#mobile>".to_string()},
    Entity {start: 33, end: 42, html:"<#startups>".to_string()},
    Entity {start: 111, end: 127, html:"<@sv_entrepreneur>".to_string()},
    Entity {start: 46, end: 51, html:"<#OF12>".to_string()},
    Entity {start: 103, end: 110, html:"<@TiEcon>".to_string()},
    ];
    entities
}

pub fn decoded_entities() -> Vec<DecodedEntity> {
    entities().into_iter().map( |e: Entity|
        DecodedEntity {start: e.start, end: e.end, html: e.html.chars().collect()}
    ).collect()
}

#[cfg(test)]
mod rendertest {
    use super::*;
    use test::Bencher;

    #[test]
    fn correctness_ascii() {
        let result = "Attend to hear 6 stellar <#mobile> <#startups> at <#OF12> Entrepreneur Idol show 2day,  <http://t.co/HtzEMgAC> <@TiEcon> <@sv_entrepreneur> <@500>!";
        assert_eq!(result, classic_ascii(ASCII_TEXT, &mut entities()))
    }

    #[test]
    fn correctness() {
        let result = "Attend 次次 hear 6 stellar <#mobile> <#startups> at <#OF12> Entrepreneur Idol show 2day,  <http://t.co/HtzEMgAC> <@TiEcon> <@sv_entrepreneur> <@500>!";
        assert_eq!(result, classic(&UNICODE_TEXT.chars().collect(), &mut decoded_entities()))
    }

    #[bench]
    fn bench_replacement_ascii(b: &mut Bencher) {
        let entities = &mut entities();
        b.iter(|| {
            classic_ascii(ASCII_TEXT, entities)
        });
    }

    #[bench]
    fn bench_replacement(b: &mut Bencher) {
        let entities = &mut decoded_entities();
        let decoded_text = UNICODE_TEXT.chars().collect();
        b.iter(|| {
            classic(&decoded_text, entities)
        });
    }
}

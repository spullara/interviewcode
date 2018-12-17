#![cfg_attr(test, feature(test))]

pub static ASCII_TEXT: &'static str = "Attend to hear 6 stellar #mobile #startups at #OF12 Entrepreneur Idol show 2day,  http://t.co/HtzEMgAC @TiEcon @sv_entrepreneur @500!";
pub static UNICODE_TEXT: &'static str = "Attend \u{20000}\u{20000} hear 6 stellar #mobile #startups at #OF12 Entrepreneur Idol show 2day,  http://t.co/HtzEMgAC @TiEcon @sv_entrepreneur @500!";

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

impl Entity {
    fn decode(&self) -> DecodedEntity {
        DecodedEntity {start: self.start, end: self.end, html: self.html.chars().collect()}
    }
}

fn render(text: &str, entities: &mut Vec<Entity>) -> String {
    let mut sb = String::with_capacity(text.len()*2);
    entities.sort_by(|e1, e2| e1.start.cmp(&e2.start) );

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

fn render_chars(text: &Vec<char>, entities: &mut Vec<DecodedEntity>) -> String {
    let mut sb: Vec<char> = Vec::with_capacity(text.len()*2);
    entities.sort_by(|e1, e2| e1.start.cmp(&e2.start) );

    let mut pos = 0 as usize;
    for entity in entities {
        sb.extend_from_slice(&text[pos..entity.start]);
        sb.extend_from_slice(&entity.html);
        pos = entity.end;
    }
    sb.extend_from_slice(&text[pos..text.len()]);
    sb.into_iter().collect()
}


fn main() {
    let result = classic(&ASCII_TEXT, &mut entities());
    println!("Result: {}", result);
}


pub fn classic(text: &str, entities: &mut Vec<Entity>) -> String {
    render(&text, entities)
}

pub fn classic_chars(text: &Vec<char>, entities: &mut Vec<DecodedEntity>) -> String {
    render_chars(&text, entities)
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
    entities().into_iter().map( |e| e.decode() ).collect()
}


#[cfg(test)] extern crate rand;
#[cfg(test)] extern crate test;

#[cfg(test)]
mod rendertest {
    use super::*;
    use rand::{self,Rng};
    use test::Bencher;

    fn generate_entities() -> Vec<Vec<Entity>> {
        let mut rng = rand::thread_rng();
        let mut entities_list: Vec<Vec<Entity>> = Vec::with_capacity(1000);

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
            entities_list.push(entities.collect());
        }

        entities_list
    }

    fn generate_decoded_entities() -> Vec<Vec<DecodedEntity>> {
        generate_entities().into_iter().map(|entries| {
            entries.into_iter().map(|e| { e.decode() } ).collect()
        }).collect()
    }

    #[test]
    fn correctness_chars() {
        let result = "Attend \u{20000}\u{20000} hear 6 stellar <#mobile> <#startups> at <#OF12> Entrepreneur Idol show 2day,  <http://t.co/HtzEMgAC> <@TiEcon> <@sv_entrepreneur> <@500>!";
        assert_eq!(result, classic_chars(&UNICODE_TEXT.chars().collect(), &mut decoded_entities()))
    }

    #[test]
    fn correctness() {
        let result = "Attend \u{20000}\u{20000} hear 6 stellar <#mobile> <#startups> at <#OF12> Entrepreneur Idol show 2day,  <http://t.co/HtzEMgAC> <@TiEcon> <@sv_entrepreneur> <@500>!";
        assert_eq!(result, classic(&UNICODE_TEXT, &mut entities()))
    }

    #[bench]
    fn bench_replacement(b: &mut Bencher) {
        let mut entities_list = generate_entities();
        let mut index_iter = (0..1000).into_iter().cycle();
        b.iter(|| {
            classic(UNICODE_TEXT, &mut entities_list[index_iter.next().unwrap()])
        });
    }

    #[bench]
    fn bench_replacement_chars(b: &mut Bencher) {
        let mut entities_list = generate_decoded_entities();
        let mut index_iter = (0..1000).into_iter().cycle();
        let decoded_text = UNICODE_TEXT.chars().collect();
        b.iter(|| {
            let option = index_iter.next();
            classic_chars(&decoded_text, &mut entities_list[option.unwrap()])
        });
    }
}

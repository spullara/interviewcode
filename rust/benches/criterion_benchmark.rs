#[macro_use]
extern crate criterion;
use criterion::Criterion;

extern crate interviewcode;
use interviewcode::{
    decoded_entities, entity_refs, render, render_chars, render_chars2,
    render_chars_entity_references, render_chars_entity_references_to_chars, render_coords,
    DecodedEntity, Entity, ASCII_TEXT, UNICODE_TEXT,
};

extern crate rand;
use rand::Rng;

fn generate_entities() -> Vec<Vec<Entity<String>>> {
    let mut rng = rand::thread_rng();
    let mut entities_list: Vec<Vec<Entity<String>>> = Vec::with_capacity(1000);

    for _ in 0..1000 {
        let total = rng.gen::<usize>() % 10;
        let mut indices = Vec::with_capacity(total);
        for _ in 0..(total * 2) {
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
            Entity {
                start: start,
                end: end,
                html: (0..length).map(|_| "X").collect(),
            }
        });
        entities_list.push(entities.collect());
    }

    entities_list
}

fn generate_decoded_entities() -> Vec<Vec<DecodedEntity>> {
    generate_entities()
        .into_iter()
        .map(|entries| decoded_entities(entries))
        .collect()
}

// Actual benchmarks start here

fn bench_replacement(c: &mut Criterion) {
    let entities_list = generate_entities();
    let mut index_iter = (0..1000).into_iter().cycle();
    c.bench_function("replacement", move |b| {
        b.iter(|| render(UNICODE_TEXT, &entities_list[index_iter.next().unwrap()]))
    });
}

fn bench_replacement_chars(c: &mut Criterion) {
    let entities_list = generate_decoded_entities();
    let mut index_iter = (0..1000).into_iter().cycle();
    let decoded_text = UNICODE_TEXT.chars().collect();
    c.bench_function("replacement chars", move |b| {
        b.iter(|| {
            let option = index_iter.next();
            render_chars(&decoded_text, &entities_list[option.unwrap()])
        })
    });
}

fn bench_replacement_chars2(c: &mut Criterion) {
    let entities_list = generate_entities();
    let mut index_iter = (0..1000).into_iter().cycle();
    let decoded_text = UNICODE_TEXT.chars().collect();
    c.bench_function("replacement chars 2", move |b| {
        b.iter(|| {
            let option = index_iter.next();
            render_chars2(&decoded_text, &entities_list[option.unwrap()])
        })
    });
}

fn bench_replacement_chars_entity_references(c: &mut Criterion) {
    c.bench_function("replacement chars entity references", move |b| {
        let entities_list = generate_entities();
        let mut refs = Vec::with_capacity(1000);
        for (i, _) in entities_list.iter().enumerate() {
            refs.push(entity_refs(&entities_list[i]));
        }
        let mut index_iter = (0..1000).into_iter().cycle();
        let decoded_text = UNICODE_TEXT.chars().collect();
        b.iter(|| {
            let option = index_iter.next();
            render_chars_entity_references(&decoded_text, &refs[option.unwrap()])
        })
    });
}

fn bench_replacement_chars_entity_references_to_chars(c: &mut Criterion) {
    c.bench_function("replacement chars entity references to chars", move |b| {
        let entities_list = generate_decoded_entities();
        let mut refs = Vec::with_capacity(1000);
        let mut index_iter = (0..1000).into_iter().cycle();
        let decoded_text = UNICODE_TEXT.chars().collect();
        for (i, _) in entities_list.iter().enumerate() {
            refs.push(entity_refs(&entities_list[i]));
        }
        b.iter(|| {
            let option = index_iter.next();
            render_chars_entity_references_to_chars(&decoded_text, &refs[option.unwrap()])
        })
    });
}

// Benchmark only sorting entities and determining substitutions.
fn bench_render_coords(c: &mut Criterion) {
    c.bench_function("render coords", move |b| {
        let entities_list = generate_decoded_entities();
        let mut refs = Vec::with_capacity(1000);
        for (i, _) in entities_list.iter().enumerate() {
            refs.push(entity_refs(&entities_list[i]));
        }
        let mut index_iter = (0..1000).into_iter().cycle();
        let decoded_text = UNICODE_TEXT.chars().collect();
        let mut ht = Vec::with_capacity(64);

        b.iter(|| {
            let option = index_iter.next();
            ht.clear();
            // Sort entities
            let refs = &refs[option.unwrap()];
            let mut sorted: Vec<&DecodedEntity> = Vec::with_capacity(refs.len());
            for e in refs {
                sorted.push(e);
            }
            sorted.sort_unstable();
            render_coords(&mut ht, &decoded_text, &sorted);
        })
    });
}

criterion_group!(
    benches,
    bench_replacement,
    bench_replacement_chars,
    bench_replacement_chars2,
    bench_replacement_chars_entity_references,
    bench_replacement_chars_entity_references_to_chars,
    bench_render_coords
);
criterion_main!(benches);

extern crate jemallocator;
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[macro_use]
extern crate criterion;

extern crate unicode_test;
use unicode_test::{
    bench_render_coords, bench_replacement, bench_replacement_chars, bench_replacement_chars2,
    bench_replacement_chars_entity_references, bench_replacement_chars_entity_references_to_chars,
};

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

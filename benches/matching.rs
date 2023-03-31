use criterion::*;
use wordqat::pattern::Pattern;

fn basic_dot_benchmark(c: &mut Criterion) {
    let wordlist = ["lone", "love", "word", "door", "dome", "lint", "leftie"];
    let pattern = Pattern::new("l..e").unwrap();
    let mut i = 0;
    c.bench_function("dot-1", |b| {
        b.iter(|| {
            black_box(pattern.matches(wordlist[i]));
            i = (i + 1) % wordlist.len();
        })
    });
}

fn basic_set_benchmark(c: &mut Criterion) {
    let wordlist = ["anise", "avize", "alone", "elide", "risen"];
    let pattern = Pattern::new("..i[sz]e").unwrap();
    let mut i = 0;
    c.bench_function("set-1", |b| {
        b.iter(|| {
            black_box(pattern.matches(wordlist[i]));
            i = (i + 1) % wordlist.len();
        })
    });
}

criterion_group!(benches, basic_dot_benchmark, basic_set_benchmark);
criterion_main!(benches);

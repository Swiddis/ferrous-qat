use criterion::*;
use wordqat::SimplePattern;

fn basic_dot_benchmark(c: &mut Criterion) {
    let wordlist = ["lone", "love", "word", "door", "dome", "lint", "leftie"];
    let pattern = SimplePattern::try_from("l..e").unwrap();
    let mut i = 0;
    c.bench_function("dot-1", |b| {
        b.iter(|| {
            black_box(pattern.is_match(wordlist[i]));
            i = (i + 1) % wordlist.len();
        })
    });
}

fn basic_set_benchmark(c: &mut Criterion) {
    let wordlist = ["anise", "avize", "alone", "elide", "risen"];
    let pattern = SimplePattern::try_from("..i[sz]e").unwrap();
    let mut i = 0;
    c.bench_function("set-1", |b| {
        b.iter(|| {
            black_box(pattern.is_match(wordlist[i]));
            i = (i + 1) % wordlist.len();
        })
    });
}

criterion_group!(benches, basic_dot_benchmark, basic_set_benchmark);
criterion_main!(benches);

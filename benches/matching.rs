use criterion::*;
use wordqat::Pattern;

fn letter_dot_benchmark(c: &mut Criterion) {
    let wordlist = ["lone", "love", "word", "door", "dome", "lint", "leftie"];
    let mut i = 0;
    let pattern = Pattern::new("l..e");
    c.bench_function("letter-dot-1", |b| b.iter(|| {
        pattern.matches(black_box(wordlist[i]));
        i = (i + 1) % wordlist.len();
    }));
}

criterion_group!(benches, letter_dot_benchmark);
criterion_main!(benches);

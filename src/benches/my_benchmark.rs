#[macro_use]
extern crate criterion;

use criterion::Criterion;
use criterion::black_box;
use std::time::Duration;
use std::collections::HashMap;

use rayon::iter::{IntoParallelRefIterator, IntoParallelRefMutIterator};
use rayon::iter::IndexedParallelIterator;
use rayon::iter::ParallelIterator;

use std::sync::Arc;

//const T: &str = "ABAABA$";
// Will Arc::copy(&file_contents) work here?
//const testing_contents: String = common::suffix_array::read_file("/Users/bxa005/masters/701/Project/rust/bwt/src/1_000_000_dna");
//const testing: &str = testing_contents.as_str();

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

fn sa_benchmark(c: &mut Criterion) {
    let file_contents: String = common::suffix_array::read_file("/Users/bxa005/masters/701/Project/rust/bwt/src/1_000_000_dna");
    let mut bench_strings = HashMap::new();
    bench_strings.insert("contents", file_contents);

    c.bench_function("simple sa generation",
                     move |b| b.iter(|| {
                         let foo = &bench_strings["contents"];
                         common::suffix_array::suffix_array(black_box(&foo))
                     })
    );
}
/*
fn lf_benchmark(c: &mut Criterion) {
    let file_contents: String = common::suffix_array::read_file("/Users/bxa005/masters/701/Project/rust/bwt/src/1_000_000_dna");
    let mut bench_strings = HashMap::new();
    bench_strings.insert("contents", file_contents);

    let foo = &bench_strings["contents"];
    let sa = common::suffix_array::suffix_array(&foo);
    c.bench_function("simple lf generation",
                     move |b| b.iter(|| {
                         let foo = "TEST";
                         common::suffix_array::construct_lf(black_box(&foo), black_box(&sa))
                     })
    );
}
*/
fn reverse_benchmark(c: &mut Criterion) {
    let file_contents: String = common::suffix_array::read_file("/Users/bxa005/masters/701/Project/rust/bwt/src/1_000_000_dna");
    let T = file_contents.as_str();

    let sa = common::suffix_array::suffix_array(&T);
    let lf = common::suffix_array::construct_lf(&T, &sa);
    c.bench_function("simple reverse",
                     move |b| b.iter(||
                         common::suffix_array::reverse_bwt(black_box(&lf.Index))
                     )
    );
}


fn search_benchmark(c: &mut Criterion) {
    let file_contents: String = common::suffix_array::read_file("/Users/bxa005/masters/701/Project/rust/bwt/src/1_000_000_dna");
    let T = file_contents.as_str();

    let sa: Vec<(usize, &[u8])> = common::suffix_array::suffix_array(&T);
    let lf: common::suffix_array::LF = common::suffix_array::construct_lf(&T, &sa);
    c.bench_function("simple search",
                     move |b| {
                         b.iter(|| {
                             let zipped: Vec<(&(char, usize), &(char, usize))> = lf.F.par_iter().zip(lf.L.par_iter()).collect();

                             common::search::search(black_box("ABA"),
                                                    black_box(&zipped),
                                                    black_box(&lf.FCounts)
                             )}
                         )
                     }
    );
}

fn parallel_search_benchmark(c: &mut Criterion) {
    let file_contents: String = common::suffix_array::read_file("/Users/bxa005/masters/701/Project/rust/bwt/src/1_000_000_dna");
    let mut bench_strings = HashMap::new();
    bench_strings.insert("contents", file_contents);

    c.bench_function("simple sa generation",
                     move |b| b.iter(|| {
                         let foo = &bench_strings["contents"];
                         common::suffix_array::suffix_array(black_box(&foo))
                     })
    );
}

fn sa_benchmark(c: &mut Criterion) {
    let file_contents: String = common::suffix_array::read_file("/Users/bxa005/masters/701/Project/rust/bwt/src/1_000_000_dna");
    let mut bench_strings = HashMap::new();
    bench_strings.insert("contents", file_contents);

    c.bench_function("simple sa generation",
                     move |b| b.iter(|| {
                         let foo = &bench_strings["contents"];
                         common::suffix_array::suffix_array(black_box(&foo))
                     })
    );
}

criterion_group!(name = benches;
                config = Criterion::default().sample_size(500).measurement_time(Duration::new(10, 0));
                targets = search_benchmark);
criterion_main!(benches);


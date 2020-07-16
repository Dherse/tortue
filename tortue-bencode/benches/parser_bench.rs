use criterion::{
    black_box, criterion_group, criterion_main, Criterion, Throughput,
};
use tortue_bencode::parser::parse;

const DATA: &[u8] = include_bytes!("test_data");

pub fn throughput_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("read_throughput");
    group.throughput(Throughput::Bytes(DATA.len() as u64));
    group.bench_function("test_data", |b| b.iter(|| parse(black_box(DATA))));
}

criterion_group!(benches, throughput_benchmark);
criterion_main!(benches);

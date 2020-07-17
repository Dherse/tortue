use criterion::{
    black_box, criterion_group, criterion_main, Criterion, Throughput,
};
use tortue_bencode::from_bytes;
use tortue_structs::Metainfo;

const DATA: &[u8] = include_bytes!("test_data");

pub fn throughput_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("deserialize");
    group.throughput(Throughput::Bytes(DATA.len() as u64));

    group.bench_function("metainfo", |b| {
        b.iter(|| black_box(from_bytes::<Metainfo>(black_box(&DATA))))
    });
}

criterion_group!(benches, throughput_benchmark);
criterion_main!(benches);

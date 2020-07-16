use criterion::{
    black_box, criterion_group, criterion_main, Criterion, Throughput,
};
use std::io::Cursor;
use tortue_bencode::{parser::parse, writer::write};

const DATA: &[u8] = include_bytes!("test_data");

pub fn throughput_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("write_throughput");
    group.throughput(Throughput::Bytes(DATA.len() as u64));
    let data = parse(DATA).unwrap().1;
    let mut out = Vec::with_capacity(DATA.len());
    group.bench_function("test_data", |b| {
        b.iter(|| {
            let mut cursor = Cursor::new(&mut out);
            write(black_box(&data), black_box(&mut cursor))
        })
    });
}

criterion_group!(benches, throughput_benchmark);
criterion_main!(benches);

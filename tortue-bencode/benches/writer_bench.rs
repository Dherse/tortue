use criterion::{
    black_box, criterion_group, criterion_main, Criterion, Throughput,
};
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use tortue_bencode::{
    from_bytes, from_value, parser::parse, to_value, to_writer, writer::write,
};

const DATA: &[u8] = include_bytes!("test_data");
#[derive(Deserialize, Serialize, Clone, Debug)]
struct Profile<'a> {
    acodec: &'a str,
    height: i64,
    vcodec: &'a str,
    width: i64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct Info<'a> {
    #[serde(rename = "file-duration")]
    file_duration: Vec<i64>,

    #[serde(rename = "file-media")]
    file_media: Vec<i64>,

    length: i64,

    name: &'a str,

    #[serde(rename = "piece length")]
    piece_length: i64,

    #[serde(with = "serde_bytes")]
    pieces: &'a [u8],

    profiles: Vec<Profile<'a>>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct Data<'a> {
    announce: &'a str,

    #[serde(rename = "announce-list")]
    announce_list: Vec<Vec<&'a str>>,

    comment: &'a str,

    #[serde(rename = "created by")]
    created_by: &'a str,

    #[serde(rename = "creation date")]
    creation_date: i64,

    encoding: &'a str,

    #[serde(rename = "url-list")]
    url_list: Vec<&'a str>,

    website: &'a str,

    info: Info<'a>,
}

pub fn throughput_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("write_throughput");
    group.throughput(Throughput::Bytes(DATA.len() as u64));
    let data = parse(DATA).unwrap().1;
    let mut out = Vec::with_capacity(DATA.len());

    group.bench_function("write_to_bytes", |b| {
        b.iter(|| {
            out.clear();
            write(black_box(&data), black_box(&mut out))
        })
    });

    let value: Data = from_value(data).unwrap();

    group.bench_function("serialize", |b| {
        b.iter(|| to_value(black_box(&value)))
    });

    group.bench_function("combined", |b| {
        b.iter(|| {
            out.clear();
            to_writer(black_box(&value), &mut out)
        })
    });
}

criterion_group!(benches, throughput_benchmark);
criterion_main!(benches);

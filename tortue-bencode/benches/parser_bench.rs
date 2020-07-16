use criterion::{
    black_box, criterion_group, criterion_main, Criterion, Throughput,
};
use serde::{Deserialize, Serialize};
use tortue_bencode::{from_bytes, parser::parse};

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
    let mut group = c.benchmark_group("read_throughput");
    group.throughput(Throughput::Bytes(DATA.len() as u64));

    group.bench_function("parse", |b| b.iter(|| parse(black_box(DATA))));

    group.bench_function("deserialize", |b| {
        b.iter(|| from_bytes::<Data>(&DATA))
    });
}

criterion_group!(benches, throughput_benchmark);
criterion_main!(benches);

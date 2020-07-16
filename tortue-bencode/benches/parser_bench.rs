use criterion::{
    black_box, criterion_group, criterion_main, Criterion, Throughput,
};
use serde::{Deserialize, Serialize};
use tortue_bencode::{from_bytes, parser::parse};

const DATA: &[u8] = include_bytes!("test_data");

#[derive(Deserialize, Serialize, Clone, Debug)]
struct Profile {
    acodec: String,
    height: i64,
    vcodec: String,
    width: i64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct Info {
    #[serde(rename = "file-duration")]
    file_duration: Vec<i64>,

    #[serde(rename = "file media")]
    file_media: Vec<i64>,

    length: i64,

    name: String,

    #[serde(rename = "piece length")]
    piece_length: i64,

    pieces: Vec<u8>,

    profiles: Vec<Profile>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct Data {
    announce: String,

    #[serde(rename = "announce-list")]
    announce_list: Vec<Vec<String>>,

    comment: String,

    #[serde(rename = "created by")]
    created_by: String,

    #[serde(rename = "creation date")]
    creation_date: i64,

    encoding: String,

    #[serde(rename = "url list")]
    url_list: Vec<String>,

    website: String,

    info: Info,
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

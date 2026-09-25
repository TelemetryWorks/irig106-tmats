// benches/parse_bench.rs
//
// # Parse Benchmarks
//
// Criterion benchmarks for TMATS parsing, serialization, and validation.
//
// ## Traceability:
//   L3-PERF-007: Benchmark suite
//   L3-TEST-012: Benchmark regression gate
//
// Run: cargo bench --bench parse_bench

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

fn small_tmats() -> Vec<u8> {
    b"G\\PN:BENCH_SMALL;G\\106:17;G\\OD:01-15-2024;\
      R-1\\ID:REC;R-1\\N:1;R-1\\TK1-1:1;R-1\\CDT-1:09;R-1\\CDLN-1:PCM1;\
      P-1\\DLN:PCM1;P-1\\D1:1000000;P-1\\D2:NRZ-L;".to_vec()
}

fn medium_tmats() -> Vec<u8> {
    let mut buf = Vec::with_capacity(20_000);
    buf.extend_from_slice(b"G\\PN:BENCH_MEDIUM;G\\106:17;G\\OD:01-15-2024;G\\TN:FT-001;");

    let num_channels = 64;
    buf.extend_from_slice(format!("G\\DSI\\N:{num_channels};").as_bytes());
    for i in 1..=num_channels {
        buf.extend_from_slice(format!("G\\DSI-{i}:SRC_{i};G\\DST-{i}:REC;G\\DSC-{i}:U;").as_bytes());
    }

    buf.extend_from_slice(format!("R-1\\ID:MDR_1;R-1\\N:{num_channels};").as_bytes());
    for i in 1..=num_channels {
        buf.extend_from_slice(format!(
            "R-1\\TK1-{i}:{i};R-1\\CDT-{i}:09;R-1\\CDLN-{i}:LINK_{i};R-1\\PDP-{i}:UN;"
        ).as_bytes());
    }

    for i in 1..=num_channels {
        buf.extend_from_slice(format!(
            "P-{i}\\DLN:LINK_{i};P-{i}\\D1:{};P-{i}\\D2:NRZ-L;P-{i}\\F1:256;P-{i}\\F2:16;",
            1_000_000 + i * 100_000
        ).as_bytes());
    }

    buf
}

fn bench_parse(c: &mut Criterion) {
    let small = small_tmats();
    let medium = medium_tmats();

    let mut group = c.benchmark_group("parse");

    group.bench_with_input(
        BenchmarkId::new("small", small.len()),
        &small,
        |b, input| {
            b.iter(|| irig106_tmats::parse::parse(black_box(input)).unwrap())
        },
    );

    group.bench_with_input(
        BenchmarkId::new("medium", medium.len()),
        &medium,
        |b, input| {
            b.iter(|| irig106_tmats::parse::parse(black_box(input)).unwrap())
        },
    );

    group.finish();
}

fn bench_parse_owned(c: &mut Criterion) {
    let medium = medium_tmats();

    c.bench_with_input(
        BenchmarkId::new("parse_owned/medium", medium.len()),
        &medium,
        |b, input| {
            b.iter(|| irig106_tmats::parse::parse_owned(black_box(input)).unwrap())
        },
    );
}

fn bench_serialize(c: &mut Criterion) {
    let medium = medium_tmats();
    let doc = irig106_tmats::parse::parse(&medium).unwrap();

    c.bench_function("serialize/medium", |b| {
        b.iter(|| irig106_tmats::serial::serialize_to_vec(black_box(&doc)).unwrap())
    });
}

fn bench_round_trip(c: &mut Criterion) {
    let medium = medium_tmats();

    c.bench_with_input(
        BenchmarkId::new("round_trip/medium", medium.len()),
        &medium,
        |b, input| {
            b.iter(|| {
                let doc = irig106_tmats::parse::parse(black_box(input)).unwrap();
                let _out = irig106_tmats::serial::serialize_to_vec(&doc).unwrap();
            })
        },
    );
}

fn bench_validate(c: &mut Criterion) {
    let medium = medium_tmats();
    let doc = irig106_tmats::parse::parse(&medium).unwrap();

    c.bench_function("validate/medium", |b| {
        b.iter(|| irig106_tmats::validate::validate(black_box(&doc)))
    });
}

fn bench_ch10_payload(c: &mut Criterion) {
    let medium = medium_tmats();
    let doc = irig106_tmats::parse::parse(&medium).unwrap();
    let csdw = irig106_tmats::ch10::SetupRecordCsdw::from_version(
        irig106_tmats::types_bridge::Irig106Version::V106_17,
        false,
    );

    c.bench_function("encode_payload/medium", |b| {
        b.iter(|| {
            irig106_tmats::ch10::encode_setup_payload(black_box(&doc), &csdw).unwrap()
        })
    });

    let payload = irig106_tmats::ch10::encode_setup_payload(&doc, &csdw).unwrap();
    let bytes = payload.to_bytes();

    c.bench_with_input(
        BenchmarkId::new("decode_payload/medium", bytes.len()),
        &bytes,
        |b, input| {
            b.iter(|| irig106_tmats::ch10::decode_setup_payload(black_box(input)).unwrap())
        },
    );
}

fn bench_query(c: &mut Criterion) {
    let medium = medium_tmats();
    let doc = irig106_tmats::parse::parse(&medium).unwrap();

    c.bench_function("resolve_channel", |b| {
        b.iter(|| irig106_tmats::query::resolve_channel(black_box(&doc), 32))
    });
}

criterion_group!(
    benches,
    bench_parse,
    bench_parse_owned,
    bench_serialize,
    bench_round_trip,
    bench_validate,
    bench_ch10_payload,
    bench_query,
);
criterion_main!(benches);

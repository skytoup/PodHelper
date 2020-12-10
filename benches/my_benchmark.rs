use std::{fs, io::BufReader};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pod_helper::utils::parse_podfile_lock_podspec;

fn bench_parse_podfile_lock_podspec(c: &mut Criterion) {
    c.bench_function("parse_podfile_lock_podspec podfile.lock", |b| {
        b.iter(|| {
            let f = fs::File::open("Podfile.lock").unwrap();
            let br = BufReader::new(f);
            let _ = parse_podfile_lock_podspec(black_box(br));
        });
    });
}

criterion_group!(benches, bench_parse_podfile_lock_podspec);
criterion_main!(benches);

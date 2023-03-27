use criterion::{
    black_box,
    criterion_group, criterion_main,
    Criterion, BenchmarkId, Throughput
};

use rand;
use pqds::stribog::{stribog, Stribog, HashSize};

fn generate_message(size: usize) -> Vec<u8> {
    let vec: Vec<u8> = (0..size).into_iter()
        .map(|_| rand::random::<u8>())
        .collect();
    vec
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let id = 's';
    let m_s = [
        generate_message(0), generate_message(1), generate_message(10),
        generate_message(100), generate_message(1000), generate_message(10000)];

    //HASH 256
    {
        let mut group = c.benchmark_group("stribog_256");
        for (i, item) in m_s.iter().enumerate() {
            let id = id.to_string() + i.to_string().as_str();
            group.throughput(Throughput::Bytes(item.len() as u64));
            group.bench_with_input(BenchmarkId::from_parameter(id), &item, |b, &item| {
                b.iter(|| stribog(black_box(&mut Stribog::new(HashSize::L256)), black_box(&item), black_box(item.len())));
            });
        }
        group.finish();
    }
    // HASH 512
    {
        let mut group = c.benchmark_group("stribog_512");
        for (i, item) in m_s.iter().enumerate() {
            let id = id.to_string() + i.to_string().as_str();
            group.throughput(Throughput::Bytes(item.len() as u64));
            group.bench_with_input(BenchmarkId::from_parameter(id), &item, |b, &item| {
                b.iter(|| stribog(black_box(&mut Stribog::new(HashSize::L512)), black_box(&item), black_box(item.len())));
            });
        }
        group.finish();
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
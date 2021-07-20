use criterion::{criterion_group, criterion_main, BatchSize, Criterion, Throughput};
use hidapi::HidApi;
use rand::Rng;

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let api = HidApi::new().unwrap();
    let mut groupe = c.benchmark_group("hid_api");
    groupe.throughput(Throughput::Elements(1_u64));

    let devices = api.device_list().collect::<Vec<_>>();
    groupe.bench_function("open device", |b| {
        b.iter_batched(
            || devices[rng.gen_range(0..devices.len())],
            |device| device.open_device(&api).unwrap(),
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

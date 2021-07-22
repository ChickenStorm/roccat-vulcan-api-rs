use criterion::{criterion_group, criterion_main, BatchSize, Criterion, Throughput};
use hidapi::HidApi;
use rand::Rng;
use roccat_vulcan_api_rs::KeyboardApi;

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
    groupe.finish();

    let mut groupe_api = c.benchmark_group("keyboard_api");
    groupe_api.bench_function("Create api", |b| b.iter(|| KeyboardApi::new().unwrap()));

    groupe_api.finish()
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

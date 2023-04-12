use criterion::{criterion_group, criterion_main, Criterion};
use qassign::queue::FIFOQueue;
use qassign::queue::Queue;

fn queue_benchmark(c: &mut Criterion) {
    c.bench_function("queue 3000", |b| {
        b.iter(|| {
            let mut q = FIFOQueue::new();
            let mut r = FIFOQueue::new();
            for i in 0..3000 {
                q.add(i);
            }
            q.dump(&mut r);
        })
    });
}

criterion_group!(benches, queue_benchmark);
criterion_main!(benches);

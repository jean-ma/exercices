use std::collections::VecDeque;
use std::collections::LinkedList;

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use pprof::criterion::Output;
use pprof::criterion::PProfProfiler;
use queues::immutable::queue::Queue;
use queues::mutable::queue::Queue as MutQueue;

use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

pub fn mutable_benchmark(c: &mut Criterion) {
    let mut rng = ChaCha8Rng::seed_from_u64(2);
    c.bench_function("mutable queue", |b| {
        b.iter(|| mut_queue_op(black_box(&mut rng)))
    });
}

pub fn mutable_vecdequeue_benchmark(c: &mut Criterion) {
    let mut rng = ChaCha8Rng::seed_from_u64(2);
    c.bench_function("mutable vecdequeue queue", |b| {
        b.iter(|| mut_vecdequeue_queue_op(black_box(&mut rng)))
    });
}

pub fn mutable_linkedlist_benchmark(c: &mut Criterion) {
    let mut rng = ChaCha8Rng::seed_from_u64(2);
    c.bench_function("mutable linkedlist queue", |b| {
        b.iter(|| mut_linkedlist_queue_op(black_box(&mut rng)))
    });
}

pub fn immutable_benchmark(c: &mut Criterion) {
    let mut rng = ChaCha8Rng::seed_from_u64(2);
    c.bench_function("immutable queue", |b| {
        b.iter(|| queue_op(black_box(&mut rng)))
    });
}

fn mut_queue_op(rng: &mut ChaCha8Rng) {
    let mut q = MutQueue::<i32>::new();
    rng.set_word_pos(0);
    for _ in 1..10000 {
        let next = rng.random();
        if next % 2 == 0 {
            q.enqueue(next);
        } else {
            q.dequeue();
        }
    }
}

fn mut_vecdequeue_queue_op(rng: &mut ChaCha8Rng) {
    let mut q = VecDeque::<i32>::new();
    rng.set_word_pos(0);
    for _ in 1..10000 {
        let next = rng.random();
        if next % 2 == 0 {
            q.push_back(next);
        } else {
            q.pop_front();
        }
    }
}

fn mut_linkedlist_queue_op(rng: &mut ChaCha8Rng) {
    let mut q = LinkedList::<i32>::new();
    rng.set_word_pos(0);
    for _ in 1..10000 {
        let next = rng.random();
        if next % 2 == 0 {
            q.push_back(next);
        } else {
            q.pop_front();
        }
    }
}

fn queue_op(rng: &mut ChaCha8Rng) {
    let mut queue = Queue::<i32>::new();
    rng.set_word_pos(0);
    for _ in 1..10000 {
        let next = rng.random();
        if next % 2 == 0 {
            queue = queue.enqueue(next);
        } else {
            (_, queue) = queue.dequeue();
        }
    }
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = mutable_benchmark, immutable_benchmark, mutable_vecdequeue_benchmark, mutable_linkedlist_benchmark
}
criterion_main!(benches);

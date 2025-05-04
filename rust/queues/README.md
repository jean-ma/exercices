# Lock Free queues

Implement some lock free queues from scratch.

## Run the benchmark

```sh
cargo bench
```

Console output:

```command

mutable queue           time:   [167.67 µs 168.37 µs 169.25 µs]
                        change: [-2.6844% -1.7240% -0.9126%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) high mild
  3 (3.00%) high severe

immutable queue         time:   [203.67 µs 203.88 µs 204.11 µs]
                        change: [-5.1550% -4.6259% -4.1858%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) high mild
  4 (4.00%) high severe

mutable vecdequeue queue
                        time:   [45.458 µs 45.481 µs 45.503 µs]
                        change: [-0.8191% -0.0585% +0.6913%] (p = 0.90 > 0.05)
                        No change in performance detected.
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild
  7 (7.00%) high severe

mutable linkedlist queue
                        time:   [129.12 µs 129.77 µs 130.47 µs]
                        change: [-1.2771% -0.7485% -0.1569%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 21 outliers among 100 measurements (21.00%)
  8 (8.00%) high mild
  13 (13.00%) high severe
```

Detailed reports as html in ./target/criterion

## CPU flame graph

It doesn't work without `--bench single_threaded`:

```sh
cargo bench --bench single_threaded -- --profile-time 5
```

See for example for the immutable_queue:
![Immutable queue Flame Graph](./doc/immutable_queue_flamegraph.svg)

We see that the greatest share of cpu time is taken by the dequeue method, in particular the free and malloc methods. Although they explain only half of the cpu usage.

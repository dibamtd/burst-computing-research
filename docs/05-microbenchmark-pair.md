# Pair Microbenchmark (BCM + RabbitMQ)

## Goal

The goal of the **pair microbenchmark** is to validate **point-to-point communication**
using the Burst Communication Middleware (BCM) when workers communicate **indirectly via a backend**.

This benchmark goes one step beyond a hello-world example by:

- using the **official microbenchmark implementation** provided with BCM
- measuring **execution time and throughput**
- validating **group execution and synchronization semantics**

---

## Execution Setup

### Environment

- Linux (WSL / Ubuntu)
- Rust toolchain
- Docker Desktop
- RabbitMQ backend

RabbitMQ is used as the **remote communication backend**, simulating environments
where direct worker-to-worker connections are not available (e.g., serverless or cloud settings).

---

## Running the Pair Benchmark

The benchmark is executed using **two terminals**, each representing one worker group.

### Worker 0 (group_id = 0, receiver)

```bash
RUST_LOG=info cargo run --release -- \
  --benchmark pair \
  --burst-size 2 \
  --group-id 0 \
  --server "amqp://guest:guest@127.0.0.1:5672" \
  rabbitmq
````

### Worker 1 (group_id = 1, sender)

```bash
RUST_LOG=info cargo run --release -- \
  --benchmark pair \
  --burst-size 2 \
  --group-id 1 \
  --server "amqp://guest:guest@127.0.0.1:5672" \
  rabbitmq
```

Each worker connects to the same RabbitMQ server but uses a different `group-id`.
Running two terminals with `group-id=0` and `group-id=1` emulates two workers
participating in the same burst context.

---

## Observed Behavior

During execution:

* Both workers start as part of the same burst.
* `group_id=1` sends a **1 MB payload** to `group_id=0`.
* `group_id=0` receives the payload via the RabbitMQ backend.
* Communication occurs **exclusively through BCM**, not via direct sockets.
* The benchmark reports per-worker throughput and an aggregated throughput value.

This confirms that:

* group execution works correctly
* point-to-point communication is correctly routed through the backend
* synchronization between workers is preserved

---

## Summary of Results

| Role     | Payload | Time (s) | Throughput (MB/s) |
| -------- | ------- | -------- | ----------------- |
| Sender   | 1 MB    | ~0.007   | ~143              |
| Receiver | 1 MB    | ~0.001   | ~1000             |

The difference between sender and receiver throughput is expected:
the sender includes serialization and backend enqueue costs,
while the receiver measures delivery time once the message becomes available.

---

## Execution Proof (Local Run)

### Terminal A (group_id = 0, receiver)

```text
2026-01-02T20:28:59.509821Z  INFO benchmark: Arguments { benchmark: Pair, backend: Rabbitmq, ... }
2026-01-02T20:28:59.510425Z  INFO benchmark: Running Pair benchmark
2026-01-02T20:28:59.598967Z  INFO benchmark: start: 1767385739.598
2026-01-02T20:28:59.607343Z  INFO benchmark::pair: Worker 0 - started receiving from 1
2026-01-02T20:28:59.608963Z  INFO benchmark::pair: Worker 0 - received 1 MB in 0.000999 s
2026-01-02T20:28:59.611991Z  INFO benchmark: Aggregated throughput: 1000 MB/s
```

### Terminal B (group_id = 1, sender)

```text
2026-01-02T20:26:02.695211Z  INFO benchmark: Running Pair benchmark
2026-01-02T20:26:02.972004Z  INFO benchmark::pair: Worker 1 - started sending to 0
2026-01-02T20:26:02.986480Z  INFO benchmark::pair: Worker 1 - sent 1 MB in 0.006999 s
2026-01-02T20:26:02.990252Z  INFO benchmark: Aggregated throughput: 143 MB/s
```

> Note: a shutdown warning may appear after completion; the benchmark results
> are printed before the process exits.

---

## Interpretation

The pair benchmark demonstrates several key aspects of Burst Computing:

* **Indirect communication** enables portability in environments where
  direct worker-to-worker connections are not possible.
* BCM provides **MPI-like semantics** (`send` / `recv`) while abstracting
  away backend-specific details.
* The use of `group-id` allows minimal yet effective coordination between workers,
  emulating burst execution in distributed deployments.

Although executed locally, the benchmark exercises the same communication
abstraction used in cloud and serverless settings.

---

## Relation to MPI

The pair microbenchmark corresponds directly to MPI point-to-point operations:

| MPI           | Burst / BCM |
| ------------- | ----------- |
| MPI_Comm_size | burst_size  |
| MPI_Comm_rank | worker_id   |
| MPI_Send      | pair (send) |
| MPI_Recv      | pair (recv) |

This benchmark can be viewed as the Burst equivalent of a minimal
`MPI_Send / MPI_Recv` validation.

---

## Relation to k-means

This benchmark serves as a foundation for the k-means implementation,
where each iteration relies on repeated point-to-point and collective
communication patterns similar to those validated here.

Validating correctness and semantics at the microbenchmark level
simplifies the analysis of higher-level distributed algorithms.

---

## Notes on Evaluation

* Results obtained locally are **not intended to match cloud-scale performance**
  reported in the paper.
* The purpose of this benchmark is to validate:

  * correctness
  * communication semantics
  * middleware behavior

Performance analysis at scale is left for future experiments.

---

## Key Takeaways

* The pair benchmark validates correct BCM point-to-point communication
  via an indirect backend.
* Backend-based messaging behaves as expected.
* This benchmark provides a solid reference for:

  * understanding BCM behavior
  * preparing live coding exercises
  * extending communication patterns for k-means and other distributed algorithms

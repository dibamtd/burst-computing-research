# Pair Microbenchmark (BCM + RabbitMQ)

## Goal

The goal of the **pair microbenchmark** is to validate **point-to-point communication**
using the Burst Communication Middleware (BCM) when workers communicate **indirectly via a backend**.

This benchmark goes one step beyond the hello-world example by:
- using the official microbenchmark implementation
- measuring throughput and execution time
- validating group execution and synchronization semantics

---

## Execution Setup

### Environment
- Linux (WSL / Ubuntu)
- Rust toolchain
- Docker Desktop
- RabbitMQ backend

RabbitMQ is used as the **remote communication backend**, simulating a setting where
direct worker-to-worker sockets may not be available.

---

## Running the Pair Benchmark

The benchmark is executed using **two terminals**, each representing one worker group.

### Worker 0 (group_id = 0)

```bash
RUST_LOG=info cargo run --release -- \
  --benchmark pair \
  --burst-size 2 \
  --group-id 0 \
  --server "amqp://guest:guest@127.0.0.1:5672" \
  rabbitmq
````

### Worker 1 (group_id = 1)

```bash
RUST_LOG=info cargo run --release -- \
  --benchmark pair \
  --burst-size 2 \
  --group-id 1 \
  --server "amqp://guest:guest@127.0.0.1:5672" \
  rabbitmq
```

Each worker connects to the same RabbitMQ server but uses a different `group-id`.
In this benchmark, `group-id` identifies the worker’s group within the burst; running two
terminals with `group-id=0` and `group-id=1` emulates two workers participating in the same burst context.

---

## Observed Behavior

During execution:

* Both workers start as part of the same burst.
* `group_id=1` sends a **1MB payload** to `group_id=0`.
* `group_id=0` receives the payload via the RabbitMQ backend.
* Communication occurs **via BCM**, not through direct sockets.
* The benchmark reports per-worker throughput and an aggregated throughput value.

This confirms that:

* group execution works correctly
* point-to-point communication is correctly routed through the backend
* synchronization between workers is preserved

---

## Execution Proof (local run)

### Terminal A (group_id = 0, receiver)

```text
2026-01-02T20:28:59.509821Z  INFO benchmark: Arguments { benchmark: Pair, backend: Rabbitmq, server: Some("amqp://guest:guest@127.0.0.1:5672"), burst_id: "burst", burst_size: 2, groups: 2, group_id: "0", payload_size: 1048576, chunking: false, chunk_size: 1048576, tokio_broadcast_channel_size: 1048576 }
2026-01-02T20:28:59.510425Z  INFO benchmark: Running Pair benchmark
2026-01-02T20:28:59.510760Z  INFO benchmark: Total data to transmit: 1 MB (1 MB per worker)
2026-01-02T20:28:59.598967Z  INFO benchmark: start: 1767385739.598
2026-01-02T20:28:59.600051Z  INFO benchmark: thread start: id=0
2026-01-02T20:28:59.600535Z  INFO benchmark::pair: worker start: id=0
2026-01-02T20:28:59.607343Z  INFO benchmark::pair: Worker 0 - started receiving from 1
2026-01-02T20:28:59.608566Z  INFO benchmark::pair: worker 0 end
2026-01-02T20:28:59.608963Z  INFO benchmark::pair: Worker 0 - received 1 MB in 0.0009999275207519531 s (throughput 1000.072484501669 MB/s)
2026-01-02T20:28:59.610682Z  INFO benchmark: thread end: id=0
2026-01-02T20:28:59.611991Z  INFO benchmark: Aggregated throughput: 1000.072484501669 MB/s
2026-01-02T20:28:59.612547Z  INFO benchmark: end: 1767385739.612
```

### Terminal B (group_id = 1, sender)

```text
2026-01-02T20:26:02.694190Z  INFO benchmark: Arguments { benchmark: Pair, backend: Rabbitmq, server: Some("amqp://guest:guest@127.0.0.1:5672"), burst_id: "burst", burst_size: 2, groups: 2, group_id: "1", payload_size: 1048576, chunking: false, chunk_size: 1048576, tokio_broadcast_channel_size: 1048576 }
2026-01-02T20:26:02.695211Z  INFO benchmark: Running Pair benchmark
2026-01-02T20:26:02.696415Z  INFO benchmark: Total data to transmit: 1 MB (1 MB per worker)
2026-01-02T20:26:02.969496Z  INFO benchmark: start: 1767385562.968
2026-01-02T20:26:02.971134Z  INFO benchmark: thread start: id=1
2026-01-02T20:26:02.972004Z  INFO benchmark::pair: worker start: id=1
2026-01-02T20:26:02.977674Z  INFO benchmark::pair: Worker 1 - started sending to 0
2026-01-02T20:26:02.985718Z  INFO benchmark::pair: worker 1 end
2026-01-02T20:26:02.986480Z  INFO benchmark::pair: Worker 1 - sent 1 MB in 0.006999969482421875 s (throughput 142.85776566757494 MB/s)
2026-01-02T20:26:02.988484Z  INFO benchmark: thread end: id=1
2026-01-02T20:26:02.990252Z  INFO benchmark: Aggregated throughput: 142.85776566757494 MB/s
2026-01-02T20:26:02.991216Z  INFO benchmark: end: 1767385562.99
```

> Note: a shutdown warning may appear after completion; the benchmark results are already printed above before the process exits.

---

## Interpretation

The pair benchmark demonstrates several key aspects of Burst Computing:

* **Indirect communication** enables portability in serverless environments
  where direct worker-to-worker connections may not be possible.
* BCM provides **MPI-like semantics** (send/recv) while hiding backend details.
* The use of `group-id` provides a minimal setup to coordinate two communicating parties,
  and can emulate cross-pack behavior in a burst setting.

Although executed locally, the benchmark reflects the same communication abstraction used in
distributed/cloud deployments.

---

## Relation to MPI

The pair microbenchmark corresponds directly to MPI point-to-point operations:

| MPI           | Burst / BCM |
| ------------- | ----------- |
| MPI_Comm_size | burst_size  |
| MPI_Comm_rank | worker_id   |
| MPI_Send      | pair (send) |
| MPI_Recv      | pair (recv) |

This benchmark can be seen as the Burst equivalent of a minimal `MPI_Send / MPI_Recv` test.

---

## Notes on Evaluation

* Results obtained locally are **not meant to match cloud-scale performance** reported in the paper.
* The purpose of this benchmark is to validate:

  * correctness
  * communication semantics
  * middleware behavior

Performance analysis at scale is left for future experiments.

---

## Key Takeaways

* The pair benchmark validates correct BCM point-to-point communication via an indirect backend.
* Indirect backend-based messaging works as expected.
* This benchmark serves as a reference for:

  * understanding BCM behavior before analyzing k-means
  * preparing live coding exercises
  * extending communication patterns for k-means and other distributed algorithms

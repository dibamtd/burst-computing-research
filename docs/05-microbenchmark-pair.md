# Pair Microbenchmark (BCM + RabbitMQ)

## Goal
The goal of the **pair microbenchmark** is to validate **point-to-point
communication** using the Burst Communication Middleware (BCM) when workers
communicate **indirectly via a backend**.

This benchmark goes one step beyond the hello-world example by:
- using the official microbenchmark implementation
- measuring throughput and execution time
- validating correct group execution semantics

---

## Execution Setup

### Environment
- Linux (WSL / Ubuntu)
- Rust toolchain
- Docker
- RabbitMQ backend

RabbitMQ is used as the **remote communication backend**, simulating
inter-pack communication.

---

## Running the Pair Benchmark

The benchmark is executed using **two terminals**, each representing one
worker group.

### Worker 0
```bash
RUST_LOG=info cargo run --release \
  -- --benchmark pair \
  --burst-size 2 \
  --group-id 0 \
  --server "amqp://guest:guest@127.0.0.1:5672" \
  rabbitmq
````

### Worker 1

```bash
RUST_LOG=info cargo run --release \
  -- --benchmark pair \
  --burst-size 2 \
  --group-id 1 \
  --server "amqp://guest:guest@127.0.0.1:5672" \
  rabbitmq
```

Each worker connects to the same RabbitMQ server but uses a different
`group-id`, emulating separate worker groups.

---

## Observed Behavior

During execution, the following behavior is observed:

* Both workers start as part of the same burst.
* Worker 1 sends a message payload.
* Worker 0 receives the payload.
* Communication occurs **via the BCM**, not through direct sockets.
* The benchmark automatically reports:

  * total execution time
  * throughput

This confirms that:

* group execution works correctly
* point-to-point communication is correctly routed through the backend
* synchronization between workers is preserved

---

## Interpretation

The pair benchmark demonstrates several key aspects of Burst Computing:

* **Indirect communication** enables portability in serverless environments
  where direct worker-to-worker connections may not be possible.
* BCM provides **MPI-like semantics** (send/recv) while hiding backend details.
* The use of `group-id` allows the benchmark to simulate communication between
  different worker packs.

Although executed locally, the benchmark reflects the same communication
semantics used in cloud deployments.

---

## Relation to MPI

The pair microbenchmark corresponds directly to MPI point-to-point operations:

| MPI           | Burst / BCM |
| ------------- | ----------- |
| MPI_Comm_size | burst_size  |
| MPI_Comm_rank | worker_id   |
| MPI_Send      | pair (send) |
| MPI_Recv      | pair (recv) |

This benchmark can be seen as the Burst equivalent of a minimal
`MPI_Send / MPI_Recv` test.

---

## Notes on Evaluation

* Results obtained locally are **not meant to match cloud-scale performance**
  reported in the paper.
* The purpose of this benchmark is to validate:

  * correctness
  * communication semantics
  * middleware behavior

Performance analysis at scale is left for future experiments.

---

## Key Takeaways

* The pair benchmark validates correct BCM point-to-point communication.
* Indirect backend-based messaging works as expected.
* This benchmark serves as a reference for:

  * understanding BCM internals
  * preparing live coding exercises
  * extending communication patterns for k-means and other algorithms

```

# Burst Computing — Communication Middleware Exploration

This repository documents my hands-on study and experimentation with **Burst Computing**
and the **Burst Communication Middleware (BCM)**, based on the USENIX ATC paper:

**Burst Computing: Isolated Functions Meet Elastic Clusters**  
Pons et al., USENIX ATC 2025

The goal of this repository is **understanding**, not re-implementing: clearly explain
why Burst exists, what problem it solves, and how its communication model works in practice.

---

## Motivation: Why Burst Computing?

### The problem with traditional FaaS

In classical Function-as-a-Service (FaaS) platforms:

- Each function (worker) is invoked independently
- Workers are isolated by design
- There is no notion of group execution
- Communication must go through external services (Redis, S3, message queues)

This model works well for stateless tasks, but breaks down for parallel and distributed algorithms.

### Traditional FaaS execution

```text
                ┌───────────┐
                │   Client   │
                └─────┬─────┘
                      │
        ┌─────────────┼─────────────┐
        │             │             │
     invoke         invoke         invoke
        │             │             │
        v             v             v
 ┌──────────┐   ┌──────────┐   ┌──────────┐
 │ Worker 0 │   │ Worker 1 │   │ Worker 2 │
 └────┬─────┘   └────┬─────┘   └────┬─────┘
      │              │              │
      └──────────────┼──────────────┘
                     │
                     v
           ┌─────────────────────┐
           │ External Service     │
           │ (Redis / S3 / MQ)    │
           └─────────────────────┘
````

Common issues:

* High startup latency
* No locality awareness
* Expensive coordination
* Poor fit for MPI-style algorithms

---

## Burst Computing: Key Idea

Burst Computing introduces **group invocation** using a concept called a **flare**.

A flare:

* Starts multiple workers simultaneously
* Groups them into packs
* Enables efficient indirect communication
* Preserves isolation while enabling cooperation

### Burst execution with flare (Figure-2 style)

```text
                ┌───────────┐
                │   Client   │
                └─────┬─────┘
                      │
                    flare
                      │
                      v
            ┌───────────────────┐
            │     Controller     │
            └─────────┬─────────┘
                      │
          ┌───────────┼───────────┐
          │                           │
          v                           v
 ┌─────────────────┐        ┌─────────────────┐
 │ Pack 0           │        │ Pack 1           │
 │ ─────────────── │        │ ─────────────── │
 │ Worker 0         │        │ Worker 3         │
 │ Worker 1         │        │ Worker 4         │
 │ Worker 2         │        │ Worker 5         │
 └─────────────────┘        └─────────────────┘
          │                           │
          └───────── indirect ───────┘
                    communication
```

Key advantages:

* Single request starts many workers
* Workers are aware of the group
* Locality inside packs
* Reduced communication overhead

---

## What is the Burst Communication Middleware (BCM)?

The **Burst Communication Middleware (BCM)** is the runtime layer that enables:

* Indirect communication between workers
* Collective operations (broadcast, gather, pair, all-to-all)
* Multiple backends:

  * RabbitMQ
  * Redis (lists / streams)
  * Tokio channels (local)

BCM provides MPI-like semantics in a serverless-compatible execution model.

---

## What I Implemented and Executed

### 1) Local execution environment

* Linux (WSL / Ubuntu)
* Rust toolchain
* Docker Desktop
* RabbitMQ backend

Documentation:

* `docs/03-local-setup.md`

### 2) Baseline validation (official hello world)

I executed the official BCM hello-world example (`hello_world_local`) to validate setup and basic send/recv.

Documentation + execution log:

* `docs/04-hello-world.md`

### 3) Microbenchmarks (pair benchmark, RabbitMQ backend)

I compiled and executed the **pair** benchmark using RabbitMQ.

Two workers, two terminals:

**Worker 0**

```bash
RUST_LOG=info cargo run --release \
  -- --benchmark pair \
  --burst-size 2 \
  --group-id 0 \
  --server "amqp://guest:guest@127.0.0.1:5672" \
  rabbitmq
```

**Worker 1**

```bash
RUST_LOG=info cargo run --release \
  -- --benchmark pair \
  --burst-size 2 \
  --group-id 1 \
  --server "amqp://guest:guest@127.0.0.1:5672" \
  rabbitmq
```

Observed behavior:

* Workers start together as part of the same burst
* Worker 0 and worker 1 exchange data via the middleware
* Communication happens through the backend, not direct sockets
* Throughput and timing are measured automatically

Documentation:

* `docs/05-microbenchmark-pair.md`

### 4) Custom point-to-point demo (P2P)

I implemented and executed a minimal P2P example adapted from the official hello-world.
The demo adds a **msg_id + ACK matching** check to reason about request/reply association.

* Code: `examples/p2p_demo.rs`
* Documentation + execution log: `docs/06-live-coding-p2p.md`

---

## Relation to MPI

A conceptual mapping between MPI and Burst/BCM:

| MPI Concept     | Burst / BCM                            |
| --------------- | -------------------------------------- |
| MPI_Init        | Flare invocation (group start)         |
| MPI_Comm_size   | burst_size                             |
| MPI_Comm_rank   | worker_id                              |
| MPI_Send / Recv | point-to-point send/recv (pair / demo) |
| MPI_Bcast       | broadcast                              |
| MPI_Gather      | gather                                 |

Note: this mapping is conceptual. Flare invocation is platform-level group start (it subsumes job
launch and initialization), whereas MPI_Init is an explicit call inside the application.

---

## What This Repository Demonstrates

* Clear understanding of FaaS limitations for distributed algorithms
* Conceptual understanding of Burst Computing (flare, packs, group execution)
* Practical execution of BCM (hello world + microbenchmark)
* Custom P2P example with correctness reasoning (msg_id + ACK)
* Readiness to analyze and extend distributed algorithms (e.g., K-means)

---

## Next Steps

* Analyze the K-means implementation on top of BCM
* Study how communication patterns scale with burst size
* Explore balanced K-means as a potential extension
* Prepare a short presentation and be ready for live explanation

---

## Author

**Diba Mtd**
Prospective PhD student — Distributed Systems & Cloud Computing
GitHub: [https://github.com/dibamtd](https://github.com/dibamtd)

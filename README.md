
# 🌀 Burst Computing — Communication Middleware Exploration
Spaces
This repository documents my hands-on study and experimentation with **Burst Computing** and the **Burst Communication Middleware**, based on the USENIX ATC paper:

> *Burst Computing: Isolated Functions Meet Elastic Clusters*
> Pons et al., USENIX ATC 2025

The goal of this repository is **understanding**, not re-implementing:
to clearly explain *why Burst exists*, *what problem it solves*, and *how its communication model works in practice*.

---

## 🎯 Motivation: Why Burst Computing?

### The problem with traditional FaaS

In classical **Function-as-a-Service (FaaS)** platforms:

* Each function (worker) is invoked independently
* Workers are isolated by design
* There is no notion of *group execution*
* Communication must go through **external services** (Redis, S3, message queues)

This model works well for stateless tasks, but **breaks down for parallel and distributed algorithms**.

### Traditional FaaS execution

```
                ┌───────────┐
                │   Client  │
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

```

**Issues:**

* High startup latency
* No locality
* Expensive coordination
* Poor fit for MPI-style algorithms

---

## 🚀 Burst Computing: Key Idea

Burst Computing introduces **group invocation** using a concept called a **flare**.

A *flare*:

* Starts multiple workers **simultaneously**
* Groups them into **packs**
* Enables efficient **indirect communication**
* Preserves isolation while enabling cooperation

### Burst execution with flare (Figure-2 style)

```
                ┌───────────┐
                │   Client  │
                └─────┬─────┘
                      │
                   flare
                      │
                      v
            ┌───────────────────┐
            │     Controller    │
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

**Key advantages:**

* Single request starts many workers
* Workers are aware of the group
* Locality inside packs
* Reduced communication overhead

---

## 🧠 What is the Burst Communication Middleware?

The **Burst Communication Middleware (BCM)** is the runtime layer that enables:

* Indirect communication between workers
* Collective operations (broadcast, gather, pair, all-to-all)
* Multiple backends:

  * RabbitMQ
  * Redis (lists / streams)
  * Tokio channels (local)

It provides **MPI-like semantics** in a serverless-compatible model.

---

## 🧪 What I Implemented and Executed

### 1. Local execution environment

* Linux (WSL / Ubuntu)
* Rust toolchain
* Docker
* RabbitMQ backend

### 2. Microbenchmarks (official Burst code)

I successfully compiled and executed the **pair benchmark** using RabbitMQ.

**Two workers, two terminals:**

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

### Observed behavior

* Workers start together as part of the same burst
* Worker 1 sends data → Worker 0 receives
* Communication happens through the middleware, not direct sockets
* Throughput and timing are measured automatically

This confirms correct **group execution + indirect communication**.

---

## 🧩 Relation to MPI

| MPI               | Burst Middleware |
| ----------------- | ---------------- |
| `MPI_Init`        | flare invocation |
| `MPI_Comm_size`   | burst_size       |
| `MPI_Comm_rank`   | worker_id        |
| `MPI_Send / Recv` | pair             |
| `MPI_Bcast`       | broadcast        |
| `MPI_Gather`      | gather           |

Burst can be seen as **MPI concepts adapted to serverless environments**.

---

## 📌 What This Repository Demonstrates

* Clear understanding of the **limitations of FaaS**
* Conceptual understanding of **Burst Computing**
* Practical execution of **Burst communication middleware**
* Ability to reason about **parallelism, locality, and coordination**
* Readiness to analyze and extend distributed algorithms (e.g., K-means)

---

## 🧭 Next Steps

* Analyze K-means implementation on top of Burst
* Study how communication patterns scale with burst size
* Explore balanced K-means as a possible extension
* Prepare short presentation and live explanation

---

## 👩‍💻 Author

**Diba Mtd**
Prospective PhD student — Distributed Systems & Cloud Computing
GitHub: [https://github.com/dibamtd](https://github.com/dibamtd)

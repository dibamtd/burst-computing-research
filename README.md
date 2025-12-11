# 🌀 Burst Computing — Distributed Middleware Research Repository

This repository documents my research journey in understanding, analyzing, and extending the **Burst Communication Middleware (BCM)** developed by **Prof. Pedro García López** and his team at URV.

It contains:

- Theoretical explanations  
- System architecture diagrams  
- Executable Rust examples  
- Performance notes and benchmarks  
- K-means on Burst  
- A plan for Balanced K-means (requested in the PhD evaluation challenge)

---

## 🎯 Goals of This Repository

This repository is designed to demonstrate:

### **1. Understanding of distributed communication middleware**
- How Burst works internally (actors, batching, async channels, RabbitMQ backend)
- How it differs from MPI and classical message-passing

### **2. Ability to execute and analyze Rust-based distributed code**
- Running official Burst examples locally
- Observing concurrency, channels, and messaging flow

### **3. Ability to build distributed algorithms on top of Burst**
- From MPI-based K-means → Burst K-means → Balanced K-means

### **4. Research-level documentation**
- Proper diagrams
- Architecture analysis
- Notes and design reflections

---

## 📁 Repository Structure

```plaintext
burst-computing-research/
│
├── burst_examples/
│   ├── hello_world_local.md
│   ├── hello_world_remote.md
│   ├── burst_ping_pong.rs        # (coming soon)
│   ├── burst_kmeans_notes.md
│
├── docs/
│   ├── BURST_OVERVIEW.md         # High-level conceptual explanation
│   ├── INTERNAL_DESIGN.md        # How Burst works internally
│   ├── HOW_CHANNELS_WORK.md      # Channels, batching, and queues
│   ├── KMEANS_ARCHITECTURE.md    # K-means distributed design
│   ├── burst_hello_world.md      # Analysis of hello world example
│   │
│   ├── diagrams/
│   │   ├── architecture.png
│   │   ├── kmeans_flow.png
│   │   └── messaging_flow.png
│   │
│   └── balanced_kmeans/
│       ├── PLAN.md               # Balanced K-means design
│       └── IMPLEMENTATION.md     # To be completed later
│
├── notes/
│   ├── rust_learning.md
│   ├── meeting_notes.md
│
├── benchmarks/
│   ├── local_tests.md
│   └── performance_evaluation.md
│
└── README.md
````

---

## 🧪 Rust Examples (Executed Locally)

These examples are based on the official Burst middleware:

* `hello_world_local` (point-to-point messaging)
* `hello_world_remote` (RabbitMQ backend)
* `broadcast.rs`
* `reduce.rs`
* `kmeans_burst.rs` (WIP)

All examples run inside:

```bash
cargo run --example <name> --features rabbitmq
```

---

## ⚖️ Balanced K-means (Requested by Prof. Pedro)

A complete design plan is available in:

```
docs/balanced_kmeans/PLAN.md
```

Includes:

* Algorithmic ideas to enforce balanced clusters
* Communication implications
* Impact on parallelization
* Evaluation strategy

The implementation will be added as:

```
docs/balanced_kmeans/IMPLEMENTATION.md
```

---

## 👩‍💻 Author

**Diba Mtd**
Prospective PhD student working with **Prof. Pedro García López**
Research areas: Distributed Systems, Cloud Computing, Serverless Middleware

Email: **[diba.mo72@gmail.com](mailto:diba.mo72@gmail.com)**
GitHub: **[https://github.com/dibamtd](https://github.com/dibamtd)**

---

## 🌱 Status

### ✔ Completed

* MPI fundamentals (Python + MPI4Py)
* Distributed MPI K-means
* Burst compiled locally (Rust + RabbitMQ)
* Running `hello_world_local` successfully
* Repository structure organized for research work

### ⬜ In Progress

* Custom Burst point-to-point example
* Burst K-means implementation
* Balanced K-means implementation

---

## 🔭 Next Steps

* Document internal Burst architecture (`BURST_OVERVIEW.md`)
* Add messaging-flow diagrams (Burst vs MPI)
* Build toy examples (ping-pong, worker pipeline)
* Compare behavior of MPI vs Burst (qualitative + experimental)

---

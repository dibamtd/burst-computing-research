# 🌀 Burst Computing — Distributed Middleware Research Repository

This repository documents my journey in understanding, analyzing, and extending the **Burst Communication Middleware (BCM)** developed by Prof. Pedro García López and his team.  
It includes:

- Theoretical explanations  
- System architecture diagrams  
- Executable Rust examples  
- Performance benchmarks  
- K-means on Burst  
- A plan for **Balanced K-means**, as required in the PhD evaluation challenge  

---

## 🎯 Goal of This Repository

This repo is designed to demonstrate:

### 1. Understanding of distributed communication middleware  
How Burst works internally (actors, batching, channels, RabbitMQ backend).

### 2. Ability to execute and analyze Rust-based distributed code  
Running and extending the official Burst middleware implementation.

### 3. Ability to build algorithms (K-means) on top of the middleware  
From MPI-based K-means to Burst-based K-means, and then to balanced K-means.

### 4. Research-level documentation & analysis  
Diagrams, complexity reasoning, and written reflections.

---

## 📁 Repository Structure

burst-computing-research/
│
├── burst_examples/            
│   ├── hello_world_local.md
│   ├── hello_world_remote.md
│   ├── burst_ping_pong.rs?   (بعداً)
│   ├── burst_kmeans_notes.md
│
├── docs/                      
│   ├── BURST_OVERVIEW.md              # توضیح کامل درباره Burst
│   ├── INTERNAL_DESIGN.md             # نحوه کار داخل Middleware
│   ├── HOW_CHANNELS_WORK.md           # Channels / batching / queues
│   ├── KMEANS_ARCHITECTURE.md         # تحلیل کامل K-means Rust
│   ├── burst_hello_world.md           # همین فایلی که الان ساختیم
│   ├── diagrams/
│   │    ├── architecture.png
│   │    ├── kmeans_flow.png
│   │    └── messaging_flow.png
│   └── balanced_kmeans/
│        ├── PLAN.md                   # طراحی الگوریتم
│        └── IMPLEMENTATION.md         # بعداً اضافه می‌کنیم
│
├── notes/                             
│   ├── rust_learning.md               # نکات Rust که یاد می‌گیری
│   ├── meeting_notes.md               # نکات جلسات با استاد
│
├── benchmarks/
│   ├── local_tests.md
│   └── performance_evaluation.md
│
└── README.md


## 🧪 Rust Examples

* `hello_world_local.rs` (point-to-point over Burst)
* `broadcast.rs`
* `reduce.rs`
* `kmeans_burst.rs` (WIP)

---

## ⚖️ Balanced K-means (Requested by Prof. Pedro)

A full design & work plan is available in:

```text
docs/balanced_kmeans/PLAN.md
```

This includes:

* High-level algorithm idea
* How to enforce balanced clusters
* Impact on communication and parallelization
* Evaluation strategy (speed vs. balance)

---

## 👩‍💻 Author

**Diba Mtd**
Prospective PhD student working with Prof. Pedro García López's group (URV)
Focused on distributed systems, cloud computing, and serverless middleware.

---

## 🌱 Status

* ✔ MPI fundamentals (Python + MPI4Py)
* ✔ MPI-based distributed K-means
* ✔ Burst compiled locally (Rust, RabbitMQ backend)
* ✔ `hello_world_local` example running with RabbitMQ
* ⬜ Custom point-to-point example using Burst
* ⬜ K-means ported to Burst
* ⬜ Balanced K-means implementation and evaluation

---

## 🔭 Next Steps

* Document internal Burst architecture in `docs/BURST_OVERVIEW.md`
* Add diagrams for message flow (Burst vs MPI)
* Implement toy apps (ping-pong, broadcast, K-means worker) on Burst
* Compare MPI vs Burst behavior (conceptually and experimentally)

```

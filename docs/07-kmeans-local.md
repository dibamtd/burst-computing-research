# 07 - kmeans_local (Local execution with BCM)

## Goal
Run a **BCM-based distributed k-means** locally using the **RabbitMQ backend** (synthetic data, no cloud/S3) and verify correct end-to-end execution.

This document also explains:
- how computation is distributed across workers,
- how workers communicate (broadcast/gather),
- and what determines scalability (compute vs communication).

---

## Prerequisites
- Rust toolchain
- Docker

---

## Start RabbitMQ
```bash
docker run -d --name rabbitmq \
  -p 5672:5672 -p 15672:15672 \
  rabbitmq:3-management
````

(Optional) UI:

* [http://localhost:15672](http://localhost:15672)
* user/pass: guest/guest

---

## Run the example

From the upstream BCM repository:

```bash
cargo run --example kmeans_local --features rabbitmq
```

---

## Output (sample)

```txt
[kmeans_local] done. final centroids = [76.433464, 76.0197, 47.352646, 43.56821, 78.36914, 21.850948, 21.756115, 75.069626, 20.713223, 20.303316]
```

---

## What this program does (high level)

This `kmeans_local` is a distributed k-means over synthetic 2D points.

* Each worker generates its own local dataset (synthetic points).
* One worker is the **root** (worker 0).
* In each iteration:

  1. root broadcasts current centroids to everyone,
  2. each worker assigns its local points to the nearest centroid and computes **partial sums + counts**,
  3. workers gather partial results to root,
  4. root aggregates and updates centroids.

This is the classic bulk-synchronous k-means structure.

---

## How computation is distributed

Let:

* **P** = number of workers
* **N** = total number of points
* **k** = number of clusters
* **d** = dimensions (here 2)

The compute-heavy part is the assignment step: for each point, compute distance to each centroid.

Per iteration, each worker does roughly:

* ~ (N/P) points × k centroid checks × d operations

So compute cost per worker is approximately:

* **O((N/P) · k · d)**

As **P increases**, compute per worker decreases ~linearly (good scaling) *until communication dominates*.

---

## How workers communicate (BCM collectives)

This example uses MPI-like collectives through BCM:

### 1) Broadcast: centroids from root → all workers

* Root sends centroid vector of size **k·d** floats.
* All workers receive the same centroids at the start of each iteration.

### 2) Gather: partial sums and counts from all workers → root

Each worker sends:

* partial sums: `k·d` floats
* partial counts: `k` integers

Root receives P messages for sums and P messages for counts, aggregates them, and updates centroids.

---

## Diagram: computation + communication pattern (Mermaid)

```mermaid
flowchart TD
  R[Worker 0 (Root)\nUpdate centroids] -->|broadcast centroids (k·d)| W1[Worker 1\nAssign points\nCompute partial sums/counts]
  R -->|broadcast centroids (k·d)| W2[Worker 2\nAssign points\nCompute partial sums/counts]
  R -->|broadcast centroids (k·d)| Wn[Worker N\nAssign points\nCompute partial sums/counts]

  W1 -->|gather sums (k·d) + counts (k)| R
  W2 -->|gather sums (k·d) + counts (k)| R
  Wn -->|gather sums (k·d) + counts (k)| R
```

## Diagram (fallback text, if Mermaid is not rendered)

```txt
Iteration i:

           broadcast centroids (k·d)
        -------------------------------->
       |                                  |
+------+-----+                     +------+-----+
| Worker 0   |                     | Worker j   |
| (root)     |                     |            |
| update     |                     | assign pts |
| centroids  |                     | compute    |
+------+-----+                     | local sums |
       ^                           +------+-----+
       |                                  |
       |     gather sums (k·d) + counts(k)|
       <----------------------------------

(All workers send partial results back to root)
```

---

## Scalability: what improves and what limits it

### What scales well

✅ Compute scales well with P:

* each worker processes fewer points: N/P
* for large N, the assignment step dominates, so speedup can be near-linear

### What limits scaling (why it can flatten)

Communication per iteration is roughly:

**Broadcast traffic**

* data size: **k·d·sizeof(f32)**

**Gather traffic**

* each worker sends:

  * sums: **k·d·sizeof(f32)**
  * counts: **k·sizeof(u32)**
* root receives O(P) messages → root becomes a hotspot

Two common bottlenecks:

1. **Root bottleneck**: root aggregates results from all workers each iteration.
2. **Synchronization cost**: k-means is iterative/bulk-synchronous; faster workers wait for slower ones.

### Rule of thumb

* If **N is large** → compute dominates → better scaling.
* If **P is large** or **k·d is large** → communication dominates → scaling flattens.

---

## Notes

* This run validates local distributed execution using BCM + RabbitMQ without any cloud services.
* The communication pattern is comparable to MPI-style programs (broadcast + gather), but implemented via BCM.
* MPI reference implementation and explanations are in: [https://github.com/dibamtd/mpi-distributed-computing](https://github.com/dibamtd/mpi-distributed-computing)
```

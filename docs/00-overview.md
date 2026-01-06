# Documentation Overview

This folder documents my understanding and hands-on experimentation with
**Burst Computing** and the **Burst Communication Middleware (BCM)**.
The focus is on local execution, communication semantics, and practical validation
of the Rust prototype (using RabbitMQ as the backend), without any cloud components.

The documents progress from conceptual background to concrete experiments and
execution results, and are intended to demonstrate both conceptual understanding
and practical system-level work in distributed computing.

## Recommended reading order

1. **01-burst-concepts.md**  
   Introduction to Burst Computing, its motivation, and how it relates to
   distributed and parallel execution models (e.g., MPI-like semantics).

2. **02-bcm-design.md**  
   Overview of the Burst Communication Middleware (BCM), including its purpose,
   architecture, and communication abstractions.

3. **03-local-setup.md**  
   Instructions for setting up the local development environment, including
   RabbitMQ and required dependencies.

4. **04-hello-world.md**  
   A minimal “hello world” example demonstrating basic worker initialization
   and communication using BCM.

5. **05-microbenchmark-pair.md**  
   Pair microbenchmark experiments to validate point-to-point communication
   behavior and measure basic communication costs.

6. **06-p2p.md**  
   A point-to-point demo used for live coding and demonstration, showing
   explicit send/receive patterns between workers.

7. **07-kmeans-local.md**  
   Local execution of a distributed k-means application using BCM with synthetic
   data. This document explains how computation is distributed across workers,
   how broadcast and gather communication are used, and how scalability is
   affected by computation and communication costs.

8. **10-balanced-kmeans.md**  
   Design and analysis of balanced k-means in distributed systems. This document
   discusses the motivation, advantages over standard k-means, and the algorithmic
   and system-level challenges that make balanced k-means more complex to
   implement in a distributed setting.

9. **99-presentation-outline.md**  
   A short outline of the planned presentation, summarizing the key ideas,
   experiments, and discussion points.

## Related work (MPI)
Fundamental MPI concepts and distributed algorithms (including k-means) are
implemented and discussed in a separate repository:
- https://github.com/dibamtd/mpi-distributed-computing

## Scope and limitations

- All experiments in this repository are executed locally.
- RabbitMQ is used as the communication backend.
- No cloud services or object storage (e.g., S3) are involved.
- The emphasis is on correctness, communication behavior, scalability reasoning,
  and system-level understanding rather than large-scale performance evaluation.

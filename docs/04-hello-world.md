# Hello World (BCM)

## Goal
The goal of this example is to understand the **minimal usage of the Burst
Communication Middleware (BCM)** by running a simple point-to-point
communication between two workers.

This example is intentionally simple and serves as a foundation for:
- understanding BCM initialization
- observing send/recv semantics
- preparing for live coding during evaluation

---

## Code Location
The hello-world example is provided by the official Burst codebase:

- `burst-communication-middleware/examples/hello_world_local.rs`

This example focuses on **local execution** and uses the BCM API directly.

---

## How to Run (Local)

### Prerequisites
- Rust toolchain
- Docker
- RabbitMQ running locally

### Start RabbitMQ
```bash
docker run -d --name rabbitmq \
  -p 5672:5672 -p 15672:15672 \
  rabbitmq:3-management
````

### Run the example

```bash
cargo run --release --example hello_world_local
```

---

## What Happens Internally

1. The runtime initializes the Burst context.
2. Two workers are created as part of the same burst.
3. The BCM is initialized with the selected backend.
4. One worker sends a message using BCM.
5. The other worker receives the message using BCM.
6. The program terminates after successful communication.

All communication is performed **through the middleware**, not through
direct socket connections between workers.

---

## Message Flow

* One worker acts as the sender.
* One worker acts as the receiver.
* BCM determines whether the communication path is:

  * local (intra-pack), or
  * indirect via the configured backend.

This abstraction hides transport details from the application code.

---

## Relation to MPI

This hello-world example maps naturally to MPI concepts:

| MPI Concept   | Burst / BCM      |
| ------------- | ---------------- |
| MPI_Init      | flare invocation |
| MPI_Comm_rank | worker_id        |
| MPI_Send      | BCM send         |
| MPI_Recv      | BCM recv         |

Burst can be seen as **MPI-style communication adapted to serverless
constraints**, where group execution and locality are managed by the platform.

---

## Key Takeaways

* BCM provides clear MPI-like send/recv semantics.
* Communication is abstracted from the underlying transport.
* This example validates correct middleware initialization and usage.
* The hello-world example serves as a base for more complex programs
  (e.g., microbenchmarks and k-means).

```
```


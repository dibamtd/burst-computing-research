# Hello World (BCM)

## Goal

The goal of this example is to understand the minimal usage of the
Burst Communication Middleware (BCM) by running a simple point-to-point
communication between two workers.

This example is intentionally simple and serves as a foundation for:

- Understanding BCM initialization
- Observing send/recv semantics
- Preparing for live coding during evaluation

## Code Location

The hello-world example is provided by the official Burst codebase:

```

burst-communication-middleware/examples/hello_world_local.rs

````

This example focuses on local execution and uses the BCM API directly.

## How to Run (Local)

### Prerequisites

- Rust toolchain
- Docker
- RabbitMQ running locally

### Start RabbitMQ

RabbitMQ is executed locally using Docker:

```bash
docker run -d --name rabbitmq \
  -p 5672:5672 -p 15672:15672 \
  rabbitmq:3-management
````

### Run the Example

The example is executed from the root of the
`burst-communication-middleware` project:

```bash
RUST_LOG=info cargo run --example hello_world_local --features rabbitmq
```

## Validation: Execution Proof

The example was executed successfully using the RabbitMQ backend.

### Sample Execution Log

```text
thread start: id=0
thread start: id=1
worker 0 sending message
worker 1 received message: "hello world"
worker 0 received message: "bye!"
thread end: id=0
thread end: id=1
```

This output confirms that:

* Both workers were initialized correctly
* Point-to-point communication succeeded
* A full send/receive round-trip completed as expected

## What Happens Internally

During execution:

* The runtime initializes the Burst context
* Two workers are created as part of the same burst
* The BCM is initialized with the selected backend (RabbitMQ)
* One worker sends a message using BCM
* The other worker receives the message and replies
* The program terminates after successful communication

All communication is performed through the middleware, not through
direct socket connections between workers.

## Message Flow

* One worker acts as the sender
* One worker acts as the receiver

BCM determines whether the communication path is:

* Local (intra-pack), or
* Indirect via the configured backend

This abstraction hides transport details from the application code.

## Relation to MPI

This hello-world example maps naturally to MPI concepts:

| MPI Concept   | Burst / BCM      |
| ------------- | ---------------- |
| MPI_Init      | Flare invocation |
| MPI_Comm_rank | worker_id        |
| MPI_Send      | BCM send         |
| MPI_Recv      | BCM recv         |

Burst can be seen as MPI-style communication adapted to serverless
constraints, where group execution and locality are managed by the
platform.

## Key Takeaways

* BCM provides clear MPI-like send/recv semantics
* Communication is abstracted from the underlying transport
* This example validates correct middleware initialization and usage
* The hello-world example serves as a base for more complex programs
  (e.g., microbenchmarks and k-means)

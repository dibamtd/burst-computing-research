## 📄 `docs/03-local-setup.md` (Merged & Clean)

````md
# Local Setup

This repository is evaluated using a local environment to validate the
Burst Communication Middleware (BCM) before moving to more complex
distributed experiments.

## Environment

- Host OS: Windows
- Linux Environment: WSL (Ubuntu)
- Language: Rust
- Async Runtime: Tokio
- Container Runtime: Docker Desktop
- Messaging Backend: RabbitMQ

The Rust toolchain is installed inside WSL.
All Rust programs are executed directly on WSL.

## Docker and RabbitMQ

RabbitMQ is used as the communication backend for BCM examples.
It is executed locally using Docker Desktop.

The following command is used to start RabbitMQ:

```bash
docker run -d --name rabbitmq \
  -p 5672:5672 -p 15672:15672 \
  rabbitmq:3-management
````

Exposed ports:

* `5672` — AMQP (used by BCM)
* `15672` — RabbitMQ management UI (optional)

RabbitMQ is started once and kept running while executing the examples.

The Rust applications connect to RabbitMQ via `localhost`.

## Feature Flags

BCM supports multiple communication backends via Cargo feature flags.
Examples using RabbitMQ must be compiled and executed with the
`rabbitmq` feature enabled.

Example:

```bash
cargo run --example hello_world_local --features rabbitmq
```

## Execution Model

* RabbitMQ runs inside a Docker container
* Rust examples run locally on WSL
* Communication occurs via AMQP over `localhost`

This setup is used consistently for all experiments documented in
this repository.

```


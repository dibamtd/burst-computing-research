# 07 - kmeans_local (Local execution with BCM)

## Goal
Run the BCM-based k-means example locally using the RabbitMQ backend (synthetic data, no cloud or S3) and verify correct end-to-end execution.

## Prerequisites
- Rust toolchain
- Docker

## Start RabbitMQ
```bash
docker run -d --name rabbitmq \
  -p 5672:5672 -p 15672:15672 \
  rabbitmq:3-management
````

## Run the example

From the upstream BCM repository:

```bash
cargo run --example kmeans_local --features rabbitmq
```

## Output (sample)

```txt
[kmeans_local] done. final centroids = [
    76.433464,
    76.0197,
    47.352646,
    43.56821,
    78.36914,
    21.850948,
    21.756115,
    75.069626,
    20.713223,
    20.303316]


```

## Notes

* The example completes successfully and prints final centroids.
* Workers execute in parallel and communicate using the Burst Communication Middleware (BCM).
* This run validates local execution without any cloud-specific components.

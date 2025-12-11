# Burst Middleware — Hello World Example (Local + RabbitMQ)

This document explains how the **hello_world_local** example in  
the *Burst Communication Middleware* works, how to run it,  
and why it is important in understanding the architecture of Burst.

This is part of my research workflow for studying the middleware  
developed in the paper **“Burst: Locality-Aware Burstable Serverless Computing” (USENIX ATC 2025)**  
by Pedro García López et al.

---

# 1. What This Example Demonstrates

The `hello_world_local` example is the simplest demonstration of:

✔ How Burst initializes workers  
✔ How worker threads communicate using the middleware  
✔ How messages are serialized/deserialized  
✔ How local execution works without needing multiple nodes  
✔ How RabbitMQ is used as the messaging backend

This is the **first step** toward understanding how Burst handles:

- point-to-point messaging  
- channels  
- batching  
- queue-based communication  
- distributed execution models  

---

# 2. How to Run the Example

Before running the example:

### **1) Start RabbitMQ**

```bash
sudo service rabbitmq-server start
sudo service rabbitmq-server status
````

Status should show:

```
Active: active (running)
```

---

### **2) Run the hello_world example**

Inside the project folder:

```bash
cd ~/burst-work/burst-communication-middleware
RUST_LOG=info cargo run --example hello_world_local --features rabbitmq
```

---

# 3. Actual Output (Example Run)

Below is the real log output from running on my system:

```
[INFO] thread start: id=1
[INFO] thread start: id=0
[INFO] worker 0 sending message
[INFO] worker 1 received message: "hello world"
[INFO] worker 0 received: "bye!"
[INFO] thread end: id=1
[INFO] thread end: id=0
```

This confirms that:

* Worker 0 → sends “hello world”
* Worker 1 → receives it
* Worker 1 → sends back “bye!”
* Worker 0 → receives the response
* Both threads terminate cleanly

---

# 4. Architecture Explanation

### ⚙️ Worker Initialization

Two workers are created:

```
Worker 0  (sender)
Worker 1  (receiver)
```

The Burst runtime wraps each worker inside a **Middleware Actor**,
which manages:

* channels
* queues
* background tasks
* async behavior
* message serialization
* communication coordination

---

### ⚙️ Message Flow Diagram

```
             ┌──────────────┐
             │  Worker 0    │
             │ (Sender)     │
             └───────┬──────┘
                     │ send("hello world")
                     ▼
          ┌───────────────────────┐
          │   Burst Middleware    │
          │ + RabbitMQ backend    │
          └──────────┬────────────┘
                     │ deliver msg
                     ▼
             ┌──────────────┐
             │  Worker 1    │
             │ (Receiver)   │
             └───────┬──────┘
                     │ send("bye!")
                     ▼
                Back to Worker 0
```

---

# 5. What This Code Teaches About Burst

### ✔ 1. Burst abstracts messaging

You do **not** deal with low-level sockets or raw RabbitMQ queues.

### ✔ 2. Worker identity = integer IDs

All communication is addressed by numeric worker IDs (0,1,2,…).

### ✔ 3. Middleware Actor Handle

Allows synchronous messaging from worker code, even though Burst is asynchronous internally.

### ✔ 4. Serialization

Messages are converted:

```
StringMessage -> Bytes -> Queue -> Bytes -> StringMessage
```

### ✔ 5. Thread-based execution

Each worker runs in a separate thread → simulating distributed workers.

---

# 6. Why This Example Matters for the Research Task

Pedro’s assignment requires:

1. **Understanding Burst middleware**
2. **Running example applications**
3. **Implementing your own small communication program**
4. **Later modifying the K-means version**

This example directly satisfies:

✔ Running Burst code
✔ Understanding worker communication
✔ Understanding backend configuration
✔ Navigating the middleware API
✔ Preparing for distributed K-means and balanced K-means

This is the foundational building block before implementing:

* `burst_kmeans.rs`
* modified balanced K-means
* performance evaluation

---

# 7. Next Steps

After this document, the next files I will create:

### 📄 `docs/burst_overview.md`

Deep explanation of architecture (scheduler, channels, batching, etc.)

### 📄 `docs/burst_point_to_point.md`

My own simplified messaging program using Burst.

### 📄 `docs/burst_kmeans_analysis.md`

Analysis + diagrams for the K-means implementation.

### 📄 `docs/balanced_kmeans_plan.md`

How I will implement balanced K-means.

---

# 8. Author

**Diba Mtd**
Distributed Systems & Cloud Computing Research
GitHub: [https://github.com/dibamtd](https://github.com/dibamtd)
Email: [diba.mo72@gmail.com](mailto:diba.mo72@gmail.com)

---

This document is part of my preparation for
**PhD research under Pedro García López (URV)**
and the Burst Computing project.

```

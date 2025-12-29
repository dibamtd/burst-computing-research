
# Presentation Outline (10 Minutes)

## 1. Motivation: Why Burst Computing? (1–2 min)
- Limitations of FaaS:
  - isolated workers
  - no group execution
  - high orchestration overhead
- Need for synchronous, parallel execution in distributed algorithms

---

## 2. Burst Computing in One Slide (1 min)
- Group invocation: **flare**
- Burst size and worker packing
- Exploiting locality while remaining serverless-compatible

---

## 3. Burst Communication Middleware (BCM) (2 min)
- Role of BCM in the Burst stack
- MPI-like communication semantics
- Local vs remote communication paths
- Backend abstraction (RabbitMQ / Redis)

---

## 4. Hands-on Validation: What I Ran (2 min)
- Local setup (WSL + Docker + RabbitMQ)
- Hello-world example
- Pair microbenchmark execution
- Observed behavior and correctness

---

## 5. Key Observations (2 min)
- Group execution enables coordination
- Indirect communication preserves portability
- Semantics matter more than raw performance

---

## 6. Research Questions That Emerged (1–2 min)
- Strength of flare guarantees (simultaneity, failure)
- Exact communication semantics of BCM
- Backpressure and scalability limits

---

## 7. Next Steps (1 min)
- Analyze k-means on top of Burst
- Study communication patterns vs burst size
- Explore balanced k-means as an extension

---

## Closing
- Focus on understanding and correctness
- This repository serves as a basis for deeper research

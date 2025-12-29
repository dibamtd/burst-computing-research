
# Live Coding – Point-to-Point BCM (Plan + Research Questions)

## Goal
In the live coding session, I expect to implement a **minimal point-to-point**
example using the Burst Communication Middleware (BCM).

This is meant to demonstrate:
- correct BCM usage (send/recv)
- understanding of worker roles (`worker_id`)
- ability to reason about execution flow and edge cases

---

## Planned Minimal Demo
Two workers participate:

- `worker_id == 1`: sender
- `worker_id == 0`: receiver

Expected behavior:
- sender sends a small message (e.g., a string or bytes)
- receiver receives it and prints/logs it

---

## Logic Sketch (Pseudocode)

```text
init BCM

if worker_id == 0:
    msg = recv(from worker 1)
    print(msg)

else if worker_id == 1:
    send("hello", to worker 0)
````

---

## What I Want to Clarify (Research Questions That Came Up)

While reading the paper and running the microbenchmarks, I realized that the
correctness of distributed algorithms depends heavily on the *strength* of the
communication and group-execution guarantees. These questions became my main
challenges:

1️⃣ Flare guarantee

If one worker in a flare is delayed or fails to start, what happens to the flare execution?
Does it fail, wait, or recover?

2️⃣ BCM semantics

What delivery and ordering guarantees does BCM provide?
Are they backend-independent or backend-specific?

3️⃣ Backpressure

If the receiver is slow, how does BCM handle buffering and backpressure?
Where do messages accumulate?

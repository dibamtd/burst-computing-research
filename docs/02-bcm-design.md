# Burst Communication Middleware (BCM)

## Role in the Burst Stack
BCM provides MPI-like message passing within a burst execution.

## Communication Model
- Point-to-point (send/recv)
- Collective operations (broadcast, gather, all-to-all, reduce)

## Local vs Remote
- Intra-pack: local communication (e.g., tokio/shared memory)
- Inter-pack: indirect communication via backends (RabbitMQ/Redis)

## Relation to MPI
| MPI | Burst |
|---|---|
| MPI_Comm_rank | worker_id |
| MPI_Comm_size | burst_size |
| MPI_Send/Recv | pair |
| MPI_Bcast | broadcast |


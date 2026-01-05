use burst_communication_middleware::{
    BurstMiddleware, BurstOptions, Middleware, RabbitMQMImpl, RabbitMQOptions, TokioChannelImpl,
    TokioChannelOptions,
};
use bytes::Bytes;
use log::info;
use std::{
    collections::{HashMap, HashSet},
    thread,
};

/// A minimal message type used for point-to-point communication.
/// BCM transports messages as Bytes, so we provide conversions.
#[derive(Debug, Clone)]
struct StringMessage(String);

impl From<Bytes> for StringMessage {
    fn from(bytes: Bytes) -> Self {
        StringMessage(String::from_utf8_lossy(&bytes).to_string())
    }
}

impl From<StringMessage> for Bytes {
    fn from(val: StringMessage) -> Self {
        Bytes::from(val.0)
    }
}

fn main() {
    env_logger::init();

    // One group containing workers {0,1}.
    let group_ranges: HashMap<String, HashSet<u32>> =
        vec![(0.to_string(), vec![0, 1].into_iter().collect())]
            .into_iter()
            .collect();

    // Tokio runtime used by BCM internals (async IO).
    let tokio_runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    // Create two worker proxies using:
    // - TokioChannelImpl for local communication
    // - RabbitMQMImpl for indirect communication (backend)
    let fut = tokio_runtime.spawn(BurstMiddleware::create_proxies::<
        TokioChannelImpl,
        RabbitMQMImpl,
        _,
        _,
    >(
        BurstOptions::new(2, group_ranges, 0.to_string())
            .burst_id("p2p_demo".to_string())
            .enable_message_chunking(true)
            .message_chunk_size(4 * 1024 * 1024)
            .build(),
        TokioChannelOptions::new()
            .broadcast_channel_size(256)
            .build(),
        RabbitMQOptions::new("amqp://guest:guest@localhost:5672".to_string())
            .durable_queues(true)
            .ack(true)
            .build(),
    ));

    let proxies = tokio_runtime.block_on(fut).unwrap().unwrap();

    // Wrap proxies with Middleware<T> so we can obtain an actor handle.
    let mut actors = proxies
        .into_iter()
        .map(|(worker_id, middleware)| {
            (
                worker_id,
                Middleware::new(middleware, tokio_runtime.handle().clone()),
            )
        })
        .collect::<HashMap<u32, Middleware<StringMessage>>>();

    let w0 = actors.remove(&0).unwrap();
    let w1 = actors.remove(&1).unwrap();

    // Run workers concurrently (local validation uses threads).
    let t0 = thread::spawn(move || run_worker(w0));
    let t1 = thread::spawn(move || run_worker(w1));

    t0.join().unwrap();
    t1.join().unwrap();
}

fn run_worker(mw: Middleware<StringMessage>) {
    let h = mw.get_actor_handle();
    let id = h.info.worker_id;

    info!("worker start: id={}", id);

    if id == 0 {
        // Worker 0: send -> recv (ACK)
        let msg_id = 1;
        let payload = format!("msg_id={} hello from worker 0", msg_id);

        info!("worker {} sending to 1: {}", id, payload);
        h.send(1, StringMessage(payload)).unwrap();

        let reply = h.recv(1).unwrap();
        info!("worker {} received reply: {:?}", id, reply);

        // Small correctness check: ensure ACK refers to our msg_id.
        if reply.0.contains("msg_id=1") {
            info!("worker {}: ACK matches msg_id=1", id);
        } else {
            info!("worker {}: ACK does NOT match msg_id=1", id);
        }
    } else if id == 1 {
        // Worker 1: recv -> send (ACK)
        let msg = h.recv(0).unwrap();
        info!("worker {} received: {:?}", id, msg);

        let ack = format!("ack for {}", msg.0);
        info!("worker {} replying to 0: {}", id, ack);
        h.send(0, StringMessage(ack)).unwrap();
    }

    info!("worker end: id={}", id);
}

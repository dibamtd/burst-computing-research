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

// kmeans crate (در repo شما اسم package داخل kmeans/kmeans معمولاً "actions" است)
use actions::{kmeans_burst, Input, S3Config};

fn main() {
    env_logger::init();

    // اگر در lib.rs مسیر local دارید/اضافه کردید، این باعث میشه S3 لازم نباشه
    std::env::set_var("KMEANS_LOCAL", "1");

    // One group containing two workers: {0, 1}
    let group_ranges: HashMap<String, HashSet<u32>> =
        vec![(0.to_string(), vec![0, 1].into_iter().collect())]
            .into_iter()
            .collect();

    // Tokio runtime used internally by BCM for async operations
    let tokio_runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    // Create worker proxies (همان الگوی hello_world_local)
    let fut = tokio_runtime.spawn(BurstMiddleware::create_proxies::<
        TokioChannelImpl,
        RabbitMQMImpl,
        _,
        _,
    >(
        BurstOptions::new(2, group_ranges, 0.to_string())
            .burst_id("kmeans_local".to_string())
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

    // Wrap proxies so we can obtain a synchronous actor handle per worker
    let mut actors = proxies
        .into_iter()
        .map(|(worker_id, middleware)| {
            (
                worker_id,
                Middleware::new(middleware, tokio_runtime.handle().clone()),
            )
        })
        .collect::<HashMap<u32, Middleware<Bytes>>>();

    let w0 = actors.remove(&0).unwrap();
    let w1 = actors.remove(&1).unwrap();

    // Run both workers concurrently (local validation uses OS threads)
    let t0 = thread::spawn(move || run_worker(w0));
    let t1 = thread::spawn(move || run_worker(w1));

    t0.join().unwrap();
    t1.join().unwrap();
}

fn run_worker(mw: Middleware<Bytes>) {
    let h = mw.get_actor_handle();
    let id = h.info.worker_id;

    info!("kmeans worker start: id={}", id);

    // input minimal (اگر KMEANS_LOCAL فعال باشد، نباید S3 بخواهد)
    let input = Input {
        bucket: "local".into(),
        key: "kmeans".into(),
        s3_config: S3Config {
            region: "local".into(),
            endpoint: "local".into(),
            aws_access_key_id: "local".into(),
            aws_secret_access_key: "local".into(),
        },
        threshold: 0.001,
        num_dimensions: 2,
        num_clusters: 5,
        max_iterations: 5,
    };

    let out = kmeans_burst(input, h);
    info!("kmeans worker end: id={}, out={:?}", id, out);
}

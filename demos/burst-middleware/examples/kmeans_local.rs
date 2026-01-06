use burst_communication_middleware::{
    BurstMiddleware, BurstOptions, Middleware, RabbitMQMImpl, RabbitMQOptions, TokioChannelImpl,
    TokioChannelOptions,
};
use bytes::Bytes;
use rand::{Rng, SeedableRng};
use std::{
    collections::{HashMap, HashSet},
    thread,
};

#[derive(Debug, Clone)]
struct BytesMessage(Bytes);

impl From<Bytes> for BytesMessage {
    fn from(b: Bytes) -> Self {
        BytesMessage(b)
    }
}
impl From<BytesMessage> for Bytes {
    fn from(m: BytesMessage) -> Self {
        m.0
    }
}

// ---- Safe helpers (no unsafe) ----
fn f32_to_msg(v: &[f32]) -> BytesMessage {
    let b: &[u8] = bytemuck::cast_slice(v);
    BytesMessage(Bytes::copy_from_slice(b))
}

fn msg_to_f32(m: &BytesMessage) -> Vec<f32> {
    let v: &[f32] = bytemuck::cast_slice(m.0.as_ref());
    v.to_vec()
}

fn u32_to_msg(v: &[u32]) -> BytesMessage {
    let b: &[u8] = bytemuck::cast_slice(v);
    BytesMessage(Bytes::copy_from_slice(b))
}

fn msg_to_u32(m: &BytesMessage) -> Vec<u32> {
    let v: &[u32] = bytemuck::cast_slice(m.0.as_ref());
    v.to_vec()
}

fn main() {
    env_logger::init();

    // One group containing two workers: {0, 1}
    let group_ranges: HashMap<String, HashSet<u32>> =
        vec![(0.to_string(), vec![0, 1].into_iter().collect())]
            .into_iter()
            .collect();

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    let proxies = rt
        .block_on(BurstMiddleware::create_proxies::<
            TokioChannelImpl,
            RabbitMQMImpl,
            TokioChannelOptions,
            RabbitMQOptions,
        >(
            BurstOptions::new(2, group_ranges, 0.to_string())
                .burst_id("kmeans_local".to_string())
                .build(),
            TokioChannelOptions::new().build(),
            RabbitMQOptions::new("amqp://guest:guest@127.0.0.1:5672".to_string())
                .ack(true)
                .durable_queues(true)
                .build(),
        ))
        .unwrap();

    let mut workers = proxies
        .into_iter()
        .map(|(id, mw)| (id, Middleware::new(mw, rt.handle().clone())))
        .collect::<HashMap<_, _>>();

    let w0 = workers.remove(&0).unwrap();
    let w1 = workers.remove(&1).unwrap();

    let t0 = thread::spawn(move || run_worker(w0));
    let t1 = thread::spawn(move || run_worker(w1));

    t0.join().unwrap();
    t1.join().unwrap();
}

fn run_worker(mw: Middleware<BytesMessage>) {
    let h = mw.get_actor_handle();
    let id = h.info.worker_id;

    // Local synthetic dataset per worker
    let n_points: usize = 1000;
    let dims: usize = 2;
    let k: usize = 5;
    let iters: usize = 5;

    let mut rng = rand::rngs::StdRng::seed_from_u64(42 + id as u64);
    let mut points: Vec<f32> = Vec::with_capacity(n_points * dims);
    for _ in 0..(n_points * dims) {
        points.push(rng.gen_range(0.0..100.0));
    }

    // Root initializes centroids
    let root: u32 = 0;
    let mut centroids: Vec<f32> = vec![0.0; k * dims];
    if id == root {
        for c in centroids.iter_mut() {
            *c = rng.gen_range(0.0..100.0);
        }
    }

    for _ in 0..iters {
        // Broadcast centroids (root -> all)
        if id == root {
            let msg = h
                .broadcast(Some(f32_to_msg(&centroids)), root)
                .unwrap();
            centroids = msg_to_f32(&msg);
        } else {
            let msg = h.broadcast(None, root).unwrap();
            centroids = msg_to_f32(&msg);
        }

        // Compute local sums + counts
        let mut local_sum = vec![0.0f32; k * dims];
        let mut local_cnt = vec![0u32; k];

        for p in 0..n_points {
            let px = points[p * dims];
            let py = points[p * dims + 1];

            let mut best = 0usize;
            let mut best_dist = f32::MAX;
            for ci in 0..k {
                let cx = centroids[ci * dims];
                let cy = centroids[ci * dims + 1];
                let d = (px - cx) * (px - cx) + (py - cy) * (py - cy);
                if d < best_dist {
                    best_dist = d;
                    best = ci;
                }
            }

            local_sum[best * dims] += px;
            local_sum[best * dims + 1] += py;
            local_cnt[best] += 1;
        }

        // Gather sums + counts to root
        let gathered_sums = h.gather(f32_to_msg(&local_sum), root).unwrap();
        let gathered_cnts = h.gather(u32_to_msg(&local_cnt), root).unwrap();

        if id == root {
            let sums_vec = gathered_sums.unwrap();
            let cnts_vec = gathered_cnts.unwrap();

            let mut sum_all = vec![0.0f32; k * dims];
            let mut cnt_all = vec![0u32; k];

            for m in sums_vec {
                let v = msg_to_f32(&m);
                for i in 0..sum_all.len() {
                    sum_all[i] += v[i];
                }
            }

            for m in cnts_vec {
                let v = msg_to_u32(&m);
                for i in 0..cnt_all.len() {
                    cnt_all[i] += v[i];
                }
            }

            for ci in 0..k {
                let c = cnt_all[ci].max(1) as f32;
                centroids[ci * dims] = sum_all[ci * dims] / c;
                centroids[ci * dims + 1] = sum_all[ci * dims + 1] / c;
            }
        }
    }

    if id == 0 {
        println!("[kmeans_local] done. final centroids = {:?}", centroids);
    }
}

use std::time::{Duration, Instant};

use tklog::{LEVEL, LOG};

fn log_init() {
    LOG.set_level(LEVEL::Trace).set_console(false).set_cutmode_by_size("bench_log.log", 1 << 30, 0, false);
    LOG.set_printmode(tklog::PRINTMODE::PUNCTUAL);
}

#[test]
fn bench_log() {
    log_init();
    let iterations = 10_000; // Total number of executions
    let batch_size = 10; // Number of logs in each batch
    let mut total_duration = Duration::new(0, 0); // Total time

    // Warm-up phase
    for _ in 0..batch_size {
        tklog::debug!("debug!", "this is sync log");
    }

    // Official test phase
    let start_total = Instant::now(); // Record the start time of the test

    for _ in 0..(iterations / batch_size) {
        let start_batch = Instant::now(); // Record the batch start time

        for _ in 0..batch_size {
            tklog::debug!("debug!", "this is sync log");
        }

        let batch_duration = start_batch.elapsed(); // Calculate the batch duration
        total_duration += batch_duration; // Accumulate the batch duration
    }

    // Calculate total duration
    let total_elapsed = start_total.elapsed();
    std::thread::sleep(Duration::from_secs(1));
    // Statistics and print results
    let avg_duration = total_duration / iterations as u32; // Average execution time per operation
    println!("Total logs: {}", iterations);
    println!("Total batch time: {:.2?}", total_duration);
    println!("Average log time per batch: {:.2?}", total_elapsed / (iterations / batch_size) as u32);
    println!("Average log time per operation: {:.2?}", avg_duration);
}

use tokio::task;

async fn parallel_logging(batch_size: usize, iterations: usize) -> Duration {
    let start_batch = Instant::now();
    for _ in 0..iterations {
        for _ in 0..batch_size {
            tklog::debug!("debug!", "this is sync log");
        }
    }
    start_batch.elapsed()
}

#[tokio::test]
async fn bench_log_parallel() {
    log_init();
    let iterations = 1_000; // Total number of executions
    let batch_size = 100; // Number of logs per batch
    let num_tasks = 10; // Number of parallel tasks

    let task_iterations = iterations / (num_tasks * batch_size);

    let start_total = Instant::now();
    let mut handles = vec![];
    for _ in 0..num_tasks {
        let handle = task::spawn(parallel_logging(batch_size, task_iterations));
        handles.push(handle);
    }
    let mut total_duration = Duration::new(0, 0);
    for handle in handles {
        let task_duration = handle.await.unwrap();
        total_duration += task_duration;
    }
    let total_elapsed = start_total.elapsed();
    let avg_duration = total_duration / iterations as u32; // Average execution time per operation
    println!("Total logs: {}", iterations);
    println!("Total time: {:.2?}", total_elapsed);
    println!("Average log time per task: {:.2?}", total_duration / num_tasks as u32);
    println!("Average log time per operation: {:.2?}", avg_duration);
}

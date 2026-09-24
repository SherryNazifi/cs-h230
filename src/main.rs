fn process_job_id(job_id: String) {
    println!("Processing job: {}", job_id);
}

fn use_worker_limit(max_workers: i32) {
    println!("Maximum workers: {}", max_workers);
}

fn main() {
    let job_id = String::from("JOB-001");

    process_job_id(job_id);

    // This would NOT work because ownership moved:
    // println!("{}", job_id);

    let max_workers: i32 = 4;

    use_worker_limit(max_workers);

    println!("Worker limit is still: {}", max_workers);
}
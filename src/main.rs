mod bit_flags;
fn process_job_id(job_id: String) {
    println!("Processing job: {}", job_id);
}

fn use_worker_limit(max_workers: i32) {
    println!("Maximum workers: {}", max_workers);
}

fn main() {
    // let job_id = String::from("JOB-001");

    // process_job_id(job_id);

    // // This would NOT work because ownership moved:
    // //println!("{}", job_id);

    // let max_workers: i32 = 4;

    // use_worker_limit(max_workers);

    // println!("Worker limit is still: {}", max_workers);
    let state: u32 = 1 | (0x1234 << 3) | (1 << 19);

    println!("Active: {}", bit_flags::is_active(state));
    println!("Completed: {}", bit_flags::is_completed(state));
    println!("Failed: {}", bit_flags::has_failed(state));
    println!("Job number: {}", bit_flags::get_job_number(state));
    println!("Big endian: {}", bit_flags::is_big_endian(state));
    println!("Job bytes: {:02X?}", bit_flags::get_job_number_bytes(state));
}


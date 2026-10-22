mod bit_flags;
mod job_lifetimes;

use job_lifetimes::{
    Job, JobAssignment, Worker, get_worker_job, same_job, show_assignment, show_job, show_worker,
};

fn process_job_id(job_id: String) {
    println!("Processing job: {}", job_id);
}

fn use_worker_limit(max_workers: i32) {
    println!("Maximum workers: {}", max_workers);
}

fn main() {
    // ----------------------------------------
    // Previous assignment: Ownership
    // ----------------------------------------

    // let job_id = String::from("JOB-001");

    // process_job_id(job_id);

    // This would NOT work because ownership moved:
    // println!("{}", job_id);

    // let max_workers: i32 = 4;

    // use_worker_limit(max_workers);

    // println!("Worker limit is still: {}", max_workers);

    // ----------------------------------------
    // Previous assignment: Bit Flags
    // ----------------------------------------

    let state: u32 = 1 | (0x1234 << 3) | (1 << 19);

    println!("Active: {}", bit_flags::is_active(state));
    println!("Completed: {}", bit_flags::is_completed(state));
    println!("Failed: {}", bit_flags::has_failed(state));
    println!("Job number: {}", bit_flags::get_job_number(state));
    println!("Big endian: {}", bit_flags::is_big_endian(state));
    println!("Job bytes: {:02X?}", bit_flags::get_job_number_bytes(state));

    // ----------------------------------------
    // New assignment: Structs and Lifetimes
    // ----------------------------------------

    println!("\n--- Structs and Lifetimes ---");

    // Create two jobs in the outer scope
    // These jobs will live longer than the workers

    let job1 = Job {
        id: String::from("JOB-001"),
        description: String::from("Process uploaded data"),
        estimated_time: 30,
    };

    let job2 = Job {
        id: String::from("JOB-002"),
        description: String::from("Generate report"),
        estimated_time: 20,
    };

    // Pass references to jobs without moving ownership
    show_job(&job1);
    show_job(&job2);

    // Inner scope for workers
    {
        // Each worker borrows a job
        // The workers do not own the jobs

        let worker1 = Worker {
            id: 1,
            name: String::from("Alice"),
            job: &job1,
        };

        let worker2 = Worker {
            id: 2,
            name: String::from("Bob"),
            job: &job2,
        };

        // Display information about both workers
        show_worker(&worker1);
        show_worker(&worker2);

        // Get a reference to the job assigned to worker1
        let borrowed_job = get_worker_job(&worker1);

        println!("Borrowed job: {}", borrowed_job.id);

        // Check if the workers reference the same job
        println!("Same job: {}", same_job(&worker1, &worker2));

        // A second inner scope for the job assignment
        {
            // The assignment borrows worker1
            // Worker1 also borrows job1
            // Both references must remain valid

            let assignment = JobAssignment {
                worker: &worker1,
                assigned_time: 15,
            };

            // Pass the assignment to a function
            show_assignment(&assignment);

            println!("Assignment time: {} minutes", assignment.assigned_time);
        }

        // The assignment is now out of scope
        // The workers are still valid here

        println!("Worker still exists: {}", worker1.name);
        println!("Worker ID: {}", worker1.id);
        println!("Other worker ID: {}", worker2.id);
    }

    // Both workers are now out of scope
    // The jobs still exist because they were created outside

    println!("Job 1 still exists: {}", job1.id);
    println!("Job 2 still exists: {}", job2.id);

    // We can still access the job information
    println!("Job 1 description: {}", job1.description);
    println!("Job 2 estimated time: {} minutes", job2.estimated_time);
}

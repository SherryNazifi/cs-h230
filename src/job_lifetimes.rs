// Stores information about a job
pub struct Job {
    pub id: String,
    pub description: String,
    pub estimated_time: u32,
}

// A worker borrows a reference to an existing job
// 'a represents how long the job reference is valid
pub struct Worker<'a> {
    pub id: u32,
    pub name: String,
    pub job: &'a Job,
}

// An assignment borrows a worker
// 'a is the lifetime of the job reference inside the worker
// 'b is the lifetime of the reference to the worker
pub struct JobAssignment<'a, 'b> {
    pub worker: &'b Worker<'a>,
    pub assigned_time: u32,
}

// Takes a job reference without taking ownership
pub fn show_job(job: &Job) {
    println!("Job: {}", job.id);
    println!("Description: {}", job.description);
    println!("Estimated time: {} minutes", job.estimated_time);
}

// Takes a worker that contains a reference to a job
pub fn show_worker<'a>(worker: &Worker<'a>) {
    println!("Worker: {}", worker.name);
    println!("Job: {}", worker.job.id);
}

// Returns a reference to the job borrowed by a worker
pub fn get_worker_job<'a>(worker: &Worker<'a>) -> &'a Job {
    worker.job
}

// Takes an assignment containing a reference to a worker
pub fn show_assignment<'a, 'b>(assignment: &JobAssignment<'a, 'b>) {
    println!("Worker: {}", assignment.worker.name);
    println!("Job: {}", assignment.worker.job.id);
    println!("Assigned time: {}", assignment.assigned_time);
}

// Compares the job references held by two workers
pub fn same_job<'a, 'b>(first: &Worker<'a>, second: &Worker<'b>) -> bool {
    std::ptr::eq(first.job, second.job)
}

// Tests for structs, borrowing, and lifetimes
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_job_fields() {
        let job = Job {
            id: String::from("JOB-001"),
            description: String::from("Process data"),
            estimated_time: 30,
        };

        assert_eq!(job.id, "JOB-001");
        assert_eq!(job.estimated_time, 30);
    }

    #[test]
    fn test_worker_borrows_job() {
        let job = Job {
            id: String::from("JOB-002"),
            description: String::from("Generate report"),
            estimated_time: 20,
        };

        let worker = Worker {
            id: 1,
            name: String::from("Alice"),
            job: &job,
        };

        assert_eq!(worker.job.id, job.id);
        assert_eq!(worker.name, "Alice");
    }

    #[test]
    fn test_job_outlives_worker() {
        let job = Job {
            id: String::from("JOB-003"),
            description: String::from("Upload file"),
            estimated_time: 10,
        };

        {
            let worker = Worker {
                id: 2,
                name: String::from("Bob"),
                job: &job,
            };

            assert_eq!(worker.job.id, "JOB-003");
        }

        // Worker no longer exists, but job is still valid
        assert_eq!(job.id, "JOB-003");
    }

    #[test]
    fn test_get_worker_job() {
        let job = Job {
            id: String::from("JOB-004"),
            description: String::from("Analyze data"),
            estimated_time: 45,
        };

        let worker = Worker {
            id: 3,
            name: String::from("Charlie"),
            job: &job,
        };

        let borrowed_job = get_worker_job(&worker);

        assert_eq!(borrowed_job.id, "JOB-004");
    }

    #[test]
    fn test_multiple_workers_borrow_job() {
        let job = Job {
            id: String::from("JOB-005"),
            description: String::from("Shared task"),
            estimated_time: 60,
        };

        let worker1 = Worker {
            id: 4,
            name: String::from("Alice"),
            job: &job,
        };

        let worker2 = Worker {
            id: 5,
            name: String::from("Bob"),
            job: &job,
        };

        assert!(same_job(&worker1, &worker2));
    }

    #[test]
    fn test_nested_lifetimes() {
        let job = Job {
            id: String::from("JOB-006"),
            description: String::from("Process request"),
            estimated_time: 15,
        };

        {
            let worker = Worker {
                id: 6,
                name: String::from("David"),
                job: &job,
            };

            {
                let assignment = JobAssignment {
                    worker: &worker,
                    assigned_time: 10,
                };

                assert_eq!(assignment.worker.job.id, job.id);
                assert_eq!(assignment.assigned_time, 10);
            }

            // Assignment is gone, but worker still exists
            assert_eq!(worker.name, "David");
        }

        // Worker is gone, but job still exists
        assert_eq!(job.id, "JOB-006");
    }
}

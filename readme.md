## Project Pitch

I plan to build a task processing server. Clients will be able to submit jobs to the server, check the status of those jobs, and receive results once processing is complete.

The server will maintain a queue of pending jobs and process them using worker threads. As the project develops, I can use a linked data structure to implement the job queue, multithreading to allow multiple jobs to be processed concurrently, and a client-server architecture so remote clients can communicate with the server.

## Ownership Experiment

I used a `String` to represent a job ID and an `i32` to represent the maximum number of worker threads.

When the `String` is passed into a function by value, ownership moves into that function. The original variable can no longer be used afterward.

The `i32` behaves differently because it implements the `Copy` trait. Passing it into a function copies the value, so the original variable remains valid and can still be used.

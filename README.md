# Task Engine

This program was made alongside Marek Pinto as part of our assignment for CS3211 (Parallel and Concurrent Programming) at the National University of Singapore during the Spring 2023 semester. For a more in-depth explanation of our project, please see the attached pdf.

A task runner is a program that schedules and executes tasks. For simplicity, there are only three types of tasks. Note that the implementation of the tasks are provided; the focus of the project is to execute tasks concurrently, and not to parallelise the computation within a task.

For this assignment, a set of initial tasks is generated from an initial seed. Each task may produce a number of further tasks to be executed. The output of all tasks is combined to produce a single output value at the end. For simplicity, the output is combined in a manner that is independent of order.

There are three types of tasks:

Hash: generate 32 bytes, then repeatedly SHA256 hash the 32 bytes a random number of times, with the output of each hash iteration being the input of the next iteration. The output u64 is taken from a random 8-byte slice of the final hash output.
Derive: generate a 32-byte secret and 32-byte salt, then run the PBKDF2 key derivation function for a random number of iterations to generate a 64-byte key. The output u64 is taken from a random 8-byte slice of the final key output.
Random: the RNG is sampled a random number of times, and the last sample is returned as the result.

The goal of this assignment was to optimize memory usage rather than performance. Using larger heights makes the program very slow very fast, so we were instructed to focus on optimizing memory usage with Tokio.

# How to Use

To build, simply run cargo build. You should specify the -r flag when testing performance, so that compiler optimisations are enabled.

To execute the program, run cargo run -r <seed>
  
Examples:
  
$ cargo run -r 5664168989938163334
Using seed 5664168989938163334, starting height 5, max. children 5
Completed in 58.200392774 s
8229144459996529628,825,876,874

$ cargo run -r 1976915708242608314
Using seed 1976915708242608314, starting height 5, max. children 5
Completed in 80.852794384 s
8889898106685444821,1173,1215,1173

$ cargo run -r 12605174704058567923
Using seed 12605174704058567923, starting height 5, max. children 5
Completed in 41.325396062 s
12607843883509729997,623,618,649

## Inputs

The task runner takes three parameters:

initial seed: this is used to generate the initial task set, which has between 0 and 64 tasks
initial height: the height of the tasks in the initial task set; child tasks get a height one less than their parent task’s height, and a task with height 0 will generate no further tasks
max. children: the maximum number of child tasks that result from each task
  
Only the initial seed is given for input in the command line.

## Outputs

The task runner should print to standard output, on a single line:

<combined output>,<number of Hash tasks>,<number of Derive tasks>,<number of Random tasks>
An example of printing the output is provided in the provided sequential implementation.

The combined output is the XOR of all task outputs.

# Explanation of Concurrency

Each task is broken into two phases: execute() and XOR/enqueue. XOR/enqueue rely on shared data, so we can not safely make it run in parallel. The data enqueued is also reliant on the result of execute(), so the two phases must be serialized, but only one of them can run asynchronously.

On each iteration, we first complete the second phase for any tasks that have finished their first phase asynchronously and sent their latest TaskResult via the channel. Once this is complete, the program then starts the first phase on a worker thread for the pulled task. The loop will keep running, but at some point execute() for the pulled task will finish on the worker thread.

When the first phase finishes, it will send its TaskResult via the channel. On the next iteration of the loop, the second phase will execute on the main thread. Our program effectively maintains serialization for each task, but allows for the concurrent execution of multiple task executions at the same time.

Please see the attached pdf for an in-depth explanation of the project parameters and our product.

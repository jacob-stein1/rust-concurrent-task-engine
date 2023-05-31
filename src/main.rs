use std::{
    collections::{HashMap, VecDeque},
    time::Instant,
};

use peak_alloc::PeakAlloc;

#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;


pub type TaskResult = (u64, Vec<Task>);

use task::{Task, TaskType};
use tokio::sync::mpsc;
use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let (seed, starting_height, max_children) = get_args();

    eprintln!(
        "Using seed {}, starting height {}, max. children {}",
        seed, starting_height, max_children
    );

    let mut count_map = HashMap::new();
    let mut taskq = VecDeque::from(Task::generate_initial(seed, starting_height, max_children));

    let mut output: u64 = 0;
    let mut num_background: u64 = 0;

    let (tx, mut rx) = mpsc::channel::<TaskResult>(2000);
    

    let start = Instant::now();
    while (!taskq.is_empty()) || (num_background > 0) {
        while let Ok(result) = rx.try_recv() {
            num_background -= 1;
            output ^= result.0;
            taskq.extend(result.1.into_iter());
        }

        let Some(next) = taskq.pop_front() else { thread::sleep(Duration::from_millis(50)); continue; };
        *count_map.entry(next.typ).or_insert(0usize) += 1;


        let tx = tx.clone();
        num_background += 1;

        tokio::spawn( async move { 
            let result = next.execute(); 
            tx.send(result).await.unwrap(); //might need await here
        } );


        
    }
    let end = Instant::now();

    eprintln!("Completed in {} s", (end - start).as_secs_f64());
    let peak_mem = PEAK_ALLOC.peak_usage_as_gb();
	println!("The max amount that was used {}", peak_mem);

    println!(
        "{},{},{},{}",
        output,
        count_map.get(&TaskType::Hash).unwrap_or(&0),
        count_map.get(&TaskType::Derive).unwrap_or(&0),
        count_map.get(&TaskType::Random).unwrap_or(&0)
    );
}

// There should be no need to modify anything below

fn get_args() -> (u64, usize, usize) {
    let mut args = std::env::args().skip(1);
    (
        args.next()
            .map(|a| a.parse().expect("invalid u64 for seed"))
            .unwrap_or_else(|| rand::Rng::gen(&mut rand::thread_rng())),
        args.next()
            .map(|a| a.parse().expect("invalid usize for starting_height"))
            .unwrap_or(5),
        args.next()
            .map(|a| a.parse().expect("invalid u64 for seed"))
            .unwrap_or(5),
    )
}

mod task;

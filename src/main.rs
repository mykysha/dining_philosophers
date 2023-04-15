use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const PHILOSOPHERS: usize = 15;
const EAT_COUNT: usize = 5;

struct Philosopher {
    id: usize,
    left_fork: Arc<Mutex<()>>,
    right_fork: Arc<Mutex<()>>,
}

impl Philosopher {
    fn new(id: usize, left_fork: Arc<Mutex<()>>, right_fork: Arc<Mutex<()>>) -> Philosopher {
        Philosopher {
            id,
            left_fork,
            right_fork,
        }
    }

    fn eat(&self) {
        println!("Philosopher {} is eating.", self.id);
        thread::sleep(Duration::from_millis(1000));
    }

    fn think(&self) {
        println!("Philosopher {} is thinking.", self.id);
        thread::sleep(Duration::from_millis(1000));
    }

    fn dine(&self) {
        for eat_number in 0..EAT_COUNT {
            let (first_fork, second_fork) = if self.id % 2 == 0 {
                (&self.right_fork, &self.left_fork)
            } else {
                (&self.left_fork, &self.right_fork)
            };

            let _first_fork = first_fork.lock().unwrap();
            let _second_fork = second_fork.lock().unwrap();

            self.eat();
            println!(
                "Philosopher {} is done eating {} times.",
                self.id,
                eat_number + 1
            );
            self.think();
        }
    }
}

fn main() {
    let forks: Vec<Arc<Mutex<()>>> = (0..PHILOSOPHERS)
        .map(|_| Arc::new(Mutex::new(())))
        .collect();

    let philosophers: Vec<_> = (0..PHILOSOPHERS)
        .map(|i| {
            let left_fork = forks[i].clone();
            let right_fork = forks[(i + 1) % PHILOSOPHERS].clone();
            Philosopher::new(i + 1, left_fork, right_fork)
        })
        .collect();

    let threads: Vec<_> = philosophers
        .into_iter()
        .map(|p| {
            thread::spawn(move || {
                p.dine();
            })
        })
        .collect();

    for t in threads {
        t.join().unwrap();
    }
}

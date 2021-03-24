use crossbeam_channel;
use std::{thread, time};
use std::sync::{Arc, Mutex};


fn parallel_map<T, U, F>(mut input_vec: Vec<T>, num_threads: usize, f: F) -> Vec<U>
where
    F: FnOnce(T) -> U + Send + Copy + 'static,
    T: Send + 'static +,
    U: Send + 'static + Default + std::clone::Clone,
{
    let mut output_vec: Vec<U> = Vec::with_capacity(input_vec.len());
    let mut rc_output_vec = Arc::new(Mutex::new(output_vec));

    // TODO: implement parallel map!
    let (sender, receiver) = crossbeam_channel::unbounded();
    let mut threads = Vec::new();
    for _ in 0..num_threads {
        let receiver = receiver.clone();
        let rc_output_vec_cloned = rc_output_vec.clone();
        threads.push(thread::spawn(move || {
            while let Ok(input_num) = receiver.recv() {
                // let (input_num, index) = sent_value;
                let output = f(input_num);
                rc_output_vec_cloned.lock().unwrap().push(output);
            }
        }))
    }

    loop{
        let input_num = input_vec.pop();
        match input_num {
            Some(input_num) => sender.send(input_num).expect("No receivers!"),
            None => break

        }        
    }

    drop(sender);
    for thread in threads {
        thread.join().expect("Thread panicked!");
    }
    let x = rc_output_vec.lock().unwrap().to_vec(); x
}

fn main() {
    let num_threads = num_cpus::get();
    let v = vec![6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 12, 18, 11, 5, 20];
    let squares = parallel_map(v, 10, |num| {
        println!("{} squared is {}", num, num * num);
        thread::sleep(time::Duration::from_millis(500));
        num * num
    });
    println!("squares: {:?}", squares);
}

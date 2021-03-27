use crossbeam_channel;
use std::{thread, time};



fn parallel_map<T, U, F>(mut input_vec: Vec<T>, num_threads: usize, f: F) -> Vec<U>
where
    F: FnOnce(T) -> U + Send + Copy + 'static + std::marker::Sync,
    T: Send + 'static,
    U: Send + 'static + Default + std::clone::Clone + std::fmt::Debug,
{   
    let mut output_vec: Vec<U> = Vec::with_capacity(input_vec.len());    
   
    // input channel
    let (sender, receiver) = crossbeam_channel::unbounded();
    // output channel
    let (sender2, receiver2) = crossbeam_channel::bounded(input_vec.len());
    let mut threads = Vec::new();
    for _ in 0..num_threads {
        let receiver = receiver.clone();
        let sender2 = sender2.clone();
        threads.push(thread::spawn(move || {
                while let Ok((index, input_num)) = receiver.recv() {
                    let output_num = f(input_num);
                    let _ = sender2.send((index, output_num));
                }
            }))
        }
    
    let mut index = input_vec.len();
    while let Some(input_num) = input_vec.pop() {
        let _ = sender.send((index, input_num));
        index -= 1
    }
    // drop both senders to close channels
    drop(sender);
    drop(sender2);

    // iterate through output channel
    // TODO: need to sort this, have the index where it should belong
    // but how to do this without sorting the output vector?
    for (_, output_val) in receiver2.iter() {
        output_vec.push(output_val)
    }
    output_vec
}

fn main() {
    let v = vec![6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 12, 18, 11, 5, 20];
    let squares = parallel_map(v, 10, |num| {
        println!("{} squared is {}", num, num * num);
        thread::sleep(time::Duration::from_millis(500));
        num * num
    });
    println!("squares: {:?}", squares);
}

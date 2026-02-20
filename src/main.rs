use std::thread;

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let handle = thread::spawn(move || {    // threads will own the numbers vector by the move keyword.
        let sum: i32 = numbers.iter().sum();
        println!("the sum is: {}", sum);
    });
    // wait from the thread to complete.
    handle.join().unwrap();
}

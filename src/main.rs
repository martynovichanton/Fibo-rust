use std::{
    io::{self, Write},
    sync::{Arc, Mutex},
    thread,
    time,
};

fn fibo(n: usize) -> usize {
    if n <= 1 {
        n
    } else {
        fibo(n - 1) + fibo(n - 2)
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let nums: usize = args[1].parse().unwrap_or(0);

    let start: time::Instant = time::Instant::now();

    // single threaded
    // let mut arr: Vec<usize> = vec![0; nums];
    // for i in 0..nums {
    //     print!("{} ", i);
    //     match io::stdout().flush() {
    //         Ok(_) => (),
    //         Err(err)=> eprintln!("Error flushing stdout: {}", err)
    //     }
    //     arr[i] = fibo(i);
    // }


    // multithreaded
    let arr: Arc<Mutex<Vec<usize>>> = Arc::new(Mutex::new(vec![0; nums]));
    
    let mut handles: Vec<thread::JoinHandle<()>> = vec![];
    for i in 0..nums {
        print!("{i} ");
        match io::stdout().flush() {
            Ok(_) => (),
            Err(err)=> eprintln!("Error flushing stdout: {err}")
        }
        let arr_clone: Arc<Mutex<Vec<usize>>> = Arc::clone(&arr);
        let handle: thread::JoinHandle<()> = thread::spawn(move || {
            let result: usize = fibo(i);
            let mut arr: std::sync::MutexGuard<'_, Vec<usize>> = arr_clone.lock().unwrap();
            arr[i] = result;

        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // let arr: Vec<usize> = Arc::try_unwrap(arr).unwrap().into_inner().unwrap();
    let arr: Vec<usize> = match Arc::try_unwrap(arr) {
        Ok(mutex) => match mutex.into_inner() {
            Ok(vec) => vec,
            Err(e) => {
                eprintln!("Failed to unlock mutex: {e}");
                return;
            }
        },
        Err(_) => {
            eprintln!("Failed to unwrap Arc: Multiple references exist");
            return;
        }
    };


    println!("\n{arr:?}");
    let time_elapsed: std::time::Duration = start.elapsed();
    println!("Total time: {time_elapsed:?}");
}

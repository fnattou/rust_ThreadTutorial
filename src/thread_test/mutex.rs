use std::sync::{Arc, Mutex};

pub fn mutex(lock:Arc<Mutex<u64>>, num:u32){
    for _ in 1..10 {
        let mut val = lock.lock().unwrap();
        *val += 1;
        println!("val is {} at thread {}", *val, num);
        if *val > 100{
            
        }
    }
}
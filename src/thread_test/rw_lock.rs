use std::sync::RwLock;


pub fn rwlock_test(){

    let lock = RwLock::new(10);
    {
        let v1 = lock.read().unwrap();
        let v2 = lock.read().unwrap();
        println!("v1 = {}", v1);
        println!("v2 = {}", v2);
    }
    {
        let mut v = lock.write().unwrap();
        *v = 9;
        println!("v = {}", v);
    }
}
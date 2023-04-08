use futures::{
    future::{
        BoxFuture, FutureExt
    },
    task::{
        waker_ref, ArcWake
    }
};
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use crate::asynchronous::my_coroutine::Hello;

//実行単位
struct  Task {
    hello: Mutex<BoxFuture<'static, ()>>,
}

impl Task {
    fn new() -> Self {
        let hello = Hello::new();
        Task { hello: Mutex::new(hello.boxed()) }
    }
}
//何もしない
impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {}
}

pub fn test() {
    // 初期化
    let task = Arc::new(Task::new());
    let waker = waker_ref(&task);
    let mut ctx = Context::from_waker(&waker);
    let mut hello 
            = task.hello.lock().unwrap();

    //停止と再生の繰り返し
    hello.as_mut().poll(&mut ctx);
    print!("\n test \n");
    hello.as_mut().poll(&mut ctx);
    print!("\n test \n");
    hello.as_mut().poll(&mut ctx);
}
use futures::{
    future::{BoxFuture, FutureExt},
    task::{waker_ref, ArcWake}
};
use std::{
    future::Future,
    pin::Pin,
    sync::{mpsc::{sync_channel, Receiver, SyncSender},Arc,Mutex},
    task::{Context, Poll}
};

pub struct Task {
    //実行するコルーチン
    future: Mutex<BoxFuture<'static, ()>>,
    //Executorへスケジューリングするためのチャネル
    sender: SyncSender<Arc<Task>>,
}
impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        //自信をスケジューリング
        let self0 = arc_self.clone();
        arc_self.sender.send(self0).unwrap();
    }
}

pub struct Executor {
    //実行キュー
    sender: SyncSender<Arc<Task>>,
    receiver: Receiver<Arc<Task>>,
}
impl Executor {
    pub fn new() -> Self {
        //チャネルを生成。キューのサイズは最大1024個
        let (sender, receiver)
            = sync_channel(1024);
        Executor { sender: (sender.clone()), receiver, }
    }
    //新たにTaskを生成するためのSpawnerを作成
    pub fn get_spawner(&self) -> Spawner {
        Spawner { sender: self.sender.clone(), }
    }
    pub fn run(&self) {
        //チャネルからTaskを受信して順に実行
        for _ in 0..3
        {
            let task = self.receiver.recv().unwrap();
            //コンテキストを生成
            let mut future
                    = task.future.lock().unwrap();
            let waker = waker_ref(&task);
            let mut ctx = Context::from_waker(&waker);
            //pollを呼び出し、実行
            let _ = future.as_mut().poll(&mut ctx);
            print!("running");
        }
        print!("finish");
    }
}

pub struct Spawner {
    sender:SyncSender<Arc<Task>>,
}
impl Spawner {
    pub fn spawn(&self, future:impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task{
            future: Mutex::new(future),
            sender: self.sender.clone(),
        });
        //実行キューにエンキュー
        self.sender.send(task).unwrap();
    }
}

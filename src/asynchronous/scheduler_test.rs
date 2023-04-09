use futures::{
    Future, task::Context, task::Poll
};
use std::pin::Pin;
use crate::asynchronous::scheduler::*;

pub struct Hello {
    state: StateHello,
}

enum StateHello {
    HELLO,
    WORLD,
    END,
}

impl Hello {
    pub fn new() -> Self {
        Hello { state: (StateHello::HELLO) }
    }
}

impl Future for Hello {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        match (*self).state {
            StateHello::HELLO => {
                print!("Hello, ");
                //WORLD状態に移行
                (*self).state = StateHello::WORLD;
                cx.waker().wake_by_ref();//自身を実行キューにエンキュー
                Poll::Pending //再度呼び出し可能
            }
            StateHello::WORLD => {
                print!("World!");
                //END状態に移行
                (*self).state = StateHello::END;
                cx.waker().wake_by_ref();//自身を実行キューにエンキュー
                Poll::Pending
            }
            StateHello::END => {
                Poll::Ready(()) //終了状態
            }
        }
    }
}

pub fn test() {
    let executor = Executor::new();
    executor.get_spawner().spawn(Hello::new());
    executor.run();
    print!("aa");
}
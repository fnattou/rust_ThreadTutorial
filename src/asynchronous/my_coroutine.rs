use futures::{
    Future, task::Context, task::Poll
};
use std::pin::Pin;

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
    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
        match (*self).state {
            StateHello::HELLO => {
                print!("Hello, ");
                //WORLD状態に移行
                (*self).state = StateHello::WORLD;
                Poll::Pending //再度呼び出し可能
            }
            StateHello::WORLD => {
                print!("World!");
                //END状態に移行
                (*self).state = StateHello::END;
                Poll::Pending
            }
            StateHello::END => {
                Poll::Ready(()) //終了状態
            }
        }
    }
}
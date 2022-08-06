
use std::{
    future::Future,
    pin::Pin,
    sync::{
        atomic::AtomicBool,
        atomic::Ordering,
        Arc,
    },
    task::{Context, Poll},
    thread,
    time::Duration,
};

use futures::{
    task::  AtomicWaker,
};
pub struct TimerFuture {
    shared_state: Arc<SharedState>,
}

/// 在Future和等待的线程间共享状态
struct SharedState {
    /// 定时(睡眠)是否结束
    completed: AtomicBool,

    /// 当睡眠结束后，线程可以用`waker`通知`TimerFuture`来唤醒任务
    waker: AtomicWaker,
}

impl Future for TimerFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("task running once");
        self.shared_state.waker.register(cx.waker());
        if self.shared_state.completed.load(Ordering::SeqCst) {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(SharedState {
            completed: AtomicBool::new(false),
            waker: AtomicWaker::new(),
        });
        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            // 睡眠指定时间实现计时功能
            thread::sleep(duration);
            let shared_state = thread_shared_state.clone();
            // 通知执行器定时器已经完成，可以继续`poll`对应的`Future`了
            shared_state.completed.store(true, Ordering::SeqCst);
            thread_shared_state.waker.wake();
        });

        TimerFuture { shared_state }
    }
}
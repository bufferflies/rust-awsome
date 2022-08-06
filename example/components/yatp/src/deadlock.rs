use std::{
    future::Future,
    sync::{
        mpsc::{sync_channel, Receiver, SyncSender},
        Arc, Mutex,
    },
    task::Context,
   
};

use futures::{
    future::{BoxFuture, FutureExt},
    task::{waker_ref, ArcWake},
};


struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

struct Task {
    future: Mutex<Option<BoxFuture<'static, ()>>>,

    task_sender: SyncSender<Arc<Task>>,
}

fn new_executor_and_spawner() -> (Executor, Spawner) {
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
    (Executor { ready_queue }, Spawner { task_sender })
}

impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task1 = Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        };
        let task = Arc::new(task1);
        let _ok = self.task_sender.send(task);
    }
}

impl ArcWake for Task {
    // put task into the global queue again.
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        arc_self.task_sender.send(cloned).expect("full queue");
    }
}

impl Executor {
    fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            // 获取一个future，若它还没有完成(仍然是Some，不是None)，则对它进行一次poll并尝试完成它
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                // 基于任务自身创建一个 `LocalWaker`
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&*waker);
                // `BoxFuture<T>`是`Pin<Box<dyn Future<Output = T> + Send + 'static>>`的类型别名
                // 通过调用`as_mut`方法，可以将上面的类型转换成`Pin<&mut dyn Future + Send + 'static>`
                if future.as_mut().poll(context).is_pending() {
                    // Future还没执行完，因此将它放回任务中，等待下次被poll
                    *future_slot = Some(future);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::time_future;
    use std::time::Duration;
    #[test]
    fn deadlock() {
        let (executor, spawner) = new_executor_and_spawner();

        let task = async {
            println!("howdy!");
            // 创建定时器Future，并等待它完成
            let f = time_future::TimerFuture::new(Duration::new(10, 0));
            f.await;
            println!("done!");
        };
        // 生成一个任务
        spawner.spawn(task);

        // drop掉任务，这样执行器就知道任务已经完成，不会再有新的任务进来
        drop(spawner);

        // 运行执行器直到任务队列为空
        // 任务运行后，会先打印`howdy!`, 暂停2秒，接着打印 `done!`
        executor.run();
    }
}

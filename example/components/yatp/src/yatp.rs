#[cfg(test)]
mod tests {
    use std::{
        thread,
        time::{Duration, Instant},
    };
    use yatp::pool::Builder;
    #[test]
    fn it_works() {
        let pool = Builder::new("test").max_thread_count(1).build_future_pool();
        for i in 0..10 {
            let instant = Instant::now();
            let task = async move {
                println!("running thread:{},wait:{:?}", i, instant.elapsed());
                thread::sleep(Duration::from_secs(1));
                return;
            };

            pool.spawn(task)
        }
        thread::sleep(Duration::from_secs(10));
        pool.shutdown();
        println!("thread:{}", 10)
    }
}

fn  main(){
    use std::sync::{Arc,RwLock};
    let count=Arc::new(RwLock::new(0));
    let count1=count.clone();
    
    let handle=std::thread::spawn(move||{
        {
            let _rg=count.read().unwrap();
        }
        std::thread::sleep(std::time::Duration::from_secs(5));
        println!("acquire read lock again");
        let _rg=count.read().unwrap();
        println!("acquire read lock again done");
    });
    // std::thread::sleep(std::time::Duration::from_secs(1));
    let handle1=std::thread::spawn(move||{
        println!("acquire write lock again");
        let _w=count1.write().unwrap();
        std::thread::sleep(std::time::Duration::from_secs(10));
        println!("acquire write lock again down {}",_w);
    });
  
    let _r=handle1.join();
    let _r1=handle.join();
}
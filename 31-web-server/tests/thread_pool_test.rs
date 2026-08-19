// 集成测试：测试 ThreadPool 的并发行为
// 运行：cargo test -p web-server-31

use web_server_lib::ThreadPool;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[test]
fn many_jobs_complete() {
    let pool = ThreadPool::new(4);
    let counter = Arc::new(AtomicUsize::new(0));

    for _ in 0..100 {
        let c = Arc::clone(&counter);
        pool.execute(move || {
            c.fetch_add(1, Ordering::SeqCst);
        });
    }

    thread::sleep(Duration::from_millis(500));
    assert_eq!(counter.load(Ordering::SeqCst), 100);
}

#[test]
fn pool_drops_cleanly() {
    let counter = Arc::new(AtomicUsize::new(0));
    {
        let pool = ThreadPool::new(2);
        for _ in 0..5 {
            let c = Arc::clone(&counter);
            pool.execute(move || {
                c.fetch_add(1, Ordering::SeqCst);
            });
        }
        // pool 在这里 drop，应触发优雅关闭
    }
    thread::sleep(Duration::from_millis(200));
    assert_eq!(counter.load(Ordering::SeqCst), 5);
}

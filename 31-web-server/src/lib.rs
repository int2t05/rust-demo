// ============================================================
// 31-web-server: 多线程 Web 服务器（库部分）
// ============================================================
// 学习点：
// 1. ThreadPool 与 Worker 结构体
// 2. Mutex<Receiver<Job>> + Arc 多线程共享任务队列
// 3. 通道驱动的工作窃取式（简化）线程池
// 4. 优雅关闭（Drop 实现停止 worker）
// 5. trait 对象 Box<dyn FnOnce() + Send + 'static> 作为 Job
//
// 对应教程：docs/notes/14-实战项目/02-多线程Web服务器.md
// 运行方式：cargo run -p web_server_demo
// ============================================================

use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

/// 任务类型：一个可在线程中执行、可 Send 的闭包
type Job = Box<dyn FnOnce() + Send + 'static>;

/// 消息：要么是执行任务，要么是要求 worker 停止
enum Message {
    NewJob(Job),
    Terminate,
}

/// 线程池：固定数量的 worker 共享一个任务队列
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Message>>,
}

impl ThreadPool {
    /// 创建线程池：必须 size > 0
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0, "线程池大小必须 > 0");

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    /// 提交一个任务到线程池
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        if let Some(ref sender) = self.sender {
            sender.send(Message::NewJob(job)).unwrap();
        }
    }

    /// 优雅关闭：发送 Terminate 给所有 worker，再 join
    pub fn shutdown(&mut self) {
        if let Some(sender) = self.sender.take() {
            for _ in &self.workers {
                sender.send(Message::Terminate).unwrap();
            }
        }
        for worker in &mut self.workers {
            if let Some(handle) = worker.thread.take() {
                if let Err(e) = handle.join() {
                    eprintln!("  worker {} join 失败: {:?}", worker.id, e);
                }
            }
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // 如果用户没显式 shutdown，Drop 时也做一次
        if self.sender.is_some() {
            self.shutdown();
        }
    }
}

/// Worker：一个常驻线程，从队列取任务执行
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let handle = thread::spawn(move || loop {
            // 加锁接收消息；锁在 recv 返回后释放
            let message = {
                let lock = receiver.lock().unwrap();
                lock.recv()
            };

            match message {
                Ok(Message::NewJob(job)) => {
                    // println!("  worker {} 执行任务", id);
                    job();
                }
                Ok(Message::Terminate) => {
                    // println!("  worker {} 收到终止信号", id);
                    break;
                }
                Err(e) => {
                    eprintln!("  worker {} 通道错误: {}", id, e);
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(handle),
        }
    }
}

// ===== 单元测试 =====
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::time::Duration;

    #[test]
    fn pool_creates_workers() {
        let pool = ThreadPool::new(4);
        assert_eq!(pool.workers.len(), 4);
    }

    #[test]
    #[should_panic(expected = "线程池大小必须 > 0")]
    fn pool_zero_panics() {
        let _ = ThreadPool::new(0);
    }

    #[test]
    fn execute_runs_jobs() {
        let pool = ThreadPool::new(2);
        let counter = Arc::new(AtomicUsize::new(0));

        for _ in 0..10 {
            let c = Arc::clone(&counter);
            pool.execute(move || {
                c.fetch_add(1, Ordering::SeqCst);
            });
        }

        // 给线程时间执行
        thread::sleep(Duration::from_millis(200));
        assert_eq!(counter.load(Ordering::SeqCst), 10);
    }

    #[test]
    fn shutdown_joins_all() {
        let mut pool = ThreadPool::new(3);
        pool.execute(|| {});
        pool.execute(|| {});
        pool.shutdown();
        // shutdown 后所有 thread 应已 join
        assert!(pool.workers.iter().all(|w| w.thread.is_none()));
    }
}

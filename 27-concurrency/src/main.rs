// ============================================================
// 27-concurrency: 线程 / mpsc / Mutex / Send+Sync
// ============================================================
// 学习点：
// 1. std::thread::spawn 与 JoinHandle.join()
// 2. move 闭包转移到线程
// 3. 通道 mpsc::channel() / send / recv
// 4. 多生产者
// 5. Mutex<T> / lock()
// 6. Arc<Mutex<T>> 多线程共享可变
// 7. Send / Sync 标记 trait
// 8. 简单线程池概念
//
// 对应教程：docs/notes/12-并发编程/01-线程模型.md
// 运行方式：cargo run -p concurrency_demo
// ============================================================

use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    println!("===== 1. spawn 与 join =====");
    // spawn 返回 JoinHandle，调用 join 阻塞等待线程结束
    let handle = thread::spawn(|| {
        for i in 1..=3 {
            println!("  子线程: {}", i);
            thread::sleep(Duration::from_millis(10));
        }
        42
    });

    // 主线程同时干活
    for i in 1..=3 {
        println!("  主线程: {}", i);
        thread::sleep(Duration::from_millis(10));
    }

    // join 等待并取回返回值
    let result = handle.join().unwrap();
    println!("  子线程返回: {}", result);

    println!("\n===== 2. move 闭包 =====");
    // 不加 move，闭包借用 data，线程可能在 data 释放后访问 -> 编译错误
    // 加 move，把 data 所有权转移到线程
    let data = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("  线程内 data: {:?}", data);
        data.len()
    });
    let len = handle.join().unwrap();
    println!("  线程返回长度: {}", len);
    // println!("{:?}", data); // 编译错误：data 已 move

    println!("\n===== 3. mpsc 通道 =====");
    // mpsc: multiple producer, single consumer
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        let msgs = vec!["hello", "from", "child"];
        for m in msgs {
            tx.send(m).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
    });

    // 主线程接收：rx 是迭代器，每次 yield 一个收到的值
    for received in rx {
        println!("  收到: {}", received);
    }
    handle.join().unwrap();

    println!("\n===== 4. 多生产者 =====");
    // clone tx 得到多个发送端
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    let tx2 = tx;

    let h1 = thread::spawn(move || {
        let vals = vec![1, 2, 3];
        for v in vals {
            tx1.send(("A", v)).unwrap();
        }
    });

    let h2 = thread::spawn(move || {
        let vals = vec![10, 20, 30];
        for v in vals {
            tx2.send(("B", v)).unwrap();
        }
    });

    // 接收所有消息（直到所有发送端关闭）
    // 用 rx.try_iter 收集所有当前可用的
    drop(h1.join().unwrap());
    drop(h2.join().unwrap());
    let collected: Vec<(&str, i32)> = rx.iter().collect();
    println!("  多生产者消息: {:?}", collected);

    println!("\n===== 5. Mutex =====");
    // Mutex 提供互斥访问，lock() 返回 MutexGuard
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num += 1;
    } // MutexGuard 在这里 drop，释放锁
    println!("  Mutex 内值: {:?}", m);

    println!("\n===== 6. Arc<Mutex<T>> 共享可变 =====");
    // Arc 多线程共享所有权，Mutex 提供可变性
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }
    println!("  10 线程各 +1 后计数器 = {}", *counter.lock().unwrap());

    println!("\n===== 7. Send / Sync 标记 trait =====");
    // Send：类型可安全地在线程间转移所有权（Rc 不是 Send，Arc 是）
    // Sync：&T 可安全地在线程间共享（RefCell 不是 Sync，Mutex 是）
    // 编译器自动推导；实现 unsafe 的 Send/Sync 需手动

    // 验证 Arc 是 Send + Sync
    fn assert_send_sync<T: Send + Sync>(_: &T) {}
    let arc = Arc::new(5);
    assert_send_sync(&arc);
    println!("  Arc<i32> 满足 Send + Sync");

    // Rc 不是 Send（注释演示）
    // let rc = std::rc::Rc::new(5);
    // thread::spawn(move || { println!("{:?}", rc); }); // 编译错误: Rc 不是 Send

    println!("\n===== 8. 简单线程池概念 =====");
    // 用通道 + 固定数量的 worker 模拟线程池
    let (job_tx, job_rx) = mpsc::channel::<Box<dyn FnOnce() + Send + 'static>>();
    let job_rx = Arc::new(Mutex::new(job_rx));

    let mut workers = vec![];
    for id in 0..4 {
        let job_rx = Arc::clone(&job_rx);
        let handle = thread::spawn(move || loop {
            // 加锁取出一个任务；通道关闭时退出
            let job = {
                let guard = job_rx.lock().unwrap();
                guard.recv()
            };
            match job {
                Ok(j) => {
                    println!("  worker {} 执行任务", id);
                    j();
                }
                Err(_) => {
                    println!("  worker {} 退出", id);
                    break;
                }
            }
        });
        workers.push(handle);
    }

    // 提交几个任务
    for i in 0..8 {
        job_tx.send(Box::new(move || {
            println!("    任务 {} 完成", i);
        })).unwrap();
    }

    drop(job_tx); // 关闭发送端，worker 收到 Err 后退出
    for h in workers {
        h.join().unwrap();
    }

    println!("\n===== 9. 总结 =====");
    println!("  线程：thread::spawn + move + join");
    println!("  通信：mpsc 通道（多生产单消费）");
    println!("  共享：Arc<Mutex<T>> 多线程可变共享");
    println!("  安全：Send/Sync 编译期保证");
}

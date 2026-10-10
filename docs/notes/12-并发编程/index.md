# Concurrency(并发)

线程、通道、锁与 Send/Sync 四件套全部由编译器在编译期把关——不安全的跨线程操作根本编译不过;本主题对应 1 个 crate(27-concurrency)。

## 文章索引

| 文章 | 配套 crate | 一句话 |
|---|---|---|
| [01 线程模型](./01-线程模型) | 27-concurrency | spawn/join、move、mpsc 通道、`Arc<Mutex>`、Send/Sync 一篇打通 |

## 练习索引

| 练习 | 覆盖 | 出处 |
|---|---|---|
| [Rustlings 20_threads](https://github.com/rust-lang/rustlings/tree/main/exercises/20_threads) | 线程、通道、Mutex、Send/Sync | [01](./01-线程模型#实践项) 的实践项 |
| [Exercism Rust track](https://exercism.org/tracks/rust) | 串行改并行 | [01](./01-线程模型#实践项) 的实践项 |

## 怎么用本主题

1. 读文章,记住「数据跨线程的三条路」;
2. `cargo run -p concurrency_demo` 跑一遍,对照九个 demo 段看输出;
3. 做 Rustlings 20_threads;
4. 对照官方解答自评,说不出「为什么错」就回到文章。

完成本主题后进入 13-高级特性。

# Final Projects(实战项目)

把前 30 个子项目练过的原语拼进真实程序:一个 CLI 搜索工具与一个并发 Web 服务器。本主题对应仓库 2 个 crate,建议先 01 后 02。

## 文章索引

| 文章 | 配套 crate | 一句话 |
|---|---|---|
| [01 Cli工具minigrep](./01-Cli工具minigrep) | 30-minigrep | 命令行参数、文件 I/O、错误处理、生命周期、迭代器落地成 CLI,lib 承载逻辑、main 做薄壳 |
| [02 多线程Web服务器](./02-多线程Web服务器) | 31-web-server | TcpListener 加自建线程池:channel 任务队列、Arc/Mutex 共享接收端、trait 对象任务、Drop 优雅关闭 |
| [03 发布到Crates-io](./03-发布到Crates-io) | — | 清单字段、publish 与 yank |

## 练习索引

| 练习 | 覆盖 | 出处 |
|---|---|---|
| [The Rust Book ch12 An I/O Project](https://doc.rust-lang.org/book/ch12-00-an-io-project.html) | 文件 I/O、参数解析、错误处理 | [01](./01-Cli工具minigrep#实践项) |
| [Rustlings 22_clippy](https://github.com/rust-lang/rustlings/tree/main/exercises/22_clippy) | lint 与代码质量 | [01](./01-Cli工具minigrep#实践项) |
| [The Rust Book ch21 Final Project: A Web Server](https://doc.rust-lang.org/book/ch21-00-final-project-a-web-server.html) | 线程池、TCP、HTTP | [02](./02-多线程Web服务器#实践项) |
| [Exercism Rust track](https://exercism.org/tracks/rust) | 综合巩固 | 两篇的实践项 |

## 怎么用本主题

1. 跑 `cargo run -p minigrep_demo`:无参数先看内置样例,再 `cargo run -p minigrep_demo -- to poem.txt` 搜自己的文件;
2. 跑 `cargo run -p web_server_demo`,另开终端 curl 首页与 /sleep,体会并发与阻塞的差别;
3. 对照两篇的源码解读表读源码,重点盯两处:`search<'a>` 的生命周期标注,Worker 里加锁的作用域;
4. 做练习索引里的项目与题目,说不出「为什么这样设计」就回到对应文章。

至此 31 个子项目的学习路径全部完成。

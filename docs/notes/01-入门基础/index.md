# Getting Started(入门基础)

两个可运行的小程序,把 cargo 工程、格式化输出、用户输入、循环与错误处理一次走通。本主题对应仓库 2 个 crate,建议按顺序学习。

## 文章索引

| 文章 | 配套 crate | 一句话 |
|---|---|---|
| [01 环境搭建](./01-环境搭建) | — | rustup/cargo 安装、验证与常用命令 |
| [02 Hello Cargo](./02-Hello-Cargo) | 01-hello | 第一个程序:cargo 工程结构与 println! 格式化全家桶 |
| [03 猜数字游戏](./03-猜数字游戏) | 02-guessing-game | 输入、随机数、循环、错误处理串成完整程序 |

## 练习索引

| 练习 | 覆盖 | 出处 |
|---|---|---|
| [Rustlings 00_intro](https://github.com/rust-lang/rustlings/tree/main/exercises/00_intro) | Hello World、cargo 运行 | [02](./02-Hello-Cargo#实践项) 的实践项 |
| [RBE Formatted print](https://doc.rust-lang.org/stable/rust-by-example/hello/print.html) | println! 格式化 | [02](./02-Hello-Cargo#实践项) 的实践项 |
| [The Rust Book ch2](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html) | 猜数字全流程 | [03](./03-猜数字游戏#实践项) 的实践项 |
| [Exercism Rust track](https://exercism.org/tracks/rust) | 综合 | 两篇的实践项 |

## 怎么用本主题

1. 确认环境就绪:`rustc --version`、`cargo --version` 有输出再开始;
2. 读文章,打开对应 crate 跑一遍(`cargo run -p hello_demo`、`cargo run -p guessing_game`),对照输出理解注释;
3. 做两篇文章实践项里的练习;
4. 对照官方解答自评,说不出「为什么错」就回到对应文章。

完成本主题后进入 02-核心语法(变量、类型与控制流)。

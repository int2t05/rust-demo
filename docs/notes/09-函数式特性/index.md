# Functional Features(函数式特性)

函数式特性 = 闭包 + 迭代器:闭包捕获环境,让「一段行为」可以作为值传递;迭代器把数据处理拆成惰性、可组合的流水线。本主题对应仓库 2 个 crate,建议按顺序学习。

## 文章索引

| 文章 | 配套 crate | 一句话 |
|---|---|---|
| [01 闭包](./01-闭包) | 24-closures | 捕获环境、move、Fn/FnMut/FnOnce、闭包作参数与返回值 |
| [02 迭代器](./02-迭代器) | 25-iterators | 三种迭代方式、适配器/消费器、自定义 Iterator、零成本抽象 |
| [03 改进CLI项目](./03-改进CLI项目) | 30-minigrep | 用迭代器/env/错误处理重构 CLI |

## 练习索引

| 练习 | 覆盖 | 出处 |
|---|---|---|
| [Rustlings 18_iterators](https://github.com/rust-lang/rustlings/tree/main/exercises/18_iterators) | 适配器、消费器 | [02](./02-迭代器#实践项) 的实践项 |
| [Exercism Rust track](https://exercism.org/tracks/rust) | 闭包与迭代器综合 | [01](./01-闭包#实践项)、[02](./02-迭代器#实践项) 的实践项 |

## 怎么用本主题

1. 读文章,记住规则;
2. 打开对应 crate 跑一遍(`cargo run -p closures_demo`、`cargo run -p iterators_demo`),对照输出理解注释;
3. 做练习;
4. 对照官方解答自评,说不出「为什么错」就回到对应文章。

完成本主题后进入 11-智能指针。

# Ownership System(所有权系统)

所有权是 Rust 最独特的部分:编译器靠它保证内存安全,不依赖垃圾回收。本主题对应仓库 3 个 crate,建议按顺序学习。

## 文章索引

| 文章 | 配套 crate | 一句话 |
|---|---|---|
| [01 所有权规则](./01-所有权规则) | 08-ownership | 栈/堆、移动、Clone、Copy、函数间转移 |
| [02 引用与借用](./02-引用与借用) | 09-references | 三条借用规则、悬垂引用 |
| [03 切片类型](./03-切片类型) | 10-slices | &str 与 &[T]、函数参数通用化 |

## 练习索引

| 练习 | 覆盖 | 出处 |
|---|---|---|
| [Rustlings 06_move_semantics](https://github.com/rust-lang/rustlings/tree/main/exercises/06_move_semantics) | 所有权、借用 | [01](./01-所有权规则#实践项)、[02](./02-引用与借用#实践项) 的实践项 |
| [Rustlings 09_strings](https://github.com/rust-lang/rustlings/tree/main/exercises/09_strings) | 切片 | [03](./03-切片类型#实践项) 的实践项 |
| [Exercism Rust track](https://exercism.org/tracks/rust) | 综合 | 三篇的实践项 |

## 怎么用本主题

1. 读文章,记住规则;
2. 打开对应 crate 跑一遍(`cargo run -p ownership_demo` 等),对照输出理解注释;
3. 做练习;
4. 对照官方解答自评,说不出「为什么错」就回到对应文章。

完成本主题后进入 04-数据结构。

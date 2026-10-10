# Core Syntax(核心语法)

核心语法是 Rust 的地基:值怎么绑定、数据有哪些类型、代码怎么组织执行。本主题对应仓库 5 个 crate,建议按顺序学习。

## 文章索引

| 文章 | 配套 crate | 一句话 |
|---|---|---|
| [01 变量与可变性](./01-变量与可变性) | 03-variables | let/let mut/遮蔽/const/static |
| [02 标量类型](./02-标量类型) | 04-scalar-types | 整数、浮点、bool、char 与溢出处理 |
| [03 复合类型](./03-复合类型) | 05-compound-types | 元组、数组、单元类型 () |
| [04 函数](./04-函数) | 06-functions | 表达式 vs 语句、Copy/move 传参 |
| [05 控制流](./05-控制流) | 07-control-flow | if/loop/while/for/match/if let |

## 练习索引

| 练习 | 覆盖 | 出处 |
|---|---|---|
| [Rustlings 01_variables](https://github.com/rust-lang/rustlings/tree/main/exercises/01_variables) | 变量、可变性、shadowing | [01](./01-变量与可变性#实践项) 的实践项 |
| [Rustlings 04_primitive_types](https://github.com/rust-lang/rustlings/tree/main/exercises/04_primitive_types) | 标量、元组、数组 | [02](./02-标量类型#实践项)、[03](./03-复合类型#实践项) 的实践项 |
| [Rustlings 02_functions](https://github.com/rust-lang/rustlings/tree/main/exercises/02_functions) | 函数与返回值 | [04](./04-函数#实践项) 的实践项 |
| [Rustlings 03_if](https://github.com/rust-lang/rustlings/tree/main/exercises/03_if) | 条件分支、match 基础 | [05](./05-控制流#实践项) 的实践项 |
| [Exercism Rust track](https://exercism.org/tracks/rust) | 综合 | 五篇的实践项 |

## 怎么用本主题

1. 读文章,记住规则;
2. 打开对应 crate 跑一遍(`cargo run -p variables_demo` 等),对照输出理解注释;
3. 做练习;
4. 对照官方解答自评,说不出「为什么错」就回到对应文章。

完成本主题后进入 03-所有权系统。

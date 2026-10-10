# Testing(测试)

Rust 内置测试框架:单元、集成、文档三层测试,一条 `cargo test` 全部跑完,文档示例也被纳入测试。本主题对应仓库 1 个 crate。

## 文章索引

| 文章 | 配套 crate | 一句话 |
|---|---|---|
| [01 单元测试](./01-单元测试) | 23-testing | assert 系列宏、should_panic、ignore |
| [02 集成测试](./02-集成测试) | 23-testing | tests/ 目录、只测公开 API |
| [03 测试组织与文档测试](./03-测试组织与文档测试) | 23-testing | 三层测试各放哪、断言宏怎么选、should_panic 与文档测试 |

## 练习索引

| 练习 | 覆盖 | 出处 |
|---|---|---|
| [Rustlings 17_tests](https://github.com/rust-lang/rustlings/tree/main/exercises/17_tests) | 断言宏、should_panic | [03](./03-测试组织与文档测试#实践项) 的实践项 |
| [Rust By Example - Testing](https://doc.rust-lang.org/stable/rust-by-example/testing.html) | 文档测试 | [03](./03-测试组织与文档测试#实践项) 的实践项 |
| [Exercism Rust track](https://exercism.org/tracks/rust) | 综合 | [03](./03-测试组织与文档测试#实践项) 的实践项 |

## 怎么用本主题

1. 读文章,记住三层测试各放哪、断言宏怎么选;
2. 跑 `cargo run -p testing_demo` 看库函数行为,再跑 `cargo test -p testing_demo` 看四段测试输出全绿;
3. 做练习;
4. 对照官方解答自评,说不出「为什么错」就回到文章。

完成本主题后进入 09-函数式特性。

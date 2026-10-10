# Advanced Features(高级特性)

unsafe 放行编译器静态验证不了的五类底层操作,宏在编译期按模式生成代码;本主题对应仓库 2 个 crate。

## 文章索引

| 文章 | 配套 crate | 一句话 |
|---|---|---|
| [01 Unsafe Rust](./01-Unsafe-Rust) | 28-unsafe | 五类 unsafe 操作、借用检查不关闭、安全 API 封装 |
| [02 高级Trait](./02-高级Trait) | — | 关联类型、supertrait、全限定语法 |
| [03 宏](./03-宏) | 29-macros | macro_rules!、片段说明符、重复与递归、生成结构体 |
| [04 类型转换与Deref](./04-类型转换与Deref) | 26-smart-pointers | From/Into/AsRef 与 Deref |

## 练习索引

| 练习 | 覆盖 | 出处 |
|---|---|---|
| [Exercism Rust track](https://exercism.org/tracks/rust) | unsafe(无专属 Rustlings 目录)、综合 | [01](./01-Unsafe-Rust#实践项)、[03](./03-宏#实践项) 的实践项 |
| [Rustlings 21_macros](https://github.com/rust-lang/rustlings/tree/main/exercises/21_macros) | 声明宏 | [03](./03-宏#实践项) 的实践项 |
| [Rust By Example - Unsafe](https://doc.rust-lang.org/stable/rust-by-example/unsafe.html) | 裸指针、FFI | [01](./01-Unsafe-Rust#实践项) 的实践项 |

## 怎么用本主题

1. 读文章,记住规则;
2. 打开对应 crate 跑一遍(`cargo run -p unsafe_demo`、`cargo run -p macros_demo`),对照输出理解注释;
3. 做练习;
4. 对照官方解答自评,说不出「为什么错」就回到对应文章。

完成本主题后进入 14-实战项目。

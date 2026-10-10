# Generics, Traits, Lifetimes(泛型、Trait 与生命周期)

一句话:泛型让一份代码适配多类型,trait 约定行为契约,生命周期标注保证引用不悬垂——三者共同构成 Rust 的编译期抽象层。本主题对应仓库 4 个 crate,建议按顺序学习。

## 文章索引

| 文章 | 配套 crate | 一句话 |
|---|---|---|
| [01 泛型](./01-泛型) | 20-generics | 类型参数 + 单态化,零运行时开销 |
| [02 Trait特征](./02-Trait特征) | 21-traits | 行为契约、默认实现与 trait bound |
| [03 Trait对象](./03-Trait对象) | 22-trait-objects | `dyn Trait` 动态分发,异构集合 |
| [04 生命周期标注](./04-生命周期标注) | 11-lifetimes | `'a` 描述引用存活关系,防悬垂 |

## 练习索引

| 练习 | 覆盖 | 出处 |
|---|---|---|
| [Rustlings 14_generics](https://github.com/rust-lang/rustlings/tree/main/exercises/14_generics) | 泛型参数与约束 | [01](./01-泛型#实践项) 的实践项 |
| [Rustlings 15_traits](https://github.com/rust-lang/rustlings/tree/main/exercises/15_traits) | trait 实现与 bound | [02](./02-Trait特征#实践项) 的实践项 |
| [Rustlings 16_lifetimes](https://github.com/rust-lang/rustlings/tree/main/exercises/16_lifetimes) | 生命周期标注 | [04](./04-生命周期标注#实践项) 的实践项 |
| [Exercism Rust track](https://exercism.org/tracks/rust) | 综合 | 四篇的实践项 |

## 怎么用本主题

1. 读文章,记住规则;
2. 打开对应 crate 跑一遍(`cargo run -p generics_demo` 等),对照输出理解注释;
3. 做练习;
4. 对照官方解答自评,说不出「为什么错」就回到对应文章。

完成本主题后进入 08-测试。

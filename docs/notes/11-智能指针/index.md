# Smart Pointers(智能指针)

智能指针在所有权之上补齐堆分配、多所有者共享与内部可变三类能力;本主题对应仓库 1 个 crate,一篇文章覆盖其全部演示。

## 文章索引

| 文章 | 配套 crate | 一句话 |
|---|---|---|
| [01 Box](./01-Box) | 26-smart-pointers | Box/Rc/Arc/RefCell/Deref/Drop 全景,按「所有者、可变性、线程」三问选指针 |
| [02 Rc与Arc](./02-Rc与Arc) | 26-smart-pointers | 单线程/多线程共享所有权 |
| [03 RefCell内部可变性](./03-RefCell内部可变性) | 26-smart-pointers | 运行时借用检查与借用守卫 |
| [04 Deref与Drop](./04-Deref与Drop) | 26-smart-pointers | 解引用强制转换与自动清理 |

## 练习索引

| 练习 | 覆盖 | 出处 |
|---|---|---|
| [Rustlings 19_smart_pointers](https://github.com/rust-lang/rustlings/tree/main/exercises/19_smart_pointers) | Box、Rc、RefCell | [01](./01-Box#实践项) 的实践项 |
| [Exercism Rust track](https://exercism.org/tracks/rust) | 综合 | 01 的实践项 |

## 怎么用本主题

1. 读文章,记住选型三问;
2. 运行 `cargo run -p smart_pointers_demo`,对照输出理解计数增减与析构顺序;
3. 做练习;
4. 对照官方解答自评,说不出「为什么选这种指针」就回到文章。

完成本主题后进入 12-并发编程。

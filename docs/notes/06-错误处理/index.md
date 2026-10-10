# Error Handling(错误处理)

Rust 不用异常:panic! 处理不可恢复的崩溃,`Result<T, E>` 把可恢复的失败编码进返回值,`?` 负责向上传播。本主题 1 篇文章对应 1 个 crate。

## 文章索引

| 文章 | 配套 crate | 一句话 |
|---|---|---|
| [Result 可恢复](./02-Result可恢复) | 19-error-handling | match 必须处理 Result,`?` + `From`/`Box<dyn Error>` 向上传播,unwrap/expect 是失败即 panic 的快捷方式 |

## 练习索引

| 练习 | 覆盖 | 出处 |
|---|---|---|
| [Rustlings 13_error_handling](https://github.com/rust-lang/rustlings/tree/main/exercises/13_error_handling) | Result、`?`、自定义错误 | [Result 可恢复](./02-Result可恢复#实践项) 的实践项 |
| [Rustlings 12_options](https://github.com/rust-lang/rustlings/tree/main/exercises/12_options) | Option 兜底取值 | 同上 |
| [Exercism Rust track](https://exercism.org/tracks/rust) | 综合 | 上文的实践项 |

## 怎么用本主题

1. 读文章,记住「panic 还是 Result」的判断标准;
2. 跑 `cargo run -p error_handling_demo`,对照输出理解每个小节;
3. 做练习;
4. 对照官方解答自评,说不出「为什么错」就回到文章。

完成本主题后进入 07-泛型Trait生命周期。

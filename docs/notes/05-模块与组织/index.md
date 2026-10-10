# Module System(模块与组织)

代码长大就要拆:mod 把代码组织成树,路径定位项,pub 控制可见性。本主题对应仓库 1 个 crate,一篇长文覆盖全部内容。

## 文章索引

| 文章 | 配套 crate | 一句话 |
|---|---|---|
| [01 Package-Crate-Module](./01-Package-Crate-Module) | 18-modules | mod/use/路径/文件组织/pub 可见性,一篇文章讲完 |
| [02 路径与use](./02-路径与use) | 18-modules | 绝对/相对路径、use 与 as 重命名 |
| [03 分离模块文件](./03-分离模块文件) | 18-modules | mod 拆分到独立文件与目录风格 |

## 练习索引

| 练习 | 覆盖 | 出处 |
|---|---|---|
| [Rustlings 10_modules](https://github.com/rust-lang/rustlings/tree/main/exercises/10_modules) | pub、use、模块路径 | [01](./01-Package-Crate-Module#实践项) 的实践项 |
| [RBE - Modules](https://doc.rust-lang.org/stable/rust-by-example/mod.html) | 可见性、文件拆分 | [01](./01-Package-Crate-Module#实践项) 的实践项 |

## 怎么用本主题

1. 读文章,记住模块树与可见性规则;
2. 跑 `cargo run -p modules_demo`,对照输出看每条调用的路径是怎么写出来的;
3. 做 Rustlings 10_modules;
4. 对照官方解答自评,说不出「为什么错」就回到对应文章。

完成本主题后进入 06-错误处理。

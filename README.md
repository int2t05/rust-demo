# rust-demo

Rust 系统学习的代码项目。每个子项目对应一个学习主题，可独立运行。

## 快速开始

```cmd
# 编译所有子项目
cargo build --workspace

# 运行测试
cargo test --workspace

# 运行某个子项目（-p 用 Cargo.toml 里的包名，如 hello_demo）
cargo run -p hello_demo
cargo run -p guessing_game
```

## 学习路径（31 个子项目）

| 编号 | 子项目 | 主题 | 对应教程 |
|---|---|---|---|
| 01 | `01-hello` | Hello World + println! 格式化 | `01-入门基础/02-Hello-Cargo.md` |
| 02 | `02-guessing-game` | 猜数字游戏（综合练习） | `01-入门基础/03-猜数字游戏.md` |
| 03 | `03-variables` | 变量、可变性、shadowing、const | `02-核心语法/01-变量与可变性.md` |
| 04 | `04-scalar-types` | 标量类型（i32/f64/bool/char） | `02-核心语法/02-标量类型.md` |
| 05 | `05-compound-types` | 复合类型（元组、数组） | `02-核心语法/03-复合类型.md` |
| 06 | `06-functions` | 函数、表达式 vs 语句 | `02-核心语法/04-函数.md` |
| 07 | `07-control-flow` | if/loop/while/for/match | `02-核心语法/05-控制流.md` |
| 08 | `08-ownership` | 所有权与移动语义 | `03-所有权系统/01-所有权规则.md` |
| 09 | `09-references` | 引用与借用 | `03-所有权系统/02-引用与借用.md` |
| 10 | `10-slices` | 切片类型 | `03-所有权系统/03-切片类型.md` |
| 11 | `11-lifetimes` | 生命周期标注 | `03-所有权系统/04-生命周期.md` |
| 12 | `12-structs` | 结构体与方法 | `04-数据结构/01-结构体.md` |
| 13 | `13-enums` | 枚举与 Option<T> | `04-数据结构/02-枚举与Option.md` |
| 14 | `14-pattern-matching` | match / if let / 解构 | `04-数据结构/03-Match模式匹配.md` |
| 15 | `15-collections` | Vec<T> 集合 | `04-数据结构/04-Vec集合.md` |
| 16 | `16-strings` | String vs &str | `04-数据结构/05-String字符串.md` |
| 17 | `17-hashmap` | HashMap<K,V> | `04-数据结构/06-HashMap映射.md` |
| 18 | `18-modules` | 模块系统 | `05-模块与组织/01-Package-Crate-Module.md` |
| 19 | `19-error-handling` | panic! / Result / ? | `06-错误处理/` |
| 20 | `20-generics` | 泛型 | `07-泛型Trait生命周期/01-泛型.md` |
| 21 | `21-traits` | Trait 定义与实现 | `07-泛型Trait生命周期/02-Trait特征.md` |
| 22 | `22-trait-objects` | dyn Trait vs 泛型 | `07-泛型Trait生命周期/03-Trait对象.md` |
| 23 | `23-testing` | 单元测试 / 集成测试 / doc test | `08-测试/` |
| 24 | `24-closures` | 闭包 / Fn/FnMut/FnOnce | `09-函数式特性/01-闭包.md` |
| 25 | `25-iterators` | 迭代器与消费器 | `09-函数式特性/02-迭代器.md` |
| 26 | `26-smart-pointers` | Box/Rc/Arc/RefCell | `11-智能指针/` |
| 27 | `27-concurrency` | 线程 / mpsc / Mutex / Send+Sync | `12-并发编程/` |
| 28 | `28-unsafe` | unsafe 五大操作 | `13-高级特性/01-Unsafe-Rust.md` |
| 29 | `29-macros` | macro_rules! 声明宏 | `13-高级特性/03-宏.md` |
| 30 | `30-minigrep` | 综合实战：CLI 工具 | `14-实战项目/01-Cli工具minigrep.md` |
| 31 | `31-web-server` | 综合实战：多线程 Web 服务器 | `14-实战项目/02-多线程Web服务器.md` |

## 目录结构

```
rust-demo/
├── Cargo.toml          # workspace 根
├── README.md           # 本文件
├── .gitignore
├── package.json        # wiki 文档站工具链(npm run docs:dev / docs:build)
├── .github/workflows/  # GH Pages 部署
├── docs/               # VitePress wiki,notes/ 下每主题一个文件夹
└── 01-hello/           # 每个子项目结构相同
    ├── Cargo.toml
    └── src/main.rs
```

## 配套教程

- wiki（在线维护版）：<https://int2t05.github.io/rust-demo/>，源文件在 `docs/notes/`
- 本地笔记：`D:\Myknows\Myknows\原理\Rust\`，每个主题一个文件夹，与 wiki 双向同步

## 环境信息

- Rust：rustup 默认安装（`%USERPROFILE%\.cargo` + `%USERPROFILE%\.rustup`），stable 工具链
- 工具链：stable
- edition：2021

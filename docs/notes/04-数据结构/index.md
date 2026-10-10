# Data Structures(数据结构)

把数据组织成类型的一章:结构体打包字段、枚举列举可能、模式匹配拆开取值、三大集合负责存取,对应仓库 6 个 crate,建议按编号顺序学习。

## 文章索引

| 文章 | 配套 crate | 一句话 |
|---|---|---|
| [01 结构体](./01-结构体) | 12-structs | 三种结构体形态,impl 方法与关联函数 |
| [02 枚举与Option](./02-枚举与Option) | 13-enums | 变体可携带数据,Option 替代 null |
| [03 Match模式匹配](./03-Match模式匹配) | 14-pattern-matching | match 强制穷尽并解构,if let / while let 简写 |
| [04 Vec集合](./04-Vec集合) | 15-collections | 可增长动态数组,索引 vs get,方法链 |
| [05 String字符串](./05-String字符串) | 16-strings | String 与 &str 分层,UTF-8 边界规则 |
| [06 HashMap映射](./06-HashMap映射) | 17-hashmap | 键值 O(1) 查找,entry API 惯用法 |

## 练习索引

| 练习 | 覆盖 | 出处 |
|---|---|---|
| [Rustlings 07_structs](https://github.com/rust-lang/rustlings/tree/main/exercises/07_structs) | 结构体与方法 | [01](./01-结构体#实践项) 的实践项 |
| [Rustlings 08_enums](https://github.com/rust-lang/rustlings/tree/main/exercises/08_enums) | 枚举 | [02](./02-枚举与Option#实践项) 的实践项 |
| [Exercism Rust track](https://exercism.org/tracks/rust) | 模式匹配 | [03](./03-Match模式匹配#实践项) 的实践项 |
| [Rustlings 05_vecs](https://github.com/rust-lang/rustlings/tree/main/exercises/05_vecs) | Vec | [04](./04-Vec集合#实践项) 的实践项 |
| [Rustlings 09_strings](https://github.com/rust-lang/rustlings/tree/main/exercises/09_strings) | 字符串 | [05](./05-String字符串#实践项) 的实践项 |
| [Rustlings 11_hashmaps](https://github.com/rust-lang/rustlings/tree/main/exercises/11_hashmaps) | HashMap | [06](./06-HashMap映射#实践项) 的实践项 |

## 怎么用本主题

1. 按编号读文章:结构体和枚举是匹配与集合的地基,不要跳读;
2. 跑配套 crate(`cargo run -p structs_demo` 一直到 `hashmap_demo`,共 6 个),对照输出读源码;
3. 做练习索引里的对应练习;
4. 对照官方解答自评,说不出「为什么错」就回到对应文章。

完成本主题后进入 05-模块与组织。

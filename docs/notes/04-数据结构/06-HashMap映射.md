# HashMap(映射)

一句话:HashMap 存键值对,靠键的哈希值做 O(1) 平均查找;entry API 是它最值得掌握的惯用法。

## 它解决什么问题

Vec 按位置找元素,但现实查找往往靠「名字」:单词找词频、队伍找得分。HashMap 把任意可哈希的键映射到值,查找、插入、删除平均都是常数时间。

它的难点正好接上所有权:insert 会把键值 move 进 map,取回只能借用或 clone;而「存在就更新、不存在才插入」这类高频逻辑,靠 entry API 一次哈希、拿可变引用原地改,比 get + insert 更快也更短。

## 核心规则

写入与更新的决策路径:

- 键类型必须实现 `Hash + Eq`;String、整数、布尔天然满足,自定义类型需要 derive。
- `get` 返回 `Option<&V>`——「键不存在」是常态而非错误,和 Vec 的 get 同一设计。
- 遍历 `for (k, v) in &map` 只借用;顺序不保证,依赖顺序的输出要另行排序。

## 示例

摘自 `17-hashmap/src/main.rs`:

```rust
// ---- 1. 创建 HashMap ----
let mut scores: HashMap<String, i32> = HashMap::new();

// ---- 2. insert / get ----
scores.insert(String::from("蓝队"), 10);
scores.insert(String::from("红队"), 25);

// get 返回 Option
let red = scores.get("红队");
match red {
    Some(score) => println!("红队得分: {}", score),
    None => println!("找不到红队"),
}

// ---- 4. entry().or_insert() ----
// 如果键不存在则插入默认值，存在则跳过
scores.entry(String::from("黄队")).or_insert(5);
scores.entry(String::from("红队")).or_insert(0); // 红队已存在，不会覆盖

// ---- 6. 所有权 ----
let key = String::from("color");
let value = String::from("red");
let mut own_map = HashMap::new();
own_map.insert(key, value);
// println!("{}", key); // 错误！key 已被 move 进 map
// 从 map 取回只能 clone 或借用
let borrowed = own_map.get("color");
println!("从 map 借用: {:?}", borrowed);
```

## 源码解读

crate:`17-hashmap/src/main.rs`,`cargo run -p hashmap_demo` 可运行。

| 小节 | 演示什么 | 关键行 |
|---|---|---|
| 创建与插入 | new + insert,键值均为拥有型 String | `scores.insert(String::from("蓝队"), 10)` |
| get | 返回 Option<&V>,用 match 消费 | `scores.get("红队")` |
| 遍历 | `for (k, v) in &map` 借用遍历 | `for (team, score) in &scores` |
| entry | 不存在才插入,已有键不覆盖 | `scores.entry(...).or_insert(5)` |
| 更新三式 | insert 覆盖 / entry 跳过 / 基于旧值累加 | `*count += 1` |
| 所有权 | insert move 走键值,取回靠借用 | `own_map.insert(key, value)` |
| Vec 构造 | zip 两个 Vec 再 collect 成 HashMap | `teams.into_iter().zip(...).collect()` |

## 实践项

| 练习 | 内容 | 对照组与评判标准 |
|---|---|---|
| [Rustlings 11_hashmaps](https://github.com/rust-lang/rustlings/tree/main/exercises/11_hashmaps) | hashmaps1~3:基础增查、计数、分组统计 | 先自己写,再对照 [官方解答](https://github.com/rust-lang/rustlings/tree/main/solutions/11_hashmaps);标准:全绿,计数类题用 entry 而非 get + insert |
| [Exercism Rust track](https://exercism.org/tracks/rust) | 任意统计/分组题,用 HashMap 归并数据 | 标准:每个键只哈希一次,看不到对同一键的连续两次查找 |

## 避坑

- insert 用 String 做键后原变量被 move,还要用就先 clone;从 map 取到的是 `Option<&V>`,不是值本身。
- entry().or_insert() 返回可变引用,`*count += 1` 原地改;先 get 再 insert 等于哈希两次,还多写代码。
- 遍历顺序每次运行可能不同,输出依赖顺序的逻辑要先排序或换有序容器。

## 参考资料

- [The Rust Book ch8.3 Storing Keys with Associated Values in Hash Maps](https://doc.rust-lang.org/book/ch08-03-hash-maps.html)
- [Rust By Example - HashMap](https://doc.rust-lang.org/stable/rust-by-example/std/hash.html)

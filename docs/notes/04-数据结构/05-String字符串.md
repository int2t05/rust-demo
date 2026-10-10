# String(字符串)

一句话:Rust 字符串分两层——String 拥有堆上的 UTF-8 字节序列,&str 是借用视图;分清「拥有」和「借用」就不纠结该用哪个。

## 它解决什么问题

字符串本质是字节,而人类文字是字符,两种概念在 UTF-8 变长编码下并不一致(一个汉字 3 字节)。Rust 不假装它们一致:禁止 `s[0]` 这类索引,逼你显式选择按字符(`chars`)还是按字节(`bytes`)处理,从语言层面杜绝「切到半个汉字」。

`String` 与 `&str` 的分层是所有权思想的延续:需要拥有、修改、传递所有权时用 String;只是读一段文本时用 &str——所以函数参数一律写 `&str`,调用方传 String 还是字面量都能自动适配。

## 核心规则

两种类型的选择与衔接:

- UTF-8 是变长编码:ASCII 1 字节,汉字 3 字节;`len()` 返回字节数,字符数要用 `chars().count()`。
- 索引被禁,切片必须落在字符边界上,否则 panic;按字符取前 n 个用 `chars().take(n).collect()`。
- 拼接选型:就地追加用 `push_str`/`push`;`+` 会 move 走左操作数;多段拼接用 `format!`,不夺任何所有权。

## 示例

摘自 `16-strings/src/main.rs`:

```rust
// ---- 1. 创建 String ----
let s1 = String::new();                         // 空 String
let s2 = String::from("hello");                 // 从字面量创建
let s3 = "world".to_string();                   // 从 &str 转换

// ---- 2. &str 切片 ----
let literal: &str = "Rust";                     // 字符串字面量就是 &str
let slice: &str = &s2[0..3];                    // 从 String 取切片

// String 和 &str 的关系：String 拥有堆数据，&str 是引用
// 函数参数优先用 &str（更通用，能接受 &String 和 &str）
fn greet(name: &str) {
    println!("你好, {}", name);
}
greet(&s2);          // &String 自动解引用为 &str
greet("张三");        // &str 字面量

// ---- 3. UTF-8 编码 ----
// Rust 的 String 是 UTF-8 字节序列，不是字符数组
let chinese = String::from("你好");
println!("'你好' 的字节数: {}", chinese.len());   // 6 字节（每汉字 3 字节）

// ---- 5. 拼接 ----
let hello = String::from("Hello, ");
let world = String::from("world!");
// + 运算符：左边必须是 String，右边是 &str
// hello 的所有权会被转移
let hi = hello + &world;

// format! 宏：不夺所有权，适合多段拼接
let s1 = String::from("Rust");
let s2 = String::from("语言");
let combined = format!("{} 编程{}", s1, s2);

// ---- 7. 切片与 UTF-8 边界 ----
let text = String::from("你好世界");
// 安全做法：用 chars().take(n) 取前 n 个字符
let first_two: String = text.chars().take(2).collect();
```

## 源码解读

crate:`16-strings/src/main.rs`,`cargo run -p strings_demo` 可运行。

| 小节 | 演示什么 | 关键行 |
|---|---|---|
| 创建 | new/from/to_string 三种入口 | `"world".to_string()` |
| &str 切片 | 字面量与 String 切片都是 &str | `&s2[0..3]` |
| 参数通用化 | 内嵌 greet 收 &str,两种实参都接 | `greet(&s2)`、`greet("张三")` |
| UTF-8 编码 | 不能索引,len 是字节数 | `chinese.len()` |
| chars/bytes | 按字符与按字节两种遍历 | `chinese.chars()`、`chinese.bytes()` |
| 拼接 | `+` move 左操作数,format! 不夺所有权 | `hello + &world` |
| push | 就地追加 &str 与 char | `mut_s.push_str("bar")` |
| 边界切片 | 切在字符边界才安全,take(n) 替代 | `&text[0..3]` |
| 加工三件套 | trim/replace/split | `"a-b-c".replace('-', "+")` |

## 实践项

| 练习 | 内容 | 对照组与评判标准 |
|---|---|---|
| [Rustlings 09_strings](https://github.com/rust-lang/rustlings/tree/main/exercises/09_strings) | strings1~3:返回切片、传 &str、补字符串方法 | 先自己写,再对照 [官方解答](https://github.com/rust-lang/rustlings/tree/main/solutions/09_strings);标准:全绿,函数参数一律 &str 而非 &String |
| [Exercism Rust track](https://exercism.org/tracks/rust) | 任意文本处理题,用 chars/bytes 正确处理非 ASCII | 标准:中文或 emoji 输入不 panic,字符数与字节数分得清 |

## 避坑

- `String` 不能索引(`s[0]` 编译错误):UTF-8 变长,「第 N 字节」可能是半个字符;要字符用 `chars().nth(i)`。
- `+` 拼接会把左边的 String move 走,原变量失效;要保留原串或拼多段就用 `format!`。
- 字节切片 `&s[0..1]` 不在字符边界就 panic;处理任意文本优先 `chars()`/`split` 等安全 API,不要手算字节下标。

## 参考资料

- [The Rust Book ch8.2 Storing UTF-8 Encoded Text with Strings](https://doc.rust-lang.org/book/ch08-02-strings.html)
- [Rust By Example - str](https://doc.rust-lang.org/stable/rust-by-example/std/str.html)

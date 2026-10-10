// ============================================================
// 16-strings: String vs &str
// ============================================================
// 学习点：
// 1. String::new() / String::from() / to_string()
// 2. &str 字符串切片
// 3. UTF-8 编码：不能索引 s[0]
// 4. chars() / bytes() 遍历
// 5. 拼接 + / format!
// 6. push_str / push
// 7. 切片要小心 UTF-8 边界
//
// 对应教程：docs/notes/04-数据结构/05-String字符串.md
// 运行方式：cargo run -p strings_demo
// ============================================================

fn main() {
    // ---- 1. 创建 String ----
    let s1 = String::new();                         // 空 String
    let s2 = String::from("hello");                 // 从字面量创建
    let s3 = "world".to_string();                   // 从 &str 转换
    println!("s1='{}' s2='{}' s3='{}'", s1, s2, s3);

    // ---- 2. &str 切片 ----
    let literal: &str = "Rust";                     // 字符串字面量就是 &str
    let slice: &str = &s2[0..3];                    // 从 String 取切片
    println!("literal='{}', slice of s2='{}'", literal, slice);

    // String 和 &str 的关系：String 拥有堆数据，&str 是引用
    // 函数参数优先用 &str（更通用，能接受 &String 和 &str）
    fn greet(name: &str) {
        println!("你好, {}", name);
    }
    greet(&s2);          // &String 自动解引用为 &str
    greet("张三");        // &str 字面量

    // ---- 3. UTF-8 编码 ----
    // Rust 的 String 是 UTF-8 字节序列，不是字符数组
    // 所以 s[0] 不合法：字节 0 可能不是完整字符
    let chinese = String::from("你好");
    // println!("{}", chinese[0]); // 编译错误！不能索引 String
    println!("'你好' 的字节数: {}", chinese.len());   // 6 字节（每汉字 3 字节）

    // ---- 4. chars() 与 bytes() ----
    println!("chars(): ");
    for c in chinese.chars() {
        print!("{} ", c);
    }
    println!();

    println!("bytes(): ");
    for b in chinese.bytes() {
        print!("{:02X} ", b);
    }
    println!();

    // ---- 5. 拼接 ----
    let hello = String::from("Hello, ");
    let world = String::from("world!");
    // + 运算符：左边必须是 String，右边是 &str
    // hello 的所有权会被转移
    let hi = hello + &world;
    println!("+ 拼接: {}", hi);

    // format! 宏：不夺所有权，适合多段拼接
    let s1 = String::from("Rust");
    let s2 = String::from("语言");
    let combined = format!("{} 编程{}", s1, s2);
    println!("format!: {}", combined);

    // ---- 6. push_str / push ----
    let mut mut_s = String::from("foo");
    mut_s.push_str("bar");            // 追加 &str
    mut_s.push('!');                   // 追加单个 char
    println!("push 后: {}", mut_s);

    // ---- 7. 切片与 UTF-8 边界 ----
    // 切片必须落在字符边界上，否则 panic
    let text = String::from("你好世界");
    let safe_slice = &text[0..3];     // 第一个汉字（3 字节）
    println!("安全切片: {}", safe_slice);

    // 危险！下面这行会 panic，因为 1 不是字符边界
    // let bad = &text[0..1];

    // 安全做法：用 chars().take(n) 取前 n 个字符
    let first_two: String = text.chars().take(2).collect();
    println!("前两个字符: {}", first_two);

    // ---- 额外：replace / trim / split ----
    let dirty = "  hello   world  ";
    println!("trim: '{}'", dirty.trim());
    let replaced = "a-b-c".replace('-', "+");
    println!("replace: {}", replaced);
    for part in "a,b,c".split(',') {
        println!("  split 段: {}", part);
    }
}

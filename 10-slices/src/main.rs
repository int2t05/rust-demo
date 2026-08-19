// ============================================================
// 10-slices: 切片类型
// ============================================================
// 学习点：
// 1. 字符串切片 &str：引用 String 中一段连续字符
// 2. 切片简写：[..2]、[2..]、[..]
// 3. 字符串字面量本身就是 &str
// 4. 数组切片 &[T]：引用数组中一段连续元素
// 5. 切片不持有所有权，只是借用视图
// 6. 函数参数用 &str 比 &String 更通用
//
// 对应教程：D:\Obsidians\Myknows\原理\Rust\08-Ownership\03-切片.md
// 运行方式：cargo run -p 10-slices
// ============================================================

fn main() {
    // ---- 字符串切片 ----
    // 切片是对 String 中一段连续字节的引用：包含起始指针和长度
    let s = String::from("hello world");
    let slice1 = &s[0..5];   // "hello"，索引按字节
    let slice2 = &s[6..11];  // "world"
    println!("slice1 = {}, slice2 = {}", slice1, slice2);

    // ---- 切片简写 ----
    let hello = &s[..5];     // 省略起始：从 0 开始
    let world = &s[6..];     // 省略结尾：到末尾
    let whole = &s[..];      // 省略两端：整个字符串
    println!("hello = {}, world = {}, whole = {}", hello, world, whole);

    // ---- 字符串字面量就是 &str ----
    // 字面量直接嵌入二进制，类型是 &'static str，生命周期为整个程序
    let literal: &str = "I am a &str";
    println!("literal = {}", literal);

    // ---- 注意：字符串切片按字节索引，不是按字符 ----
    // ASCII 字符一字节，切片安全；中文 UTF-8 编码占 3 字节
    // 若切在多字节字符中间会 panic：let bad = &"你好"[0..1];
    // 正确切法要配合 chars() 等字符迭代器处理非 ASCII 文本

    // ---- 数组切片 ----
    // 数组切片 &[T]：对数组一段连续元素的引用视图
    let arr = [1, 2, 3, 4, 5];
    let arr_slice = &arr[1..3]; // [2, 3]
    println!("数组切片长度 = {}, 内容 = {:?}", arr_slice.len(), arr_slice);

    // 切片不持有所有权：原数据仍归 arr/s 所有
    println!("原数组仍然有效: {:?}", arr);
    println!("原 String 仍然有效: {}", s);

    // ---- 函数参数用 &str 比 &String 更通用 ----
    // &str 既能接收 &String 切片，也能接收字符串字面量，还能接收 &String 整体（自动解引用）
    let word = first_word(&s);          // 传 &String，自动转 &str
    println!("first word = {}", word);
    let literal_word = first_word("rust"); // 直接传 &str 字面量
    println!("literal first word = {}", literal_word);

    // 数组切片同样可以作为函数参数，更通用
    let sum = slice_sum(&arr[..]);
    println!("sum of arr = {}", sum);
    let partial_sum = slice_sum(&arr[1..4]);
    println!("sum of arr[1..4] = {}", partial_sum);
}

// 参数用 &str：既能接收 &String，也能接收 &str 字面量
// Rust 会自动把 &String 转换为 &str（deref coercion）
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i]; // 返回切片，引用同一块内存
        }
    }
    s // 没有空格，整个字符串就是第一个单词
}

// 数组切片作为参数：接收任意长度的 &[i32] 视图
fn slice_sum(nums: &[i32]) -> i32 {
    let mut total = 0;
    for n in nums {
        total += n;
    }
    total
}

// ============================================================
// 15-collections: Vec<T> 集合
// ============================================================
// 学习点：
// 1. Vec::new() 与 vec![] 宏创建向量
// 2. push / pop / get / [] 访问元素
// 3. 遍历 for x in &v
// 4. iter().map().filter().collect() 方法链
// 5. 用 enum 在 Vec 中储存不同类型
// 6. 容量 capacity() 与长度 len()
//
// 对应教程：docs/notes/04-数据结构/04-Vec集合.md
// 运行方式：cargo run -p collections_demo
// ============================================================

use std::collections::VecDeque;

/// 用 enum 表示多种类型，Vec 即可储存异构数据
#[derive(Debug)]
enum Cell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // ---- 1. 创建 Vec ----
    let v1: Vec<i32> = Vec::new();        // 空向量，需标注类型
    let v2 = vec![1, 2, 3, 4, 5];         // 宏初始化，自动推断

    println!("v1 = {:?} (空)", v1);
    println!("v2 = {:?}", v2);

    // ---- 2. 增删访问 ----
    let mut v = Vec::new();
    v.push(10);                            // 末尾添加
    v.push(20);
    v.push(30);
    println!("push 后: {:?}", v);

    let last = v.pop();                    // 弹出末尾
    println!("pop 返回: {:?}, 剩余: {:?}", last, v);

    // 索引访问：越界会 panic
    let first = v[0];
    println!("v[0] = {}", first);

    // get 返回 Option，越界返回 None（更安全）
    match v.get(0) {
        Some(val) => println!("v.get(0) = {}", val),
        None => println!("无此索引"),
    }
    println!("v.get(5) = {:?}", v.get(5)); // None

    // ---- 3. 遍历 ----
    let nums = vec![1, 2, 3, 4, 5];
    print!("遍历 &v: ");
    for x in &nums {
        print!("{} ", x);
    }
    println!();

    // 可变遍历：修改元素
    let mut mut_nums = vec![1, 2, 3];
    for x in &mut mut_nums {
        *x *= 10;
    }
    println!("可变遍历后: {:?}", mut_nums);

    // ---- 4. 方法链 ----
    let doubled: Vec<i32> = nums.iter()
        .map(|&x| x * 2)                  // 每个元素乘 2
        .filter(|&x| x > 5)               // 保留大于 5 的
        .collect();                       // 收集为 Vec
    println!("map+filter+collect: {:?}", doubled);

    // enumerate 带索引遍历
    print!("enumerate: ");
    for (i, val) in nums.iter().enumerate() {
        print!("[{}]={} ", i, val);
    }
    println!();

    // ---- 5. 用 enum 储存不同类型 ----
    let cells = vec![
        Cell::Int(42),
        Cell::Float(3.14),
        Cell::Text(String::from("hello")),
    ];
    for c in &cells {
        println!("  {:?}", c);
    }

    // ---- 6. 容量与长度 ----
    let mut cap_demo: Vec<i32> = Vec::with_capacity(10); // 预分配容量
    println!("空时: len={}, capacity={}", cap_demo.len(), cap_demo.capacity());
    cap_demo.extend(0..5);
    println!("加 5 个后: len={}, capacity={}", cap_demo.len(), cap_demo.capacity());

    // 额外：VecDeque 双端队列
    let mut dq = VecDeque::new();
    dq.push_back(1);
    dq.push_front(0);
    println!("VecDeque: {:?}", dq);
}

// ============================================================
// 06-functions: 函数
// ============================================================
// 学习点：
// 1. fn 定义函数、参数、返回值
// 2. 表达式 vs 语句
// 3. 提前 return
// 4. 无返回值函数返回单元类型 ()
// 5. 值传递与 move 语义
//
// 对应教程：docs/notes/02-核心语法/04-函数.md
// 运行方式：cargo run -p functions_demo
// ============================================================

// --- 函数定义 ---
// fn 函数名(参数名: 类型) -> 返回类型 { 函数体 }
// Rust 命名规范：snake_case（小写+下划线）
fn add(a: i32, b: i32) -> i32 {
    // 最后一个表达式（无分号）作为返回值
    // 这是 Rust 的核心语法：表达式 vs 语句
    a + b  // 注意：没有分号！这是表达式，不是语句
}

// --- 显式 return ---
// 可以用 return 提前返回，但惯用方式是末尾表达式
fn abs(x: i32) -> i32 {
    if x < 0 {
        return -x;  // 提前 return，需要分号
    }
    x  // 末尾表达式作为返回值
}

// --- 无返回值（单元类型 ()）---
// 没有返回类型标注的函数，隐式返回 ()
// 末尾是语句（有分号）时也返回 ()
fn greet(name: &str) {
    // 这个函数没有返回值，实际返回 ()
    println!("你好, {}!", name);
    // 最后一行是语句（有分号），所以返回 ()
}

// 显式标注返回单元类型（少见，但合法）
fn do_nothing() -> () {
    // 什么也不做
}

// --- 表达式 vs 语句 ---
// 语句：执行操作但不返回值，以分号结尾
// 表达式：计算并返回值，没有分号
// 块 {} 的最后一个表达式（无分号）是块的返回值
fn expression_demo() -> i32 {
    // 语句：let 是语句，不返回值
    let x = 5;  // 语句

    // 块表达式：{} 可以作为表达式
    let y = {
        let z = x * 2;
        z + 1  // 块的最后一个表达式（无分号）作为块的值
    };  // y = 11

    y  // 函数的返回表达式
}

// --- Copy 类型的值传递 ---
// i32 是 Copy 类型，传入函数后原变量仍可用
fn double(n: i32) -> i32 {
    n * 2
}

// --- String 等非 Copy 类型会 move ---
// 传入函数后原变量不可再用（ownership 转移）
// 详细的 ownership 会在 08 章讲解
fn print_string(s: String) {
    println!("传入的字符串: {}", s);
    // s 在这里离开作用域，被 drop
}

fn main() {
    // --- 基本函数调用 ---
    let result = add(3, 4);
    println!("add(3, 4) = {}", result);

    // --- 提前 return 的函数 ---
    println!("abs(-5) = {}", abs(-5));
    println!("abs(10) = {}", abs(10));

    // --- 无返回值函数 ---
    greet("Rust 学习者");

    // 接收 () 返回值
    let nothing = do_nothing();
    println!("do_nothing() = {:?}", nothing);  // ()

    // --- 表达式 vs 语句 ---
    let val = expression_demo();
    println!("expression_demo() = {}", val);

    // 块表达式可以直接用在 let 中
    let computed = {
        let a = 10;
        let b = 20;
        a + b  // 块的返回值
    };
    println!("块表达式计算: {}", computed);

    // 语句不能用于赋值，以下会编译错误：
    // let x = (let y = 5);  // 错误：let 是语句，不返回值

    // --- Copy 类型的值传递 ---
    let num = 42;
    let doubled = double(num);
    println!("double({}) = {}", num, doubled);
    // Copy 类型：num 传入后仍可用
    println!("原 num 仍可用: {}", num);

    // --- String 的 move ---
    let text = String::from("Hello");
    print_string(text);
    // print_string 后 text 已 move，以下会编译错误：
    // println!("text: {}", text);  // 错误：value borrowed here after move
    // 注意：函数参数是值传递，但「值」对 String 来说是 ownership
    // 深入的 ownership/引用/borrow 会在 08-11 章讲解

    // 如果想在函数后继续使用 String，可以传入引用 &（借用）
    // 或者 clone 一份传进去
    let text2 = String::from("World");
    print_string(text2.clone());  // clone 传副本
    println!("text2 仍可用: {}", text2);
}

// ============================================================
// 23-testing: 主程序入口
// ============================================================
// 调用 lib.rs 中的函数，演示实际使用
//
// 对应教程：docs/notes/08-测试/03-测试组织与文档测试.md
// 运行方式：cargo run -p testing_demo
// ============================================================

use testing_demo::{gcd, is_palindrome, parse_positive, word_count};

fn main() {
    println!("=== 测试项目演示 ===");

    // 演示 parse_positive
    match parse_positive("42") {
        Ok(n) => println!("parse_positive('42') = {}", n),
        Err(e) => println!("错误: {}", e),
    }
    match parse_positive("abc") {
        Ok(n) => println!("parse_positive('abc') = {}", n),
        Err(e) => println!("parse_positive('abc') 错误: {}", e),
    }

    // 演示 gcd
    println!("gcd(54, 24) = {}", gcd(54, 24));

    // 演示 is_palindrome
    println!("is_palindrome('Radar') = {}", is_palindrome("Radar"));
    println!("is_palindrome('hello') = {}", is_palindrome("hello"));

    // 演示 word_count
    println!("word_count('hello world rust') = {}", word_count("hello world rust"));

    println!("\n运行测试: cargo test -p testing_demo");
}

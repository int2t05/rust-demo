// ============================================================
// 08-ownership: 所有权与移动语义
// ============================================================
// 学习点：
// 1. 栈与堆的区别，String 在堆上分配
// 2. 移动语义：赋值或传参会转移所有权，原变量失效
// 3. Clone trait 显式深拷贝
// 4. Copy trait：i32 等简单类型赋值时复制而非移动
// 5. 函数返回值也能转移所有权
//
// 对应教程：docs/notes/03-所有权系统/01-所有权规则.md
// 运行方式：cargo run -p ownership_demo
// ============================================================

fn main() {
    // ---- 栈 vs 堆 ----
    // 栈：LIFO，存大小固定、编译期已知的数据（如 i32、bool、&str 指针部分）
    // 堆：运行时分配，存大小未知或可能变化的数据（如 String 的字符内容）
    // String 由三部分构成（存栈上）：指针、长度、容量；实际字符内容存堆上
    let s1 = String::from("hello"); // 在堆上分配 "hello"，s1 持有其所有权
    println!("s1 = {}", s1);

    // ---- 移动语义 ----
    // String 没有实现 Copy，赋值时发生 move：s1 的所有权转移给 s2，s1 失效
    // 这是 Rust 的核心设计：避免双重释放，无需 GC 即可保证内存安全
    let s2 = s1;
    // 下面这行若取消注释会编译失败：value borrowed here after move
    // println!("s1 = {}", s1);
    println!("s2 = {}", s2);

    // ---- Clone：显式深拷贝 ----
    // 当确实需要复制堆数据时，调用 clone()。代价较高，但意图明确
    let s3 = String::from("world");
    let s4 = s3.clone(); // s3 仍然有效，s4 是独立副本
    println!("s3 = {}, s4 = {}", s3, s4);

    // ---- Copy trait：栈上简单类型赋值即复制 ----
    // 实现了 Copy 的类型（i32、bool、char、f64、(i32, i32) 等纯栈类型）赋值时复制
    // 规则：实现 Drop 的类型不能实现 Copy；Copy 类型所有字段必须都是 Copy
    let x = 42;
    let y = x;        // i32 是 Copy，x 仍可用
    println!("x = {}, y = {}", x, y);

    // ---- 函数传参也会 move ----
    let s5 = String::from("passed");
    takes_ownership(s5); // s5 的所有权进入函数，函数结束后被 drop
    // 下面这行取消注释会编译失败：s5 已被 move 进函数
    // println!("{}", s5);

    // 仍然可用的 Copy 类型传参：复制一份进去
    let n = 100;
    makes_copy(n);
    println!("after call, n still works: {}", n);

    // ---- 函数返回值转移所有权 ----
    // 函数内部创建的 String 通过返回值把所有权交还给调用者
    let s6 = gives_ownership();     // 所有权从函数内部移动到 s6
    println!("s6 = {}", s6);

    // 传入再返回：所有权从 s7 移动到函数，再从函数移动到 s8
    let s7 = String::from("round-trip");
    let s8 = takes_and_returns(s7);
    // s7 已失效（被 move 进函数）
    println!("s8 = {}", s8);
}

// 接收 String：参数会拿走所有权，函数结束自动 drop
fn takes_ownership(s: String) {
    println!("inside takes_ownership: {}", s);
}

// 接收 i32：Copy 类型，复制一份进来，外部变量不受影响
fn makes_copy(num: i32) {
    println!("inside makes_copy: {}", num);
}

// 返回 String：把内部创建的堆数据所有权交给调用者
fn gives_ownership() -> String {
    String::from("from function") // 作为返回值移动出去
}

// 接收 String 再返回：所有权一进一出
fn takes_and_returns(s: String) -> String {
    s // 直接把传入的 String 移动回去
}

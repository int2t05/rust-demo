// ============================================================
// 09-references: 引用与借用
// ============================================================
// 学习点：
// 1. 不可变引用 &T，可同时存在多个
// 2. 可变引用 &mut T，同一作用域只能有一个
// 3. 不可变引用与可变引用不能同时存在
// 4. 引用必须始终有效，悬垂引用会被编译器拒绝
//
// 对应教程：D:\Obsidians\Myknows\原理\Rust\08-Ownership\02-引用与借用.md
// 运行方式：cargo run -p 09-references
// ============================================================

fn main() {
    // ---- 借用的三条核心规则 ----
    // 规则 1：任意时刻，要么有唯一一个可变引用，要么有任意多个不可变引用
    // 规则 2：引用必须始终有效（不能悬垂）
    // 规则 3：可变引用与不可变引用的作用域不能重叠
    // 这些规则由编译器在编译期通过借用检查器（borrow checker）强制执行

    // ---- 不可变引用 &String：借用数据但不获取所有权 ----
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // 传入引用，s1 所有权仍在
    println!("'{}' 的长度是 {}", s1, len);

    // 可以同时有多个不可变引用：只读共享是安全的
    let r1 = &s1;
    let r2 = &s1;
    println!("多个不可变引用共存: {} {}", r1, r2);

    // ---- 可变引用 &mut String：能修改被引用的数据 ----
    let mut s2 = String::from("hi");
    change(&mut s2); // 传入可变引用，函数内可修改
    println!("修改后: {}", s2);

    // 同一时间只能有一个可变引用：防止数据竞争
    let mut s3 = String::from("mutable");
    let r3 = &mut s3;
    // 下面这行取消注释会编译失败：cannot borrow s3 as mutable more than once
    // let r4 = &mut s3;
    println!("唯一的可变引用: {}", r3);
    // r3 在上一行最后一次使用后失效，这里可以再借用
    let r4 = &mut s3; // 这是合法的：r3 已不再使用（NLL 借用检查）
    r4.push_str("!");
    println!("再次借用后: {}", r4);

    // ---- 不可变引用与可变引用不能同时存在 ----
    let mut s4 = String::from("conflict");
    let r5 = &s4; // 不可变借用开始
    // 下面这行取消注释会编译失败：可变借用与不可变借用冲突
    // let r6 = &mut s4;
    println!("不可变借用使用完毕: {}", r5);
    // 此处 r5 已结束使用（NLL），下面可以可变借用
    let r6 = &mut s4;
    r6.push_str(" ok");
    println!("切换到可变借用后: {}", r6);

    // ---- 正确做法演示：返回 String 而非引用 ----
    let safe = no_dangle();
    println!("no_dangle 返回: {}", safe);
}

// 接收不可变引用：只读访问，不获取所有权
fn calculate_length(s: &String) -> usize {
    s.len()
    // s 在这里失效，但它只是引用，不 drop 任何堆数据
}

// 接收可变引用：可以修改被引用的数据
fn change(s: &mut String) {
    s.push_str(", world");
}

// ---- 悬垂引用：编译器拒绝 ----
// 下面这个函数若取消注释会编译失败：missing lifetime specifier
// 因为返回的引用指向的 String 在函数结束就被 drop，引用会悬垂
// fn dangle() -> &String {
//     let s = String::from("dangling");
//     &s // s 在函数结束被释放，引用变成悬垂指针
// }
// 正确做法：直接返回 String，把所有权交还给调用者
fn no_dangle() -> String {
    String::from("safe")
}

// ============================================================
// 07-control-flow: 控制流
// ============================================================
// 学习点：
// 1. if / else if / else 条件分支
// 2. if 是表达式（可返回值）
// 3. loop 无限循环（break 返回值, continue）
// 4. while 条件循环
// 5. for in 遍历（range, 数组, Vec）
// 6. match 基础模式匹配
// 7. if let 简化 Option 处理
//
// 对应教程：docs/notes/02-核心语法/05-控制流.md
// 运行方式：cargo run -p control_flow_demo
// ============================================================

fn main() {
    // --- 1. if / else if / else ---
    // 条件必须是 bool 类型，Rust 不会自动转换类型
    let number = 6;
    if number % 4 == 0 {
        println!("{} 能被 4 整除", number);
    } else if number % 3 == 0 {
        println!("{} 能被 3 整除", number);
    } else {
        println!("{} 不能被 4 或 3 整除", number);
    }

    // --- 2. if 是表达式 ---
    // if 可以返回值，所有分支的返回类型必须一致
    let condition = true;
    let result = if condition { 5 } else { 6 };
    println!("if 表达式结果: {}", result);
    // 以下会编译错误（类型不一致）：
    // let bad = if true { 5 } else { "six" };

    // --- 3. loop 无限循环 ---
    // loop 会一直执行，直到 break；break 可以返回值
    let mut count = 0;
    let loop_result = loop {
        count += 1;
        if count == 5 {
            break count * 2;  // break 带返回值
        }
    };
    println!("loop 返回值: {}", loop_result);  // 10

    // continue 跳过本次循环
    print!("奇数: ");
    let mut n = 0;
    loop {
        n += 1;
        if n > 10 { break; }
        if n % 2 == 0 { continue; }
        print!("{} ", n);
    }
    println!();

    // --- 4. while 条件循环 ---
    // 条件为 false 时退出
    let mut countdown = 3;
    while countdown > 0 {
        println!("倒计时: {}", countdown);
        countdown -= 1;
    }
    println!("发射！");

    // --- 5. for in 遍历 ---
    // 1..=5 闭区间（含 5），1..5 左闭右开（不含 5）
    print!("1..=5: ");
    for i in 1..=5 { print!("{} ", i); }
    println!();

    // 遍历数组
    let arr = [10, 20, 30, 40, 50];
    print!("数组: ");
    for elem in arr.iter() { print!("{} ", elem); }  // .iter() 借用遍历
    println!();

    // 遍历 Vec（后续 collections 章详讲）
    let fruits = vec!["apple", "banana", "cherry"];
    print!("Vec: ");
    for fruit in &fruits { print!("{} ", fruit); }  // & 借用，不消费 fruits
    println!();

    // rev() 反转遍历
    print!("反转: ");
    for i in (1..=5).rev() { print!("{} ", i); }
    println!();

    // --- 6. match 基础模式匹配 ---
    // match 必须穷尽所有可能；_ 是通配符
    // 深入的 match 在 14 章讲解，这里只做基础演示
    let coin = 25;
    let coin_name = match coin {
        1 => "一分",
        5 => "五分",
        10 => "一角",
        25 => "两角五",
        50 => "五角",
        _ => "未知面值",
    };
    println!("硬币 {} 分 = {}", coin, coin_name);

    // --- 7. if let 简化 Option 处理 ---
    // 当只关心一种情况时，if let 比 match 更简洁
    let some_value: Option<i32> = Some(42);

    // 用 match 处理（需要穷尽两个分支）
    match some_value {
        Some(val) => println!("match: 值是 {}", val),
        None => println!("match: 没有值"),
    }

    // 用 if let 处理（更简洁，else 可选）
    if let Some(val) = some_value {
        println!("if let: 值是 {}", val);
    }

    // if let 处理 None 的情况
    let none_value: Option<i32> = None;
    if let Some(val) = none_value {
        println!("不会执行: {}", val);
    } else {
        println!("none_value 是 None");
    }
}

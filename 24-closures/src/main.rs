// ============================================================
// 24-closures: 闭包 / Fn / FnMut / FnOnce
// ============================================================
// 学习点：
// 1. 闭包语法：|x| x + 1、|x: i32| -> i32 x + 1
// 2. 捕获环境变量：不可变 &、可变 &mut、move
// 3. Fn / FnMut / FnOnce 三种 trait 的区别
// 4. 闭包作为参数：fn apply(f: impl Fn(i32) -> i32)
// 5. 闭包作为返回值：Box<dyn Fn()>
// 6. move 关键字转移所有权
// 7. 闭包与函数对比
//
// 对应教程：docs/notes/09-函数式特性/01-闭包.md
// 运行方式：cargo run -p closures_demo
// ============================================================

/// 一个普通函数，用于和闭包对比
fn add_one_fn(x: i32) -> i32 {
    x + 1
}

/// 接受闭包作为参数：使用 impl Fn 语法
/// Fn(i32) -> i32 表示：可被不可变调用、接收 i32、返回 i32 的闭包
fn apply(f: impl Fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}

/// 接受 FnMut 闭包：允许闭包内部修改自己捕获的状态
fn apply_mut(mut f: impl FnMut(i32), x: i32) {
    f(x);
}

/// 返回一个闭包：必须用 Box 包装，因为闭包大小在编译期未知
/// 这里使用 move 把捕获的 start 值移动进闭包，保证闭包生命周期独立
fn make_adder(start: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x + start)
}

/// 演示 FnOnce：只能调用一次的闭包（会消耗捕获的值）
fn consume_fn<F: FnOnce() -> String>(f: F) -> String {
    f()
}

fn main() {
    println!("===== 1. 闭包基础语法 =====");
    // 类型推断可省略：|x| x + 1
    let add_v1 = |x| x + 1;
    // 显式标注：|x: i32| -> i32
    let add_v2 = |x: i32| -> i32 { x + 1 };
    // 多参数
    let mul = |a: i32, b: i32| a * b;

    println!("add_v1(5) = {}", add_v1(5));
    println!("add_v2(5) = {}", add_v2(5));
    println!("mul(3, 4) = {}", mul(3, 4));

    // 与普通函数对比：函数不能捕获环境，闭包可以
    println!("add_one_fn(5) = {}", add_one_fn(5));

    println!("\n===== 2. 捕获环境变量 =====");
    // (a) 不可变借用捕获：默认行为，闭包以 & 方式引用外部变量
    let base = 10;
    let add_base = |x| x + base; // 借用 base
    println!("add_base(5) = {} (base 仍可用: {})", add_base(5), base);

    // (b) 可变借用捕获：闭包内部需要修改外部变量
    let mut counter = 0;
    let mut increment = || {
        counter += 1; // 可变借用 counter
    };
    increment();
    increment();
    // 闭包还在作用域内时 counter 被可变借用，不能同时访问
    // println!("{}", counter); // 编译错误
    drop(increment); // 释放借用
    println!("counter after 2 increments = {}", counter);

    // (c) move 捕获：把变量所有权转移进闭包
    let name = String::from("Rust");
    let greet = move || {
        println!("hello, {}!", name); // name 被移动进闭包
    };
    greet();
    // println!("{}", name); // 编译错误：name 已被 move

    println!("\n===== 3. Fn / FnMut / FnOnce 三种 trait =====");
    // FnOnce：所有闭包都实现，至少能调用一次。会消耗捕获值（如 move String 并消耗）
    // FnMut：可多次调用，可修改捕获的变量（可变借用）
    // Fn：可多次调用，不可修改捕获的变量（不可变借用）
    // 层级：Fn ⊂ FnMut ⊂ FnOnce（实现 Fn 的也实现 FnMut、FnOnce）

    let s = String::from("data");
    // 这个闭包消耗 s，只能调用一次 -> FnOnce
    let once = || {
        println!("consuming: {}", s);
        s // 返回 s，转移所有权给调用者
    };
    let result = consume_fn(once);
    println!("  consume_fn 返回: {}", result);

    // FnMut 示例：累加器
    let mut total = 0;
    let mut accumulator = |x: i32| {
        total += x;
        println!("  accumulated: total = {}", total);
    };
    apply_mut(accumulator, 10);
    apply_mut(|x: i32| println!("  empty mut call: {}", x), 99);

    println!("\n===== 4. 闭包作为参数 =====");
    println!("apply(|x| x * 2, 21) = {}", apply(|x| x * 2, 21));
    println!("apply(add_one_fn, 41) = {}", apply(add_one_fn, 41));

    println!("\n===== 5. 闭包作为返回值 =====");
    let adder = make_adder(100);
    println!("make_adder(100)(23) = {}", adder(23));

    // 经典模式：函数工厂
    let multipliers: Vec<Box<dyn Fn(i32) -> i32>> = vec![
        make_adder(1),
        make_adder(10),
        make_adder(100),
    ];
    for (i, m) in multipliers.iter().enumerate() {
        println!("  multiplier[{}](5) = {}", i, m(5));
    }

    println!("\n===== 6. move 在多线程场景的必要性（概念演示） =====");
    // 线程闭包通常需要 move，把所有权转移到新线程
    let data = vec![1, 2, 3];
    let handle = std::thread::spawn(move || {
        println!("  in thread: {:?}", data);
    });
    handle.join().unwrap();

    println!("\n===== 7. 闭包 vs 函数对比 =====");
    // 函数：不能捕获环境，签名固定，零成本
    // 闭包：可捕获环境，可作为值传递，三种 trait 灵活
    // 性能：闭包在编译期展开为匿名结构体 + call 方法，零成本抽象
    let captured = 5;
    let closure_eq = |x| x + captured; // 捕获 captured
    let func_eq = add_one_fn;            // 不捕获

    println!("closure_eq(10) = {}", closure_eq(10));
    println!("func_eq(10)    = {}", func_eq(10));
}

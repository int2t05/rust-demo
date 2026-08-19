// ============================================================
// 03-variables: 变量、可变性、shadowing、const
// ============================================================
// 学习点：
// 1. let 声明不可变变量
// 2. let mut 声明可变变量
// 3. shadowing 变量遮蔽（可改变类型）
// 4. const 编译时常量
// 5. static 静态变量
//
// 对应教程：D:\Obsidians\Myknows\原理\Rust\02-基础\01-variables.md
// 运行方式：cargo run -p 03-variables
// ============================================================

// const 编译时常量：
// - 必须显式标注类型
// - 必须用常量表达式初始化（不能调用函数）
// - 命名规范：全大写 + 下划线分隔
// - 内联到使用处，没有固定内存地址
const MAX_POINTS: u32 = 100_000;

// static 静态变量：
// - 有固定内存地址，程序运行期间一直存在
// - 默认不可变（可变需 unsafe，后续章节讲）
// - 生命周期是整个程序（'static）
static GREETING: &str = "Hello from static";

fn main() {
    // --- 1. let 不可变变量 ---
    // Rust 默认所有变量都不可变，这是 Rust 安全性的核心之一
    let x = 5;
    println!("x = {}", x);
    // x = 6;  // 编译错误！cannot assign twice to immutable variable
    // 如需修改，必须用 let mut 声明

    // --- 2. let mut 可变变量 ---
    let mut y = 10;
    println!("y 初始 = {}", y);
    y = 20; // mut 声明的变量可以重新赋值
    println!("y 修改后 = {}", y);

    // --- 3. shadowing 变量遮蔽 ---
    // 用 let 重新声明同名变量，新变量遮蔽（覆盖）旧变量
    // shadowing 与 mut 不同：
    //   - shadowing 创建了新变量，可以改变类型
    //   - mut 是修改同一变量，不能改变类型
    let z = "42";       // z 是 &str 类型
    println!("z 是字符串: {}", z);
    let z: i32 = z.parse().unwrap(); // z 被 shadow 为 i32 类型
    println!("z 变成数字: {}", z);
    let z = z + 8;      // 继续遮蔽，类型不变
    println!("z 加 8 后: {}", z);

    // shadowing 的另一个用途：转换后保持变量名不变，避免 z_str / z_num 这种命名

    // --- 4. const 常量 ---
    // const 在编译时完全内联，不能 shadow
    println!("MAX_POINTS = {}", MAX_POINTS);

    // --- 5. static 静态变量 ---
    // static 有固定内存地址，可以获取其地址
    println!("GREETING = {}", GREETING);

    // --- 6. 变量类型标注 ---
    // Rust 有类型推导，通常不需要显式标注
    // 但有时需要帮助编译器（泛型、字面量类型不明确时）
    let a = 5;          // 推导为 i32（默认整数类型）
    let b: u64 = 5;     // 显式指定 u64
    let c = 5u64;       // 字面量后缀指定类型
    println!("a(i32)={}, b(u64)={}, c(u64)={}", a, b, c);

    // --- 7. 未使用变量警告 ---
    // 变量名前加 _ 可以消除未使用警告
    // _x 表示「我知道这个变量没用到，别警告我」
    let _unused = 100;
    // 注意：_x 和 x 是不同的变量，_x 不会 move

    // --- 8. 解构赋值 ---
    // 后面 compound-types 会详讲，这里简单演示
    let (p, q) = (1, 2);
    println!("解构: p={}, q={}", p, q);
}

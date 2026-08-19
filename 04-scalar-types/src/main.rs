// ============================================================
// 04-scalar-types: 标量类型
// ============================================================
// 学习点：
// 1. 整数类型：i8/i16/i32/i64/i128/isize, u8/u16/u32/u64/u128/usize
// 2. 整数字面量：0xff 0b101 0o77 1_000 b'A'
// 3. 浮点类型：f32, f64
// 4. bool 布尔类型
// 5. char Unicode 字符（4 字节）
// 6. 整数溢出行为
//
// 对应教程：D:\Obsidians\Myknows\原理\Rust\02-基础\02-scalar-types.md
// 运行方式：cargo run -p 04-scalar-types
// ============================================================

fn main() {
    // --- 1. 整数类型 ---
    // i 带符号（可负），u 无符号（非负）
    // 数字表示位数：8/16/32/64/128
    // isize/usize 是指针宽度类型，64 位系统上是 64 位
    let a: i8 = -128;           // i8 范围: -128 ~ 127
    let b: u8 = 255;            // u8 范围: 0 ~ 255
    let c: i32 = -100;          // i32 是默认整数类型
    let d: u64 = 1_000_000;     // u64 常用于大计数
    let e: i128 = 999_999_999_999_999;
    let f: usize = 42;          // usize 常用于数组索引和大小
    println!("i8={} u8={} i32={} u64={} i128={} usize={}", a, b, c, d, e, f);

    // --- 2. 整数字面量写法 ---
    // 0x 前缀：十六进制
    // 0b 前缀：二进制
    // 0o 前缀：八进制
    // 1_000：下划线提升可读性（不影响数值）
    // b'A'：字节字面量（u8），等于字符 A 的 ASCII 码
    let hex = 0xff;          // 255
    let bin = 0b1010;        // 10
    let oct = 0o77;          // 63
    let readable = 1_000_000; // 1000000
    let byte_char = b'A';    // 65
    println!("hex={} bin={} oct={} readable={} byte_char={}", hex, bin, oct, readable, byte_char);

    // --- 3. 浮点类型 ---
    // f32 单精度（4 字节），f64 双精度（8 字节，默认）
    let f32_val: f32 = 3.14159265;
    let f64_val: f64 = 3.141592653589793;
    println!("f32: {}", f32_val);        // 精度约 6-7 位
    println!("f64: {}", f64_val);        // 精度约 15-16 位

    // 浮点数运算
    let sum = 0.1 + 0.2;
    println!("0.1 + 0.2 = {}", sum);     // 注意浮点精度问题：0.30000000000000004

    // --- 4. bool 布尔类型 ---
    // bool 只有两个值：true / false，占 1 字节
    let is_ready: bool = true;
    let is_done = false;
    println!("is_ready={} is_done={}", is_ready, is_done);

    // bool 可以做逻辑运算
    let both = is_ready && is_done;  // 逻辑与
    let either = is_ready || is_done; // 逻辑或
    let not_ready = !is_ready;       // 逻辑非
    println!("both={} either={} not_ready={}", both, either, not_ready);

    // --- 5. char 字符类型 ---
    // Rust 的 char 是 Unicode 标量值，占 4 字节
    // 可以表示中文、emoji、符号等
    let letter: char = 'A';
    let chinese = '中';
    let emoji = '😀';
    let heart = '❤';
    println!("letter={} chinese={} emoji={} heart={}", letter, chinese, emoji, heart);

    // char 的 Unicode 码点
    println!("'中' 的 Unicode 码点: {}", '中' as u32);

    // --- 6. 整数溢出行为 ---
    // debug 模式：溢出会 panic（程序崩溃）
    // release 模式：溢出会 wrapping（回绕），不会 panic
    //
    // 示例（debug 下会 panic，所以注释掉）：
    // let max_u8: u8 = 255;
    // let overflow = max_u8 + 1;  // debug: panic! release: 变成 0

    // 显式处理溢出的方法：
    let max_u8: u8 = 255;
    // wrapping_add：溢出时回绕（无论 debug/release 都不 panic）
    let wrapped = max_u8.wrapping_add(1);  // 结果 0
    println!("255 wrapping_add 1 = {}", wrapped);

    // checked_add：返回 Option，溢出时返回 None
    let checked = max_u8.checked_add(1);
    println!("255 checked_add 1 = {:?}", checked); // None

    // saturating_add：溢出时饱和到最大值
    let saturated = max_u8.saturating_add(1);
    println!("255 saturating_add 1 = {}", saturated); // 255

    // overflowing_add：返回 (结果, 是否溢出)
    let (result, overflowed) = max_u8.overflowing_add(1);
    println!("255 overflowing_add 1 = ({}, {})", result, overflowed); // (0, true)
}

// ============================================================
// 01-hello: Hello World 与 println! 格式化
// ============================================================
// 学习点：
// 1. println! 宏的基本用法与格式化占位符
// 2. {} {:?} {:#?} {:>10} {:<10} {:^10} {:.2} {:b} {:o} {:x} {:X} {:e}
// 3. 命名参数 {name}
// 4. print! vs println! vs eprintln! 的区别
// 5. 转义字符 \t \n \\ \"
//
// 对应教程：D:\Obsidians\Myknows\原理\Rust\01-入门\01-hello.md
// 运行方式：cargo run -p 01-hello
// ============================================================

fn main() {
    // --- 1. 最基础的 Hello World ---
    // println! 是宏（macro），不是函数，以 ! 结尾
    // 宏在编译时展开，比函数更灵活，可以接受可变参数
    println!("Hello, World!");

    // --- 2. 基本格式化 {} ---
    // {} 调用 Display trait，适合人类阅读的输出
    println!("{} 今年 {} 岁", "小明", 18);

    // --- 3. Debug 格式化 {:?} 和 {:#?} ---
    // {:?} 调用 Debug trait，单行输出
    // {:#?} 多行美化输出，带缩进，适合复杂结构
    let nums = vec![1, 2, 3]; // 这里用 vec 演示，后面 collections 会详讲
    println!("Debug 单行: {:?}", nums);
    println!("Debug 美化:\n{:#?}", nums);

    // --- 4. 对齐格式化 ---
    // {:>10} 右对齐，总宽度 10
    // {:<10} 左对齐，总宽度 10
    // {:^10} 居中对齐，总宽度 10
    println!("右对齐: |{:>10}|", "right");
    println!("左对齐: |{:<10}|", "left");
    println!("居中对齐: |{:^10}|", "center");

    // --- 5. 浮点数精度 {:.2} ---
    // {:.2} 保留 2 位小数
    let pi = 3.14159265;
    println!("保留 2 位: {:.2}", pi);
    println!("保留 4 位: {:.4}", pi);

    // --- 6. 进制格式化 ---
    // {:b} 二进制, {:o} 八进制, {:x} 小写十六进制, {:X} 大写十六进制
    let n: i32 = 255;
    println!("十进制: {}", n);
    println!("二进制: {:b}", n);
    println!("八进制: {:o}", n);
    println!("十六进制小写: {:x}", n);
    println!("十六进制大写: {:X}", n);

    // --- 7. 科学计数法 {:e} ---
    println!("科学计数法: {:e}", 1_000_000.0);

    // --- 8. 命名参数 ---
    // {name} 使用命名参数，比位置参数更清晰
    println!(
        "{name} 的成绩: 语文 {chinese}, 数学 {math}",
        name = "小红",
        chinese = 95,
        math = 98
    );

    // --- 9. print! vs println! vs eprintln! ---
    // print!   不换行输出到 stdout
    // println! 换行输出到 stdout
    // eprintln! 输出到 stderr（标准错误流），适合错误信息
    print!("这段文字");
    print!("不会自动换行");
    println!("；直到 println! 才换行");

    eprintln!("这条消息输出到 stderr（标准错误流）");

    // --- 10. 转义字符 ---
    // \t 制表符, \n 换行, \\ 反斜杠, \" 双引号
    println!("制表符:\tA\tB\tC");
    println!("换行:第一行\n第二行");
    println!("反斜杠: C:\\Users\\name");
    println!("双引号: 他说\"你好\"");

    // --- 11. 原始字符串 r"" ---
    // r"" 原始字符串，不处理转义
    println!(r"原始字符串: C:\Users\name 不需要转义");

    // --- 12. 多行字符串 ---
    // 字符串字面量可以跨行
    println!(
        "这是
多行
字符串"
    );
}

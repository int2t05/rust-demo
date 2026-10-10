// ============================================================
// 02-guessing-game: 猜数字游戏（综合练习）
// ============================================================
// 学习点：
// 1. use 导入标准库和外部 crate
// 2. rand::thread_rng().gen_range() 生成随机数
// 3. io::stdin().read_line() 读取用户输入
// 4. trim().parse() 字符串转数字
// 5. match 匹配 Ordering 三分支
// 6. loop + break 循环控制
//
// 对应教程：docs/notes/01-入门基础/03-猜数字游戏.md
// 运行方式：cargo run -p guessing_game
// ============================================================

// use 导入需要的模块
// std::io 提供标准输入输出
// rand::Rng trait 提供随机数生成方法（必须导入 trait 才能用 gen_range）
// std::cmp::Ordering 是比较结果枚举：Less / Equal / Greater
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("=== 猜数字游戏 ===");
    println!("我已经想好了一个 1 到 100 之间的数字，来猜猜看！");

    // 生成随机数：rand::thread_rng() 获取当前线程的随机数生成器
    // gen_range(1..=100) 生成 [1, 100] 闭区间的随机整数
    // ..= 是包含上界的范围语法
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // loop 无限循环，直到 break 才退出
    loop {
        println!("\n请输入你猜的数字：");

        // mut 声明可变变量（Rust 默认不可变）
        // String::new() 创建空字符串
        let mut guess = String::new();

        // io::stdin() 获取标准输入句柄
        // read_line(&mut guess) 把用户输入读入 guess 字符串
        // &mut 表示可变引用（后续章节详讲）
        // .expect() 处理可能的 io::Error，失败时打印消息并 panic
        io::stdin()
            .read_line(&mut guess)
            .expect("读取输入失败");

        // guess.trim() 去除首尾空白（包括用户输入的换行符 \n）
        // .parse::<u32>() 尝试把字符串解析为 u32 无符号整数
        // 返回 Result<u32, ParseIntError>，用 match 处理
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,           // 解析成功，取出数字
            Err(_) => {               // _ 通配所有错误
                println!("请输入有效的数字！");
                continue;             // 跳过本次循环，重新输入
            }
        };

        println!("你猜的是: {}", guess);

        // guess.cmp(&secret_number) 比较两个值，返回 Ordering
        // match 对 Ordering 的三个变体分别处理
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了！"),    // guess < secret
            Ordering::Greater => println!("太大了！"), // guess > secret
            Ordering::Equal => {                      // guess == secret
                println!("恭喜你猜对了！");
                break;                                // 猜对后退出循环
            }
        }
    }
}

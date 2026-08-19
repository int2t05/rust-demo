// ============================================================
// 14-pattern-matching: match / if let / 解构
// ============================================================
// 学习点：
// 1. match 表达式：必须穷尽所有可能
// 2. 通配符 _：匹配剩余情况
// 3. 解构结构体、元组、数组、枚举
// 4. if let：只关心一种情况时的简写
// 5. while let：循环中匹配
// 6. matches! 宏：布尔判定简写
//
// 对应教程：D:\Obsidians\Myknows\原理\Rust\13-Enums\02-match.md
// 运行方式：cargo run -p 14-pattern-matching
// ============================================================

struct User {
    name: String,
    age: u32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    // ---- match 表达式：必须穷尽 ----
    let coin = 25;
    let cents = match coin {
        1 => "一分",
        5 => "五分",
        10 => "一角",
        25 => "两角五",
        _ => "未知面值", // 通配符：处理剩余所有情况，保证穷尽
    };
    println!("硬币面值: {}", cents);

    // ---- 解构结构体 ----
    let user = User {
        name: String::from("Alice"),
        age: 30,
    };
    let User { name, age } = user; // 字段名必须一致
    println!("解构结构体: name={}, age={}", name, age);

    // ---- 解构元组 ----
    let tup = (10, "hello", 3.14);
    let (a, b, c) = tup;
    println!("解构元组: a={}, b={}, c={}", a, b, c);

    // ---- 解构数组 ----
    let arr = [1, 2, 3, 4, 5];
    let [first, second, .., last] = arr; // .. 忽略中间元素
    println!("解构数组: first={}, second={}, last={}", first, second, last);

    // ---- 解构枚举 ----
    let msgs: Vec<Message> = vec![
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("hi")),
        Message::ChangeColor(255, 0, 0),
    ];
    for m in msgs {
        match m {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move: x={}, y={}", x, y),
            Message::Write(text) => println!("Write: {}", text),
            Message::ChangeColor(r, g, b) => println!("Color: ({},{},{})", r, g, b),
        }
    }

    // ---- if let：只关心一种情况时的简写 ----
    let some_value: Option<i32> = Some(42);
    if let Some(v) = some_value {
        println!("if let 捕获到: {}", v);
    } else {
        println!("None");
    }

    // if let 也能解构枚举结构体变体
    let msg = Message::Move { x: 5, y: 8 };
    if let Message::Move { x, y } = msg {
        println!("if let 解构 Move: x={}, y={}", x, y);
    }

    // ---- while let：循环中匹配，直到不匹配为止 ----
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("while let 弹出: {}", top);
    }

    // ---- matches! 宏：返回布尔值，简写单个模式判定 ----
    let opt: Option<i32> = Some(5);
    if matches!(opt, Some(_)) {
        println!("matches! 判定 opt 是 Some");
    }
    let msg = Message::Quit;
    if matches!(msg, Message::Quit) {
        println!("matches! 判定 msg 是 Quit");
    }
}

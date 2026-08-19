// ============================================================
// 13-enums: 枚举与 Option<T>
// ============================================================
// 学习点：
// 1. 简单枚举：无数据变体
// 2. 带值枚举：每个变体可携带不同类型数据
// 3. 类单元、结构体、元组混合变体
// 4. Option<T>：替代 null 的安全抽象
// 5. Option 的常用方法：is_some、is_none、unwrap、unwrap_or、map
//
// 对应教程：D:\Obsidians\Myknows\原理\Rust\13-Enums\01-枚举.md
// 运行方式：cargo run -p 13-enums
// ============================================================

// ---- 简单枚举 ----
enum IpAddrKind {
    V4,
    V6,
}

// ---- 带值枚举：每个变体携带同类型数据 ----
enum IpAddr {
    V4(String),
    V6(String),
}

// ---- 混合变体：单元、结构体、元组三种形式可共存 ----
enum Message {
    Quit,                       // 类单元变体：无数据
    Move { x: i32, y: i32 },    // 结构体变体：命名字段
    Write(String),              // 元组变体：单值
    ChangeColor(i32, i32, i32), // 元组变体：多值
}

fn main() {
    // ---- 简单枚举 ----
    let kind4 = IpAddrKind::V4;
    print_ip_kind(kind4);
    let kind6 = IpAddrKind::V6;
    print_ip_kind(kind6);

    // ---- 带值枚举 ----
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    print_ip_addr(home);
    print_ip_addr(loopback);

    // ---- 混合变体：构造不同形态 ----
    let msgs: Vec<Message> = vec![
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("hello")),
        Message::ChangeColor(255, 0, 0),
    ];
    for m in msgs {
        describe_message(m);
    }

    // ---- Option<T>：Rust 没有 null，用 Option 表示可能缺失 ----
    // Some(value) 表示有值，None 表示无值
    let some_num: Option<i32> = Some(42);
    let no_num: Option<i32> = None;

    // ---- Option 的常用方法 ----
    // is_some / is_none：判断是否有值（只读借用，不消耗 Option）
    println!("some_num.is_some() = {}", some_num.is_some());
    println!("no_num.is_none() = {}", no_num.is_none());

    // unwrap：有值返回值，无值 panic（生产代码慎用）
    // unwrap 会消耗 Option，所以这里用独立变量演示
    let value = some_num.unwrap();
    println!("some_num.unwrap() = {}", value);

    // unwrap_or：无值时返回默认值（消耗 Option）
    let fallback = no_num.unwrap_or(0);
    println!("no_num.unwrap_or(0) = {}", fallback);

    // map：对内部的值做变换，Some 时应用函数，None 时保持 None
    // map 也消耗 Option，每个演示用独立变量，避免 move 冲突
    let doubled = Some(42).map(|x| x * 2);
    println!("Some(42).map(|x| x*2) = {:?}", doubled);

    // None 需要显式类型标注，否则 map 闭包参数类型无法推断
    let none_doubled: Option<i32> = Option::<i32>::None.map(|x| x * 2);
    println!("None.map(|x| x*2) = {:?}", none_doubled);
}

fn print_ip_kind(kind: IpAddrKind) {
    match kind {
        IpAddrKind::V4 => println!("IPv4"),
        IpAddrKind::V6 => println!("IPv6"),
    }
}

fn print_ip_addr(addr: IpAddr) {
    match addr {
        IpAddr::V4(s) => println!("IPv4: {}", s),
        IpAddr::V6(s) => println!("IPv6: {}", s),
    }
}

fn describe_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quit：无数据变体"),
        Message::Move { x, y } => println!("Move: x={}, y={}", x, y),
        Message::Write(text) => println!("Write: {}", text),
        Message::ChangeColor(r, g, b) => println!("ChangeColor: ({},{},{})", r, g, b),
    }
}

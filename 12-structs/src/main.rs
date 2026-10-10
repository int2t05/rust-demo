// ============================================================
// 12-structs: 结构体与方法
// ============================================================
// 学习点：
// 1. 命名结构体：字段带名字
// 2. 字段简写：变量名与字段名相同时
// 3. 元组结构体：字段无名有顺序
// 4. 单元结构体：无字段，用于实现 trait
// 5. impl 块定义方法：&self、&mut self、self
// 6. 关联函数：类似构造器，用 Self:: 调用
// 7. 多个 impl 块：Rust 允许对同一类型分散实现
//
// 对应教程：docs/notes/04-数据结构/01-结构体.md
// 运行方式：cargo run -p structs_demo
// ============================================================

// ---- 命名结构体 ----
struct User {
    name: String,
    age: u32,
    active: bool,
}

// ---- 元组结构体 ----
struct Color(i32, i32, i32);

// ---- 单元结构体 ----
struct AlwaysEqual;

// ---- 方法与关联函数实现 ----
impl User {
    // 关联函数（无 self）：类似构造器，用 User::new 调用
    fn new(name: String, age: u32) -> Self {
        Self { name, age, active: true } // 字段简写：name 变量名与字段名相同
    }

    // 方法 &self：只读访问
    fn greeting(&self) -> String {
        let status = if self.active { "活跃" } else { "停用" };
        format!("Hi, I'm {}, age {}, status={}", self.name, self.age, status)
    }

    // 方法 &mut self：可变修改
    fn birthday(&mut self) {
        self.age += 1;
    }

    // 方法 self（不是引用）：获取所有权，调用后实例被消耗
    fn into_name(self) -> String {
        self.name
    }
}

// ---- 多个 impl 块：Rust 允许分散写 ----
impl User {
    fn is_adult(&self) -> bool {
        self.age >= 18
    }
}

fn main() {
    // ---- 命名结构体实例化 ----
    let user1 = User {
        name: String::from("Alice"),
        age: 30,
        active: true,
    };
    println!("{}", user1.greeting());

    // 字段简写：当变量名与字段名相同时
    let name = String::from("Bob");
    let age = 25;
    let user2 = User { name, age, active: false };
    println!("{}", user2.greeting());

    // ---- 关联函数调用：类似构造器 ----
    let user3 = User::new(String::from("Carol"), 17);
    println!("{}", user3.greeting());

    // ---- &mut self 方法 ----
    let mut user4 = User::new(String::from("Dave"), 20);
    user4.birthday();
    println!("过完生日: {}", user4.greeting());

    // ---- 元组结构体 ----
    let red = Color(255, 0, 0);
    println!("RGB = ({}, {}, {})", red.0, red.1, red.2);

    // ---- 单元结构体 ----
    let _marker = AlwaysEqual; // 常用于类型层面占位

    // ---- self 方法（消耗实例）----
    let user5 = User::new(String::from("Eve"), 40);
    let name = user5.into_name(); // user5 被消耗
    println!("取出的 name = {}", name);
    // 下面这行取消注释会编译失败：user5 已被 move
    // println!("{}", user5.greeting());

    // ---- 多个 impl 块的方法 ----
    let user6 = User::new(String::from("Frank"), 18);
    println!("{} 是否成年: {}", user6.greeting(), user6.is_adult());
}

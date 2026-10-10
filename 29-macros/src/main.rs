// ============================================================
// 29-macros: macro_rules! 声明宏
// ============================================================
// 学习点：
// 1. macro_rules! 定义
// 2. $x:expr 等片段说明符
// 3. 重复 $(...)* / $(...)+
// 4. 递归宏
// 5. 宏 vs 函数（编译时展开、可变参数）
// 6. 导出 #[macro_export]
// 7. 实用示例：vec! 简化版、自定义 println
//
// 对应教程：docs/notes/13-高级特性/03-宏.md
// 运行方式：cargo run -p macros_demo
// ============================================================

// ===== 1. 最简单的宏：单分支 =====
// $x:expr 表示 $x 匹配一个表达式
macro_rules! say_hello {
    () => {
        println!("hello from macro!")
    };
}

// ===== 2. 带参数的宏 =====
macro_rules! print_value {
    ($x:expr) => {
        println!("{} = {}", stringify!($x), $x);
    };
}

// ===== 3. 重复：$(...)* 零次或多次，$(...)+ 一次或多次 =====
// 用逗号分隔，末尾逗号可选（$(,)?）
macro_rules! my_vec {
    ( $( $x:expr ),* $(,)? ) => {
        {
            let mut v = Vec::new();
            $(
                v.push($x);
            )*
            v
        }
    };
}

// ===== 4. 递归宏：逐步处理参数 =====
// 计算多个数的和
macro_rules! my_sum {
    // 单个值：递归终止
    ($single:expr) => {
        $single
    };
    // 多个值：第一个 + 剩余的递归求和
    ($first:expr, $($rest:expr),*) => {
        $first + my_sum!($($rest),*)
    };
}

// ===== 5. 自定义 println（简化版） =====
// 展示 fmt 字面量拼接
macro_rules! my_println {
    ($fmt:literal) => {
        println!($fmt)
    };
    ($fmt:literal, $($arg:expr),*) => {
        println!($fmt, $($arg),*)
    };
}

// ===== 6. 条件编译宏：不同的分支匹配不同语法 =====
macro_rules! math {
    // 加法
    (add $a:expr, $b:expr) => {
        $a + $b
    };
    // 乘法
    (mul $a:expr, $b:expr) => {
        $a * $b
    };
    // 三元运算
    (max $a:expr, $b:expr) => {
        if $a > $b { $a } else { $b }
    };
}

// ===== 7. 块作为参数：tt（token tree）匹配任意 =====
macro_rules! measure_time {
    ($name:expr, $body:block) => {
        {
            let start = std::time::Instant::now();
            let result = $body;
            let elapsed = start.elapsed();
            println!("  {} 耗时: {:?}", $name, elapsed);
            result
        }
    };
}

// ===== 8. 导出宏：#[macro_export] 让宏可被其他 crate 使用 =====
#[macro_export]
macro_rules! assert_non_empty {
    ($s:expr) => {
        if $s.is_empty() {
            panic!("{} 不应为空", stringify!($s));
        }
    };
}

// ===== 片段说明符速查（注释） =====
// expr   : 表达式，如 1+2、foo()
// ident  : 标识符，如 x、foo
// ty     : 类型，如 i32、Vec<u8>
// pat    : 模式，如 Some(x)、_
// stmt   : 语句，如 let x = 1;
// literal: 字面量，如 "hi"、42
// tt     : token tree，匹配任意 token
// vis    : 可见性，如 pub
// meta   : 元属性，如 #[derive(Debug)]
// lifetime: 生命周期，如 'a
// path   : 路径，如 std::vec::Vec

fn main() {
    println!("===== 1. 最简单的宏 =====");
    say_hello!();

    println!("\n===== 2. 带参数的宏 =====");
    let x = 42;
    print_value!(x);
    print_value!(x + 1);
    print_value!(x * 2);

    println!("\n===== 3. 重复构造 vec =====");
    let v1 = my_vec![1, 2, 3, 4, 5];
    println!("  my_vec!: {:?}", v1);
    let v2 = my_vec!["a", "b", "c",];
    println!("  末尾逗号: {:?}", v2);
    let v3: Vec<i32> = my_vec![];
    println!("  空 vec: {:?}", v3);

    println!("\n===== 4. 递归宏求和 =====");
    let s1 = my_sum!(1);
    let s2 = my_sum!(1, 2);
    let s3 = my_sum!(1, 2, 3, 4, 5);
    println!("  my_sum!(1) = {}", s1);
    println!("  my_sum!(1,2) = {}", s2);
    println!("  my_sum!(1,2,3,4,5) = {}", s3);

    println!("\n===== 5. 自定义 println =====");
    my_println!("纯字面量");
    my_println!("带参数: x={}, s={}", x, "hello");

    println!("\n===== 6. 条件分支宏 =====");
    println!("  add(3,4) = {}", math!(add 3, 4));
    println!("  mul(3,4) = {}", math!(mul 3, 4));
    println!("  max(7,2) = {}", math!(max 7, 2));

    println!("\n===== 7. 块参数宏 =====");
    let n = measure_time!("空循环1e6", {
        let mut sum = 0u64;
        for i in 0..1_000_000 {
            sum += i;
        }
        sum
    });
    println!("  结果 = {}", n);

    println!("\n===== 8. assert_non_empty =====");
    let s = String::from("hello");
    assert_non_empty!(s);
    println!("  assert_non_empty!(s) 通过");
    // let empty: String = String::new();
    // assert_non_empty!(empty); // 会 panic

    println!("\n===== 9. 宏 vs 函数 =====");
    // 函数：运行时调用，类型固定，签名明确
    // 宏：编译时展开，可接受可变参数和任意语法，生成代码
    fn add(a: i32, b: i32) -> i32 { a + b }
    println!("  函数 add(2,3) = {}", add(2, 3));
    println!("  宏   my_sum!(2,3,4) = {}", my_sum!(2, 3, 4)); // 可变参数

    // 宏能做的事函数做不到：
    // - 实现类似 println! 的格式化（任意数量参数）
    // - 生成结构体/枚举（如 derive 宏，声明宏也能做简化版）
    println!("\n===== 10. 用宏生成结构体（演示） =====");
    macro_rules! make_struct {
        ($name:ident { $($field:ident : $ty:ty),* $(,)? }) => {
            #[derive(Debug, Clone)]
            struct $name {
                $($field: $ty),*
            }
        };
    }
    make_struct!(Point { x: f64, y: f64 });
    let p = Point { x: 1.0, y: 2.5 };
    println!("  宏生成的结构体: {:?}", p);

    make_struct!(Person { name: String, age: u32 });
    let person = Person { name: String::from("Alice"), age: 30 };
    println!("  另一个: {:?}", person);

    println!("\n===== 总结 =====");
    println!("  macro_rules! 定义声明宏");
    println!("  片段说明符: expr/ident/ty/tt/literal 等");
    println!("  重复: $(...)* 和 $(...)+");
    println!("  递归：多分支 + 自调用");
    println!("  导出：#[macro_export]");
    println!("  优势：编译时展开、可变参数、生成代码");
}

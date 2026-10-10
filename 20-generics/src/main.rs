// ============================================================
// 20-generics: 泛型
// ============================================================
// 学习点：
// 1. 泛型函数 largest<T: PartialOrd>
// 2. 泛型结构体 Point<T>
// 3. 多类型参数 Point<T, U>
// 4. 泛型方法 impl<T> Point<T>
// 5. 泛型枚举 Option<T> / Result<T, E>
// 6. 性能：单态化（零成本抽象）
//
// 对应教程：docs/notes/07-泛型Trait生命周期/01-泛型.md
// 运行方式：cargo run -p generics_demo
// ============================================================

/// 泛型函数：找出切片中最大的元素
/// T: PartialOrd 约束 T 必须支持比较
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut max = &list[0];
    for item in &list[1..] {
        if item > max {
            max = item;
        }
    }
    max
}

/// 泛型结构体：坐标点，x 和 y 同类型
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

/// 多类型参数：x 和 y 可以是不同类型
#[derive(Debug)]
struct MixedPoint<T, U> {
    x: T,
    y: U,
}

// 泛型方法实现
impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }

    fn get_y(&self) -> &T {
        &self.y
    }
}

// 为特定类型添加额外方法
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// MixedPoint 的泛型方法：混合另一个 Point 的坐标
impl<T, U> MixedPoint<T, U> {
    /// 用当前 x 和另一个点的 y 组合成新点
    fn mix<V, W>(self, other: MixedPoint<V, W>) -> MixedPoint<T, W> {
        MixedPoint { x: self.x, y: other.y }
    }
}

fn main() {
    println!("=== 泛型演示 ===");

    // ---- 1. 泛型函数 ----
    let nums = vec![34, 50, 25, 100, 65];
    let chars = vec!['y', 'm', 'a', 'q'];
    println!("最大整数: {}", largest(&nums));
    println!("最大字符: {}", largest(&chars));

    // ---- 2 & 4. 泛型结构体与方法 ----
    let p_int = Point { x: 5, y: 10 };
    let p_float = Point { x: 1.0, y: 4.0 };
    println!("整数点: {:?}, x={}", p_int, p_int.get_x());
    println!("浮点点: {:?}, 距离原点={:.2}", p_float, p_float.distance_from_origin());

    // ---- 3. 多类型参数 ----
    let mixed = MixedPoint { x: 5, y: 3.14 };
    let mixed2 = MixedPoint { x: "Hello", y: 'c' };
    println!("混合点1: {:?}", mixed);
    println!("混合点2: {:?}", mixed2);

    // mix 方法：组合不同类型
    let m1 = MixedPoint { x: 1, y: 2.0 };
    let m2 = MixedPoint { x: "a", y: 'b' };
    let m3 = m1.mix(m2);
    println!("mix 后: {:?}", m3);

    // ---- 5. 泛型枚举（标准库的 Option 和 Result） ----
    let some_int: Option<i32> = Some(42);
    let some_str: Option<&str> = Some("hello");
    println!("Option<i32>: {:?}, Option<&str>: {:?}", some_int, some_str);

    let ok: Result<i32, &str> = Ok(100);
    let err: Result<i32, &str> = Err("失败");
    println!("Result: {:?}, {:?}", ok, err);

    // ---- 6. 单态化说明 ----
    // 泛型在编译时生成具体类型的副本，如 largest_i32 和 largest_char
    // 运行时零额外开销，这就是"零成本抽象"
    println!("\n[单态化] 编译时为每种类型生成专用代码，运行时零开销");
}

// ============================================================
// 22-trait-objects: dyn Trait vs 泛型
// ============================================================
// 学习点：
// 1. 静态分发（泛型，编译时单态化）
// 2. 动态分发（dyn Trait，运行时虚表）
// 3. Box<dyn Trait> 在 Vec 中存不同类型
// 4. trait object 的对象安全限制
// 5. 对比性能（注释说明）
//
// 对应教程：docs/notes/07-泛型Trait生命周期/03-Trait对象.md
// 运行方式：cargo run -p trait_objects_demo
// ============================================================

/// 共同的 trait：可绘制图形
trait Drawable {
    fn draw(&self) -> String;
    fn area(&self) -> f64;
}

struct Circle { radius: f64 }
struct Rectangle { width: f64, height: f64 }
struct Triangle { base: f64, height: f64 }

impl Drawable for Circle {
    fn draw(&self) -> String {
        format!("画一个半径为 {} 的圆", self.radius)
    }
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Drawable for Rectangle {
    fn draw(&self) -> String {
        format!("画一个 {}x{} 的矩形", self.width, self.height)
    }
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Drawable for Triangle {
    fn draw(&self) -> String {
        format!("画一个底{}高{}的三角形", self.base, self.height)
    }
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

/// 1. 静态分发：泛型 + trait bound
/// 编译时为每种类型生成专用代码（单态化），零运行时开销
fn draw_static<T: Drawable>(shape: &T) {
    println!("[静态] {} 面积={:.2}", shape.draw(), shape.area());
}

/// 2. 动态分发：dyn Trait
/// 运行时通过虚表查找方法，有一次间接调用
fn draw_dynamic(shape: &dyn Drawable) {
    println!("[动态] {} 面积={:.2}", shape.draw(), shape.area());
}

/// 4. 对象安全的 trait 示例
/// 下面的 trait 不是对象安全的（方法返回 Self），不能用 dyn
// trait Cloneable {
//     fn clone_me(&self) -> Self; // 返回 Self，不对象安全
// }

/// 对象安全的 trait：方法不返回 Self，无泛型参数
trait Greetable {
    fn greet(&self) -> String;
    // fn make(&self) -> Self;        // 不安全：返回 Self
    // fn process<T>(&self, x: T);   // 不安全：有泛型参数
}

struct Person { name: String }
impl Greetable for Person {
    fn greet(&self) -> String {
        format!("你好，我是 {}", self.name)
    }
}

fn main() {
    println!("=== Trait 对象 vs 泛型 ===");

    let circle = Circle { radius: 5.0 };
    let rect = Rectangle { width: 3.0, height: 4.0 };
    let tri = Triangle { base: 6.0, height: 3.0 };

    // ---- 1. 静态分发 ----
    println!("--- 静态分发（泛型）---");
    draw_static(&circle);
    draw_static(&rect);
    draw_static(&tri);

    // ---- 2. 动态分发 ----
    println!("--- 动态分发（dyn Trait）---");
    draw_dynamic(&circle);
    draw_dynamic(&rect);
    draw_dynamic(&tri);

    // ---- 3. Box<dyn Trait> 存不同类型 ----
    // 泛型 Vec<T> 只能存同类型；Box<dyn Trait> 可存不同类型
    println!("--- Vec<Box<dyn Drawable>> ---");
    let shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle { radius: 2.0 }),
        Box::new(Rectangle { width: 3.0, height: 4.0 }),
        Box::new(Triangle { base: 5.0, height: 2.0 }),
    ];
    for shape in &shapes {
        println!("  {} 面积={:.2}", shape.draw(), shape.area());
    }
    // 总面积
    let total: f64 = shapes.iter().map(|s| s.area()).sum();
    println!("  总面积: {:.2}", total);

    // ---- 4. 对象安全示例 ----
    let person = Person { name: String::from("张三") };
    let greet_box: Box<dyn Greetable> = Box::new(person);
    println!("--- 对象安全 trait ---");
    println!("  {}", greet_box.greet());

    // ---- 5. 性能对比说明 ----
    println!("--- 性能对比 ---");
    println!("  静态分发：编译时生成专用代码，零运行时开销，但二进制更大");
    println!("  动态分发：运行时虚表查找，一次间接调用，更灵活但略慢");
    println!("  选择：需要异构集合用 dyn，追求性能用泛型");
}

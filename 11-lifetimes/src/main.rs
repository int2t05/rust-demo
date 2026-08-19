// ============================================================
// 11-lifetimes: 生命周期标注
// ============================================================
// 学习点：
// 1. 为什么需要生命周期：防止悬垂引用
// 2. 函数生命周期标注：'a 连接参数与返回值的生命关系
// 3. 'static 生命周期：整个程序运行期
// 4. 生命周期省略规则（3 条）
// 5. 结构体中的生命周期：引用字段必须标注
//
// 对应教程：D:\Obsidians\Myknows\原理\Rust\08-Ownership\04-生命周期.md
// 运行方式：cargo run -p 11-lifetimes
// ============================================================

// ---- 函数生命周期标注 ----
// 'a 是生命周期参数：表示返回值的引用至少和 x、y 中较短的那个活一样久
// 编译器据此检查：返回的引用不会比两个参数活得更长
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// ---- 结构体中的生命周期 ----
// 结构体持有引用时必须标注生命周期：确保引用字段不会比结构体实例活得更短
struct Excerpt<'a> {
    part: &'a str, // part 引用的数据必须比 Excerpt 实例活得久
}

impl<'a> Excerpt<'a> {
    // 返回 &str，省略规则下自动推断为 &'a str
    fn announce(&self) -> &str {
        self.part
    }
}

fn main() {
    // ---- 为什么需要生命周期 ----
    // 没有生命周期标注时，编译器无法判断返回的引用来自 x 还是 y
    // 从而无法保证返回引用不悬垂。标注 'a 让编译器做这个检查
    let s1 = String::from("long string");
    let s2 = String::from("hi");
    let result = longest(s1.as_str(), s2.as_str());
    println!("较长的: {}", result);

    // ---- 正确使用：返回的引用作用域内，源数据仍然有效 ----
    {
        let s3 = String::from("another long one");
        let r = longest(s1.as_str(), s3.as_str());
        println!("块内 r = {}", r);
    } // s3 在这里 drop，但 r 已在块内用完，没有悬垂

    // ---- 'static 生命周期 ----
    // 'static 表示引用在整个程序运行期间都有效
    // 所有字符串字面量都是 &'static str：直接嵌入二进制，永不失效
    // 'static 并不一定是"手动标注"，而是编译器对字面量的默认推断
    // 注意：把 'static 当作"活得够久"的兜底方案是常见误用，应优先精确标注
    let static_str: &'static str = "I live forever";
    println!("static = {}", static_str);

    // ---- 生命周期省略规则（3 条） ----
    // 规则 1：每个引用参数都有自己的生命周期
    // 规则 2：只有一个输入引用参数时，返回值生命周期取它
    // 规则 3：有 &self 或 &mut self 时，返回值生命周期取 self
    // 三条规则都套不上时，编译器要求显式标注
    let s = String::from("hello world");
    let ex = Excerpt { part: &s[..5] }; // part 引用 s 的一段
    println!("excerpt = {}", ex.announce());

    // ---- 误用示例（编译器会拒绝） ----
    // 下面这种写法若取消注释会编译失败：r 引用的内部 String 在函数结束被 drop
    // let r;
    // {
    //     let inner = String::from("inner");
    //     r = longest(inner.as_str(), "static");
    // }
    // println!("{}", r); // inner 已 drop，r 会悬垂
}

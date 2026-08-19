// ============================================================
// 21-traits: Trait 定义与实现
// ============================================================
// 学习点：
// 1. trait 定义
// 2. impl Trait for Type
// 3. 默认实现
// 4. 调用 trait 方法
// 5. trait 作为参数 fn show(item: &impl Display)
// 6. trait bound fn show<T: Display>(item: &T)
// 7. 多 trait bound 用 +
// 8. where 子句
// 9. 实现 trait 的 trait
//
// 对应教程：D:\Obsidians\Myknows\原理\Rust\18-泛型与Trait\02-Trait.md
// 运行方式：cargo run -p 21-traits
// ============================================================

use std::fmt::{Debug, Display};

/// 1. 定义 trait：抽象行为契约
trait Summary {
    /// 必须实现的方法
    fn summarize(&self) -> String;

    /// 3. 默认实现：可选择覆盖
    fn author(&self) -> String {
        String::from("(未知作者)")
    }
}

/// 2. 新闻文章结构体
#[derive(Debug)]
struct NewsArticle {
    title: String,
    author: String,
    content: String,
}

/// 2. 为 NewsArticle 实现 Summary
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} - {}", self.title, self.author)
    }
    // 覆盖默认实现
    fn author(&self) -> String {
        self.author.clone()
    }
}

/// 推文结构体
struct Tweet {
    username: String,
    text: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.text)
    }
    // 不覆盖 author，使用默认实现
}

/// 为 Tweet 实现 Display，让它同时拥有 Summary + Display
impl Display for Tweet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Tweet({})", self.username)
    }
}

/// 5. trait 作为参数：impl Trait 语法（语法糖）
fn print_summary(item: &impl Summary) {
    println!("摘要: {}", item.summarize());
    println!("作者: {}", item.author());
}

/// 6. trait bound 完整写法（等价于上面的 impl Trait）
fn print_summary_bound<T: Summary>(item: &T) {
    println!("[bound] {}", item.summarize());
}

/// 7. 多个 trait bound 用 + 连接
/// 同时要求实现 Summary 和 Display
fn show_summary_and_display<T: Summary + Display>(item: &T) {
    println!("[多bound] {} | 摘要: {}", item, item.summarize());
}

/// 8. where 子句：让复杂签名更清晰
/// T 需要 Summary + Debug，U 需要 Summary + Display
fn show_both<T, U>(t: &T, u: &U)
where
    T: Summary + Debug,
    U: Summary + Display,
{
    println!("[where] T={:?} 摘要={} | U={} 摘要={}", t, t.summarize(), u, u.summarize());
}

/// 9. 为实现了某 trait 的类型添加额外方法
/// 这里：任何实现了 Display 的类型都能用 shout
trait Shout {
    fn shout(&self) -> String;
}

// blanket implementation：为所有实现了 Display 的类型实现 Shout
impl<T: Display> Shout for T {
    fn shout(&self) -> String {
        format!("{}!!!", self)
    }
}

fn main() {
    println!("=== Trait 演示 ===");

    // 4. 调用 trait 方法
    let article = NewsArticle {
        title: String::from("Rust 1.97 发布"),
        author: String::from("官方"),
        content: String::from("新增多项特性..."),
    };
    let tweet = Tweet {
        username: String::from("rustlang"),
        text: String::from("Hello Rust!"),
    };

    println!("文章: {}", article.summarize());
    println!("推文: {}", tweet.summarize());
    println!("推文作者(默认): {}", tweet.author());

    // 5. impl Trait 参数
    print_summary(&article);

    // 6. trait bound
    print_summary_bound(&tweet);

    // 7. 多 trait bound
    // Tweet 同时实现了 Summary 和 Display
    show_summary_and_display(&tweet);

    // 8. where 子句
    // article 有 Summary + Debug，tweet 有 Summary + Display
    show_both(&article, &tweet);

    // 9. blanket implementation
    // 所有实现了 Display 的类型自动有了 shout 方法
    println!("42 shout: {}", 42i32.shout());
    println!("hello shout: {}", "hello".shout());
    println!("3.14 shout: {}", 3.14f64.shout());
}

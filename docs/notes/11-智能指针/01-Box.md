# Smart Pointers(智能指针)

一句话:所有权默认一份数据一个主人,智能指针在这条规则之上补齐堆分配、多所有者共享和内部可变三类能力。

## 它解决什么问题

所有权规则要求每份数据恰好一个所有者、大小编译期已知,但有些场景规则直接表达不了:递归类型大小无限、要把不同类型装进同一集合、或多个所有者共享同一份数据。智能指针是带额外元数据的结构体,统一实现 Deref(能像引用一样使用)和 Drop(离开作用域自动清理),让堆数据和栈数据用起来没有区别。

本 crate 覆盖四件套:Box 独占堆数据,Rc/Arc 共享只读(单线程/多线程),RefCell 提供内部可变,`Rc<RefCell<T>>` 是「多所有者 + 可变」的经典组合。

## 核心规则

选型只看三个问题:几个所有者、要不要修改、跨不跨线程。

- Box 是起点:数据上堆,栈上只留固定大小的指针,递归类型与 trait 对象都靠它把「大小未知」变成「大小已知」。
- Rc/Arc 只解决「共享」不解决「修改」;修改要叠加内部可变性:单线程用 RefCell,多线程用 Mutex。
- Arc 的每次计数更新都是原子操作,单线程场景别顺手用 Arc。

## 示例

摘自 `26-smart-pointers/src/main.rs`:

```rust
// 第 1 节 Box:递归类型必须用 Box 把递归部分放堆上,大小才有限
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}
let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));

// 第 2 节 Rc:clone 计数加一,离开作用域减一,归零才释放
let a = Rc::new(String::from("shared data"));
let b = Rc::clone(&a);
println!("clone 后 strong_count = {}", Rc::strong_count(&a));
// ……(块内创建 c 计数到 3,块结束回落到 2,省略)

// 第 3 节 Arc:原子计数,数据可 move 进线程
let arc = Arc::new(vec![1, 2, 3]);
let arc_clone = Arc::clone(&arc);
let handle = std::thread::spawn(move || {
    println!("  线程内 Arc: {:?}", arc_clone);
});
handle.join().unwrap();

// 第 4 节 RefCell:运行时借用检查
let cell = RefCell::new(vec![1, 2, 3]);
let mut r = cell.borrow_mut();
r.push(4);
r.push(5);
// ……(borrow 只读演示、双 borrow_mut 会 panic 的注释,省略)

// 第 5 节 Rc<RefCell<T>>:多所有者 + 可变
let shared = Rc::new(RefCell::new(vec![1, 2, 3]));
let owner1 = Rc::clone(&shared);
let owner2 = Rc::clone(&shared);
owner1.borrow_mut().push(4);
owner2.borrow_mut().push(5);

// 第 6 节 Deref:自定义实现 + 标准库强转
let x = 5;
let mybox = MyBox::new(x);
println!("MyBox 解引用: {} (等价于 *mybox = {})", *mybox, *mybox);

let s = String::from("hello");
let s_ref: &str = &s; // String: Deref<Target=str>

// 第 7 节 Drop:离开作用域按声明逆序析构
let boxed = Box::new(Droppy { name: "boxed" });
drop(boxed); // 显式 drop
```

## 源码解读

crate:`26-smart-pointers/src/main.rs`,`cargo run -p smart_pointers_demo` 可运行(MyBox 与 Droppy 定义在 main 上方)。

| 小节 | 演示什么 | 关键行 |
|---|---|---|
| 1. `Box<T>` | 堆分配;递归类型用 Box 限定大小;`Box<dyn Debug>` 装 trait 对象 | `List::Cons(1, Box::new(...))`、`Vec<Box<dyn Debug>>` |
| 2. `Rc<T>` | 单线程引用计数:clone 加一,离开作用域减一,归零释放 | `Rc::clone(&a)`、`Rc::strong_count(&a)` |
| 3. `Arc<T>` | 原子计数,数据 move 进线程后两边都能读 | `Arc::clone(&arc)`、`thread::spawn(move ...)` |
| 4. `RefCell<T>` | 运行时借用检查:borrow 只读、borrow_mut 可写,冲突 panic | `cell.borrow()`、`cell.borrow_mut()` |
| 5. `Rc<RefCell<T>>` | 多所有者 + 内部可变,任一 owner 改动所有 owner 可见 | `owner1.borrow_mut().push(4)` |
| 6. Deref 强制转换 | 自定义 Deref(MyBox);标准库 &String 到 &str 的强转 | `impl Deref for MyBox<T>`、`let s_ref: &str = &s` |
| 7. Drop 析构顺序 | 变量按声明逆序 drop;drop() 可提前显式析构 | `impl Drop for Droppy`、`drop(boxed)` |
| 8. 综合对比表 | 六种组合的适用场景一览 | 结尾对比输出 |

## 实践项

| 练习 | 内容 | 对照组与评判标准 |
|---|---|---|
| [Rustlings 19_smart_pointers](https://github.com/rust-lang/rustlings/tree/main/exercises/19_smart_pointers) | 用 Box 修递归类型、Rc 共享数据、RefCell 实现内部可变 | 先自己写,再对照 [官方解答](https://github.com/rust-lang/rustlings/tree/main/solutions/19_smart_pointers);标准:全绿,且每个练习能说出「为什么选这种指针」 |
| [Exercism Rust track](https://exercism.org/tracks/rust) | 任意含递归/共享数据的题,选对智能指针 | 标准:编译通过,且能解释为什么裸引用或 clone 数据不是更好的解 |

## 避坑

- RefCell 违规是 panic 不是编译错误:borrow_mut 还活着时再调 borrow()/borrow_mut() 会运行时崩溃,借用尽快释放(像 crate 第 4 节那样包在块里)。
- Rc 循环引用会泄漏:双方 strong_count 永不归零;「父→子、子→父」结构要用 Weak 打破环。
- Box 只解决「大小未知」,不解决「多所有者」;发现靠 clone 数据绕所有权,该换的是 Rc/Arc 而不是继续加 clone。

## 参考资料

- [The Rust Book ch15.1 Using Box](https://doc.rust-lang.org/book/ch15-01-box.html)
- [The Rust Book ch15.4 Rc](https://doc.rust-lang.org/book/ch15-04-rc.html)
- [The Rust Book ch15.5 Interior Mutability](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html)
- [The Rust Book ch15.2 Deref](https://doc.rust-lang.org/book/ch15-02-deref.html)
- [The Rust Book ch15.3 Drop](https://doc.rust-lang.org/book/ch15-03-drop.html)
- [Rust By Example - Arc](https://doc.rust-lang.org/stable/rust-by-example/std/arc.html)

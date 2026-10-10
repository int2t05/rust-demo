// ============================================================
// 26-smart-pointers: Box / Rc / Arc / RefCell
// ============================================================
// 学习点：
// 1. Box<T>：堆分配，独占所有权
// 2. Rc<T>：引用计数，单线程共享
// 3. Arc<T>：原子引用计数，多线程共享
// 4. RefCell<T>：运行时借用检查，内部可变性
// 5. Rc<RefCell<T>> 组合：多所有权 + 可变
// 6. Deref 强制转换
// 7. Drop trait 析构顺序
//
// 对应教程：docs/notes/11-智能指针/01-Box.md
// 运行方式：cargo run -p smart_pointers_demo
// ============================================================

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;

/// 演示 Drop trait：观察析构顺序
struct Droppy {
    name: &'static str,
}

impl Drop for Droppy {
    fn drop(&mut self) {
        println!("  Droppy({}) 被 drop", self.name);
    }
}

/// 演示 Deref：自定义解引用行为
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    println!("===== 1. Box<T> =====");
    // Box 把数据放到堆上，栈上只保留指针
    // 适用场景：递归类型、大数据转移、trait 对象
    let b = Box::new(5);
    println!("Box<i32> = {}", b); // 自动解引用

    // 递归类型：Cons 列表（必须用 Box 限定大小）
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
    println!("Cons list = {:?}", list);

    // Box 作为 trait 对象
    let shapes: Vec<Box<dyn std::fmt::Debug>> = vec![Box::new(1), Box::new("hi"), Box::new(3.14)];
    println!("trait 对象: {:?}", shapes);

    println!("\n===== 2. Rc<T> 引用计数（单线程） =====");
    // Rc 允许多个所有者共享同一数据
    // 每次 clone 增加引用计数，drop 时减少；归零时真正释放
    let a = Rc::new(String::from("shared data"));
    println!("初始 strong_count = {}", Rc::strong_count(&a));

    let b = Rc::clone(&a);
    println!("clone 后 strong_count = {}", Rc::strong_count(&a));

    {
        let c = Rc::clone(&a);
        println!("块内 strong_count = {}", Rc::strong_count(&a));
        println!("  b 和 c 指向同一数据: b='{}', c='{}'", b, c);
    } // c 被 drop
    println!("块结束后 strong_count = {}", Rc::strong_count(&a));

    // 共享数据的典型场景：图节点
    println!("a='{}', b='{}'（共享同一 String）", a, b);

    println!("\n===== 3. Arc<T> 原子引用计数（多线程） =====");
    // Arc 是线程安全的 Rc，原子操作保证计数正确
    // 注意：Rc 不能跨线程，Arc 才能 Send/Sync
    let arc = Arc::new(vec![1, 2, 3]);
    let arc_clone = Arc::clone(&arc);
    let handle = std::thread::spawn(move || {
        println!("  线程内 Arc: {:?}", arc_clone);
    });
    handle.join().unwrap();
    println!("主线程 Arc: {:?}", arc);

    println!("\n===== 4. RefCell<T> 内部可变性 =====");
    // RefCell 在运行时检查借用规则：可变借用期间不能有其他借用
    // 适用：不可变上下文中需要修改内部数据
    let cell = RefCell::new(vec![1, 2, 3]);

    // borrow()：不可变借用
    {
        let r = cell.borrow();
        println!("  borrow: {:?}", *r);
    } // 必须先释放借用才能再 borrow_mut

    // borrow_mut()：可变借用
    {
        let mut r = cell.borrow_mut();
        r.push(4);
        r.push(5);
        println!("  borrow_mut 后: {:?}", *r);
    }

    // 运行时借用检查：同时多个可变借用会 panic
    // let r1 = cell.borrow_mut();
    // let r2 = cell.borrow_mut(); // panic!

    println!("\n===== 5. Rc<RefCell<T>> 多所有权 + 可变 =====");
    // 经典组合：多个所有者 + 可变内部
    // Rc 提供共享，RefCell 提供可变性
    let shared = Rc::new(RefCell::new(vec![1, 2, 3]));

    let owner1 = Rc::clone(&shared);
    let owner2 = Rc::clone(&shared);

    // owner1 修改
    owner1.borrow_mut().push(4);
    // owner2 修改
    owner2.borrow_mut().push(5);

    // 通过任一 owner 都能看到修改
    println!("owner1 看到的: {:?}", *owner1.borrow());
    println!("owner2 看到的: {:?}", *owner2.borrow());
    println!("strong_count = {}", Rc::strong_count(&shared));

    println!("\n===== 6. Deref 强制转换 =====");
    let x = 5;
    let mybox = MyBox::new(x);
    // MyBox 实现了 Deref<Target=i32>，所以可当 &i32 用
    println!("MyBox 解引用: {} (等价于 *mybox = {})", *mybox, *mybox);

    // 标准库的 deref coercion：&String -> &str
    let s = String::from("hello");
    let s_ref: &str = &s; // String: Deref<Target=str>
    println!("String deref 到 &str: {}", s_ref);

    println!("\n===== 7. Drop 析构顺序 =====");
    // 变量离开作用域时按声明逆序 drop
    {
        let _a = Droppy { name: "A" };
        let _b = Droppy { name: "B" };
        let _c = Droppy { name: "C" };
        println!("  块结束，开始析构（逆序 C B A）：");
    } // 析构顺序: C, B, A

    // Box 的 drop 也触发内部数据的 Drop
    let boxed = Box::new(Droppy { name: "boxed" });
    println!("  Box<Droppy> 块结束：");
    drop(boxed); // 显式 drop

    println!("\n===== 8. 综合对比表 =====");
    println!("  Box<T>     : 独占堆数据，单所有者");
    println!("  Rc<T>      : 共享只读，单线程，多所有者");
    println!("  Arc<T>     : 共享只读，多线程，多所有者");
    println!("  RefCell<T> : 单线程内部可变，运行时借用检查");
    println!("  Arc<Mutex<T>> : 多线程内部可变");
    println!("  Rc<RefCell<T>>: 单线程多所有者 + 可变内部");
}

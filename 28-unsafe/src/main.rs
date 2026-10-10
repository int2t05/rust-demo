// ============================================================
// 28-unsafe: unsafe 五大操作
// ============================================================
// 学习点：
// 1. 解引用裸指针 *const T / *mut T
// 2. 调用 unsafe 函数
// 3. 访问或修改可变 static 变量
// 4. 实现 unsafe trait
// 5. 访问 union 字段
// 6. 何时该用 unsafe（FFI、性能、底层）
// 7. unsafe 不关闭借用检查
//
// 对应教程：docs/notes/13-高级特性/01-Unsafe-Rust.md
// 运行方式：cargo run -p unsafe_demo
// ============================================================

// ===== 3. 可变 static 变量 =====
// static mut 的读写是 unsafe 的，因为多线程下有数据竞争
static mut COUNTER: u32 = 0;

// 普通的不可变 static 是安全的
static HELLO: &str = "hello from static";

// ===== 4. unsafe trait =====
// 某些 trait 的实现可能违反 Rust 不变量，需标记 unsafe
// 例如：实现 Send/Sync 给含裸指针的类型
unsafe trait Trusted {}
struct MyRaw { ptr: *const u8 }
// 实现 unsafe trait 也需要 unsafe 块
unsafe impl Trusted for MyRaw {}

// ===== 5. union =====
// union 的字段共享内存，读取时编译器无法保证类型正确
// 仅Copy类型才能安全放进 union 用于演示
union IntOrFloat {
    i: i32,
    f: f32,
}

// ===== 2. unsafe 函数 =====
// unsafe fn 的调用必须在 unsafe 块内
unsafe fn dangerous() -> i32 {
    42
}

fn main() {
    println!("===== 1. 解引用裸指针 =====");
    // 裸指针 *const T / *mut T 不受借用检查约束
    // 创建裸指针本身是安全的，解引用才需要 unsafe
    let mut num = 5;
    let r1 = &num as *const i32;   // 不可变裸指针
    let r2 = &mut num as *mut i32; // 可变裸指针

    // 解引用必须在 unsafe 块内
    unsafe {
        println!("  r1 指向: {}", *r1);
        println!("  r2 指向: {}", *r2);
    }

    println!("\n===== 2. 调用 unsafe 函数 =====");
    let x = unsafe { dangerous() };
    println!("  dangerous() = {}", x);

    println!("\n===== 3. 访问 / 修改可变 static =====");
    // 读和写 static mut 都需要 unsafe
    unsafe {
        COUNTER += 1;
        COUNTER += 1;
        println!("  COUNTER = {}", COUNTER);
    }
    println!("  安全的 HELLO = {}", HELLO);

    println!("\n===== 4. 实现 unsafe trait =====");
    let raw = MyRaw { ptr: &42u8 as *const u8 };
    // 使用 Trusted 需要在 unsafe 中（如果其方法依赖不变量）
    // 这里仅演示实现了 unsafe trait
    fn use_trusted<T: Trusted>(_: &T) {}
    unsafe { use_trusted(&raw); }
    println!("  unsafe trait 实现成功");

    println!("\n===== 5. 访问 union 字段 =====");
    let u = IntOrFloat { i: 42 };
    unsafe {
        // 读 union 字段是不安全的，可能得到无效值
        println!("  作为 i32 读: {}", u.i);
        println!("  作为 f32 读: {}", u.f);
    }
    let u2 = IntOrFloat { f: 3.14 };
    unsafe {
        println!("  u2 作为 f32 读: {}", u2.f);
    }

    println!("\n===== 6. 何时该用 unsafe =====");
    // 场景：
    // (a) FFI：调用 C 库函数
    // (b) 性能：绕过运行时检查（如 Vec 的某些初始化）
    // (c) 底层：实现 Vec、HashMap、自引用结构
    // (d) 硬件：嵌入式直接读写内存映射 IO
    // 原则：能不用就不用；用了要把 unsafe 范围缩到最小
    //       并对外提供安全的 API

    println!("  - FFI：调用 C 函数");
    println!("  - 性能：绕过冗余检查");
    println!("  - 底层：容器/运行时实现");
    println!("  - 硬件：嵌入式内存映射");

    println!("\n===== 7. unsafe 不关闭借用检查 =====");
    // 重要：unsafe 块只豁免上述五类操作
    // 它不会关闭借用检查器，普通借用规则依然生效
    let mut v = vec![1, 2, 3];
    let r = &v[0];
    // unsafe {
    //     v.push(4); // 仍会编译错误：v 被借用
    // }
    println!("  unsafe 不会让借用规则失效: r = {}", r);
    drop(r);

    println!("\n===== 8. 用 unsafe 实现安全 API 的范例 =====");
    // 标准库的 Vec 内部用 unsafe 操作裸内存，但对外是安全的
    // 这里演示：用裸指针分割切片，返回两个不重叠的引用
    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();
        assert!(mid <= len);
        let ptr = values.as_mut_ptr();
        // 这里的 unsafe 让编译器相信两个切片不重叠
        unsafe {
            (
                std::slice::from_raw_parts_mut(ptr, mid),
                std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }

    let mut data = vec![10, 20, 30, 40, 50];
    let (left, right) = split_at_mut(&mut data, 2);
    println!("  split_at_mut: left={:?}, right={:?}", left, right);

    println!("\n===== 9. extern FFI 演示（不实际调用） =====");
    // extern 块声明外部函数，调用需 unsafe
    extern "C" {
        // 标准库已链接，可调用 abs
        fn abs(x: i32) -> i32;
    }
    let n = unsafe { abs(-5) };
    println!("  extern C abs(-5) = {}", n);

    println!("\n===== 10. 总结 =====");
    println!("  unsafe 的五大操作：");
    println!("    1. 解引用裸指针");
    println!("    2. 调用 unsafe 函数/方法");
    println!("    3. 访问/修改可变 static");
    println!("    4. 实现 unsafe trait");
    println!("    5. 访问 union 字段");
    println!("  原则：最小化 unsafe 范围，对外封装安全 API");
}

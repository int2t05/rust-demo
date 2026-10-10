// ============================================================
// 18-modules: 模块系统
// ============================================================
// 学习点：
// 1. mod 声明模块
// 2. pub 可见性
// 3. use 引入路径
// 4. as 重命名
// 5. 嵌套模块
// 6. 文件作为模块（front_of_house.rs）
// 7. pub(crate) / pub(super)
// 8. 在 main.rs 里用 mod 引入子模块
//
// 对应教程：docs/notes/05-模块与组织/01-Package-Crate-Module.md
// 运行方式：cargo run -p modules_demo
// ============================================================

// 声明子模块：Rust 会找 src/front_of_house.rs
mod front_of_house;

// 内联模块演示嵌套与可见性
mod kitchen {
    // 私有函数：模块外不可见
    fn wash_dishes() {
        println!("  厨房：洗碗完毕");
    }

    // pub 函数：对外公开
    pub fn cook_meal(name: &str) {
        println!("  厨房：正在烹饪 {}", name);
        wash_dishes(); // 模块内可调用私有函数
    }

    // 调用 storage 的 check_inventory（pub(super) 只对 kitchen 可见）
    pub fn check_storage() {
        storage::check_inventory();
    }

    // 嵌套模块
    pub mod storage {
        // pub(super)：对父模块 kitchen 可见
        pub(super) fn check_inventory() {
            println!("  仓库：库存检查完毕");
        }

        // pub(crate)：对整个 crate 可见
        pub(crate) fn restock() {
            println!("  仓库：已补货");
        }
    }
}

// use 引入路径，as 重命名
use kitchen::cook_meal as prepare;
use front_of_house::hosting;
use front_of_house::serving;

fn main() {
    println!("=== 模块系统演示 ===");

    // 调用内联模块的公开函数
    println!("[内联模块]");
    kitchen::cook_meal("红烧肉");
    prepare("清蒸鱼"); // as 重命名后的调用

    // 调用嵌套模块
    // check_inventory 是 pub(super) 只对 kitchen 可见，main 不能直接调
    // 通过 kitchen 的公开函数 check_storage 间接调用
    println!("[嵌套模块]");
    kitchen::check_storage();
    kitchen::storage::restock();

    // 调用文件模块
    println!("[文件模块 front_of_house]");
    hosting::add_to_waitlist();
    hosting::seat_at_table(3);

    // 通过 use 简化路径
    println!("[use 引入]");
    hosting::greet("Alice");

    // 调用 serving 子模块
    println!("[serving 子模块]");
    serving::serve_order(1);
    serving::take_payment(1);

    // 演示完整路径调用（不使用 use）
    println!("[完整路径]");
    front_of_house::hosting::greet("Bob");

    // 演示 pub(crate) 可见性
    println!("[pub(crate)]");
    kitchen::storage::restock();
}

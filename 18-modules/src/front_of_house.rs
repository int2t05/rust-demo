// 文件作为模块：src/front_of_house.rs
// 对应 main.rs 里的 mod front_of_house; 声明

/// hosting 子模块：接待客人的逻辑
pub mod hosting {
    /// 添加到等候名单
    pub fn add_to_waitlist() {
        println!("  前厅：客人已加入等候名单");
    }

    /// 安排入座
    pub fn seat_at_table(count: u32) {
        println!("  前厅：安排 {} 人入座", count);
    }

    /// 打招呼
    pub fn greet(name: &str) {
        println!("  前厅：欢迎 {} 光临！", name);
    }
}

/// serving 子模块：上菜逻辑
pub mod serving {
    /// 上菜
    pub fn serve_order(table: u32) {
        println!("  服务：{} 号桌上菜", table);
    }

    /// 结账
    pub fn take_payment(table: u32) {
        println!("  服务：{} 号台结账完成", table);
    }
}

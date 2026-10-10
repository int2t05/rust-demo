// ============================================================
// 30-minigrep: 综合实战 CLI 工具（入口）
// ============================================================
// 学习点：
// 1. 二进制入口调用 lib 逻辑
// 2. 错误传播与进程退出码
// 3. 命令行参数读取
// 4. 环境变量读取
//
// 对应教程：docs/notes/14-实战项目/01-Cli工具minigrep.md
// 运行方式：cargo run -p minigrep_demo
//   cargo run -p minigrep_demo -- <query> <file>
//   IGNORE_CASE=1 cargo run -p minigrep_demo -- to poem.txt
//   无参数时运行内置 demo
// ============================================================

use std::env;
use std::process;

use minigrep_lib::{run, run_demo, Config};

fn main() {
    // 收集命令行参数为 Vec<String>
    let args: Vec<String> = env::args().collect();

    // 无额外参数时运行 demo，便于在没有文件的情况下学习
    if args.len() < 2 {
        run_demo();
        return;
    }

    // 构造配置：把迭代器传给 Config::build
    // 这里用 into_iter 消耗 Vec，让 Config 拿到 String 所有权
    let config = Config::build(args.into_iter()).unwrap_or_else(|err| {
        eprintln!("参数错误: {}", err);
        eprintln!("用法: cargo run -p minigrep_demo -- <query> <file>");
        eprintln!("环境变量: IGNORE_CASE=1 开启大小写不敏感");
        process::exit(1);
    });

    // 运行并处理错误
    if let Err(e) = run(&config) {
        eprintln!("运行错误: {}", e);
        process::exit(1);
    }
}

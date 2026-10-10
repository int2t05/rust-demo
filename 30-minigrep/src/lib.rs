// ============================================================
// 30-minigrep: 综合实战 CLI 工具（库部分）
// ============================================================
// 学习点：
// 1. 命令行参数解析（std::env::args）
// 2. 文件读取（std::fs::read_to_string）
// 3. 环境变量（std::env::var）
// 4. 错误处理（Result + ?）
// 5. 模块化重构：lib.rs 承载核心逻辑
// 6. 大小写不敏感搜索（环境变量开关）
// 7. 用迭代器优化搜索
//
// 对应教程：docs/notes/14-实战项目/01-Cli工具minigrep.md
// 运行方式：cargo run -p minigrep_demo -- <query> <file>
//           或运行内置 demo（无参数时）
// ============================================================

use std::env;
use std::error::Error;
use std::fs;

/// 配置：从命令行参数构造
/// 字段都为 owned 数据，避免持有引用带来的生命周期复杂度
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    /// 从迭代器构造配置
    /// 第一个参数是程序名，跳过
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // 跳过程序名

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("未提供查询字符串"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("未提供文件路径"),
        };

        // 大小写不敏感：环境变量 IGNORE_CASE=1 开启
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}

/// 入口：按配置执行搜索并打印结果
pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

/// 大小写敏感搜索：返回所有匹配的行
/// 生命周期标注说明：返回的引用来自 contents
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

/// 大小写不敏感搜索：把 query 和 line 都转小写比较
/// 注意 to_lowercase 会分配新 String，这里只在比较时做
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query_lower = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query_lower))
        .collect()
}

/// 内置 demo：在无命令行参数时运行，避免必须提供文件
pub fn run_demo() {
    println!("===== minigrep demo（无参数，使用内置样例） =====");

    let sample = "\
To be, or not to be, that is the question:
Whether 'tis nobler in the mind to suffer
The slings and arrows of outrageous fortune,
Or to take arms against a sea of troubles
And by opposing end them. To die—to sleep,
No more; and by a sleep to say we end
The heart-ache and the thousand natural shocks
That flesh is heir to. 'Tis a consummation
Devoutly to be wish'd. To die, to sleep.";

    println!("\n[大小写敏感] 搜索 'to':");
    for line in search("to", sample) {
        println!("  > {}", line);
    }

    println!("\n[大小写不敏感] 搜索 'TO':");
    for line in search_case_insensitive("TO", sample) {
        println!("  > {}", line);
    }

    println!("\n[大小写敏感] 搜索 'the':");
    for line in search("the", sample) {
        println!("  > {}", line);
    }

    println!("\n[搜索不存在的词] 'xyz':");
    let r = search("xyz", sample);
    if r.is_empty() {
        println!("  （无匹配）");
    }

    // 演示 Config::build
    println!("\n[Config::build 演示]");
    let fake_args = vec![
        "minigrep".to_string(),
        "rust".to_string(),
        "input.txt".to_string(),
    ];
    match Config::build(fake_args.into_iter()) {
        Ok(cfg) => println!("  query='{}', file_path='{}', ignore_case={}", cfg.query, cfg.file_path, cfg.ignore_case),
        Err(e) => println!("  错误: {}", e),
    }

    // 缺参数的情况
    let bad_args = vec!["minigrep".to_string()];
    match Config::build(bad_args.into_iter()) {
        Ok(_) => println!("  不应该到这里"),
        Err(e) => println!("  预期错误: {}", e),
    }
}

// ===== 单元测试 =====
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn no_match() {
        let query = "xyz123";
        let contents = "hello\nworld";
        assert!(search(query, contents).is_empty());
    }

    #[test]
    fn config_build_ok() {
        let args = vec!["prog".to_string(), "q".to_string(), "f.txt".to_string()];
        let cfg = Config::build(args.into_iter()).unwrap();
        assert_eq!(cfg.query, "q");
        assert_eq!(cfg.file_path, "f.txt");
    }

    #[test]
    fn config_build_missing_query() {
        let args = vec!["prog".to_string()];
        let r = Config::build(args.into_iter());
        assert!(r.is_err());
    }
}

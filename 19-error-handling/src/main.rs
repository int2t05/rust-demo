// ============================================================
// 19-error-handling: panic! / Result / ?
// ============================================================
// 学习点：
// 1. panic! 不可恢复错误
// 2. Result<T, E> / Ok / Err
// 3. match 处理 Result
// 4. unwrap() / expect()（生产慎用）
// 5. ? 传播错误
// 6. From trait 自动转换错误
// 7. Box<dyn Error> 通用错误类型
// 8. 自定义错误类型
//
// 对应教程：docs/notes/06-错误处理/02-Result可恢复.md
// 运行方式：cargo run -p error_handling_demo
// ============================================================

use std::error::Error;
use std::fmt;
use std::fs;
use std::num::ParseIntError;

// ---- 8. 自定义错误类型 ----
#[derive(Debug)]
struct AppError {
    code: i32,
    message: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[错误{}] {}", self.code, self.message)
    }
}

impl Error for AppError {}

// From<ParseIntError> 让 ? 自动转换 ParseIntError -> AppError
impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError { code: 1001, message: format!("解析失败: {}", e) }
    }
}

/// 解析字符串为整数，失败时返回自定义错误
fn parse_number(input: &str) -> Result<i32, AppError> {
    // ? 会自动调用 From 转换错误类型
    let n: i32 = input.parse()?;
    Ok(n * 2)
}

/// 用 ? 传播错误，返回 Box<dyn Error> 通用类型
fn read_and_parse(path: &str) -> Result<i32, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;       // IO 错误自动传播
    let num: i32 = content.trim().parse()?;        // 解析错误自动传播
    Ok(num)
}

fn main() {
    println!("=== 错误处理演示 ===");

    // ---- 1. panic! ----
    // 不可恢复错误：程序会终止，打印回溯
    // panic!("致命错误：无法继续"); // 取消注释会导致崩溃

    // ---- 2 & 3. Result 与 match ----
    let result: Result<i32, &str> = Ok(42);
    match result {
        Ok(v) => println!("match Ok: {}", v),
        Err(e) => println!("match Err: {}", e),
    }

    let err_result: Result<i32, &str> = Err("出问题了");
    match err_result {
        Ok(v) => println!("match Ok: {}", v),
        Err(e) => println!("match Err: {}", e),
    }

    // ---- 4. unwrap / expect ----
    // unwrap：成功返回值，失败 panic（生产环境慎用）
    // Ok(100) 需要类型标注：编译器无法推断 E 的类型
    let ok_val: i32 = Ok::<i32, &str>(100).unwrap();
    println!("unwrap Ok: {}", ok_val);
    // let bad = Err("oops").unwrap(); // panic!

    // expect：同 unwrap 但可自定义 panic 信息
    let ok_val2: i32 = Ok::<i32, &str>(200).expect("应该成功");
    println!("expect Ok: {}", ok_val2);

    // ---- 5. ? 传播错误 ----
    match parse_number("21") {
        Ok(n) => println!("parse_number('21') = {}", n),
        Err(e) => println!("parse_number 失败: {}", e),
    }
    match parse_number("abc") {
        Ok(n) => println!("parse_number('abc') = {}", n),
        Err(e) => println!("parse_number 失败: {}", e),
    }

    // ---- 6 & 7. ? + From + Box<dyn Error> ----
    // read_and_parse 内部用 ? 传播多种错误，统一为 Box<dyn Error>
    match read_and_parse("不存在的文件.txt") {
        Ok(n) => println!("文件读到: {}", n),
        Err(e) => println!("文件操作失败: {}", e),
    }

    // ---- 8. 自定义错误的 Display ----
    let custom_err = AppError { code: 500, message: String::from("内部错误") };
    println!("自定义错误: {}", custom_err);

    // ---- 额外：Option 的 unwrap_or / unwrap_or_else ----
    let maybe: Option<i32> = None;
    let val = maybe.unwrap_or(0);
    println!("unwrap_or 默认值: {}", val);
}

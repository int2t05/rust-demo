// ============================================================
// 23-testing: 单元测试 / 集成测试 / doc test
// ============================================================
// 学习点：
// 1. 单元测试：#[cfg(test)] mod tests { #[test] fn ... }
// 2. assert! / assert_eq! / assert_ne!
// 3. #[should_panic]
// 4. #[ignore]
// 5. 集成测试：tests/ 目录
// 6. 文档测试：/// 注释里的代码块
// 7. 测试组织原则
//
// 对应教程：D:\Obsidians\Myknows\原理\Rust\19-测试\01-单元测试.md
// 运行方式：cargo test -p testing_demo
// ============================================================

/// 将字符串解析为正整数
///
/// # 示例
/// ```
/// use testing_demo::parse_positive;
/// assert_eq!(parse_positive("42"), Ok(42));
/// assert!(parse_positive("abc").is_err());
/// ```
pub fn parse_positive(s: &str) -> Result<i32, String> {
    if s.is_empty() {
        return Err(String::from("输入为空"));
    }
    s.parse::<i32>()
        .map_err(|e| format!("解析失败: {}", e))
        .and_then(|n| {
            if n > 0 {
                Ok(n)
            } else {
                Err(format!("期望正数，得到 {}", n))
            }
        })
}

/// 计算两个数的最大公约数
///
/// # 示例
///
/// ```
/// use testing_demo::gcd;
/// assert_eq!(gcd(54, 24), 6);
/// ```
pub fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

/// 判断是否为回文字符串（忽略大小写）
///
/// # 示例
///
/// ```
/// use testing_demo::is_palindrome;
/// assert!(is_palindrome("Radar"));
/// assert!(!is_palindrome("hello"));
/// ```
pub fn is_palindrome(s: &str) -> bool {
    let cleaned: Vec<char> = s.chars().map(|c| c.to_ascii_lowercase()).collect();
    let len = cleaned.len();
    for i in 0..len / 2 {
        if cleaned[i] != cleaned[len - 1 - i] {
            return false;
        }
    }
    true
}

/// 计算字符串的单词数
pub fn word_count(s: &str) -> usize {
    s.split_whitespace().count()
}

// ---- 单元测试模块 ----
// #[cfg(test)] 表示这段代码只在 cargo test 时编译
#[cfg(test)]
mod tests {
    // use super::* 引入父模块的所有公开项
    use super::*;

    #[test]
    fn test_parse_positive_ok() {
        assert_eq!(parse_positive("42"), Ok(42));
    }

    #[test]
    fn test_parse_positive_empty() {
        // assert! 验证布尔条件
        assert!(parse_positive("").is_err());
    }

    #[test]
    fn test_parse_positive_negative() {
        // assert_ne! 验证不相等
        assert_ne!(parse_positive("-5"), Ok(-5));
    }

    #[test]
    #[should_panic(expected = "除零")]
    fn test_should_panic() {
        // #[should_panic] 验证函数确实会 panic
        // expected 参数匹配 panic 信息片段
        panic!("触发除零错误");
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(54, 24), 6);
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(17, 13), 1); // 互质
        assert_eq!(gcd(0, 5), 5);
    }

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome("Radar"));
        assert!(is_palindrome("level"));
        assert!(!is_palindrome("hello"));
        assert!(is_palindrome("")); // 空字符串算回文
    }

    #[test]
    fn test_word_count() {
        assert_eq!(word_count("hello world"), 2);
        assert_eq!(word_count(""), 0);
        assert_eq!(word_count("one"), 1);
        assert_eq!(word_count("  multiple   spaces  "), 2);
    }

    #[test]
    #[ignore]
    fn test_slow_example() {
        // #[ignore] 标记的测试默认跳过
        // 用 cargo test -- --ignored 单独运行
        // 适用于耗时长的测试
        let result: u64 = (1..=1000).map(|x| x * x).sum();
        assert_eq!(result, 333833500);
    }
}

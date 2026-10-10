// ============================================================
// 23-testing: 集成测试
// ============================================================
// 集成测试放在 tests/ 目录下，作为独立 crate 运行
// 只能测试库的公开 API（pub 函数）
// 通过 use testing_demo::... 引入库
//
// 对应教程：docs/notes/08-测试/03-测试组织与文档测试.md
// 运行方式：cargo test -p testing_demo
// ============================================================

use testing_demo::{gcd, is_palindrome, parse_positive, word_count};

#[test]
fn integration_parse_positive() {
    assert_eq!(parse_positive("100"), Ok(100));
    assert!(parse_positive("").is_err());
    assert!(parse_positive("-1").is_err());
}

#[test]
fn integration_gcd() {
    assert_eq!(gcd(48, 36), 12);
    assert_eq!(gcd(7, 3), 1);
}

#[test]
fn integration_is_palindrome() {
    assert!(is_palindrome("noon"));
    assert!(is_palindrome("WasItACarOrACatISaw"));
    assert!(!is_palindrome("rust"));
}

#[test]
fn integration_word_count() {
    assert_eq!(word_count("the quick brown fox"), 4);
    assert_eq!(word_count(""), 0);
}

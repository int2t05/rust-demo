// 集成测试：测试 minigrep_lib 的公开 API
// 运行：cargo test -p minigrep_demo

use minigrep_lib::{search, search_case_insensitive};

#[test]
fn search_finds_matches() {
    let contents = "hello world\nhello rust\ngoodbye";
    let result = search("hello", contents);
    assert_eq!(result, vec!["hello world", "hello rust"]);
}

#[test]
fn search_case_insensitive_works() {
    let contents = "Hello\nHELLO\nhello\nworld";
    let result = search_case_insensitive("hello", contents);
    assert_eq!(result.len(), 3);
}

#[test]
fn search_empty_contents() {
    assert!(search("x", "").is_empty());
}

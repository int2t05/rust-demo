// ============================================================
// 17-hashmap: HashMap<K,V>
// ============================================================
// 学习点：
// 1. HashMap::new() 创建
// 2. insert / get
// 3. for (k, v) in &map 遍历
// 4. entry().or_insert() 检查存在
// 5. 更新值：覆盖、entry API、基于旧值
// 6. 所有权：insert 会 move 走 String
//
// 对应教程：docs/notes/04-数据结构/06-HashMap映射.md
// 运行方式：cargo run -p hashmap_demo
// ============================================================

use std::collections::HashMap;

fn main() {
    // ---- 1. 创建 HashMap ----
    let mut scores: HashMap<String, i32> = HashMap::new();

    // ---- 2. insert / get ----
    scores.insert(String::from("蓝队"), 10);
    scores.insert(String::from("红队"), 25);

    // get 返回 Option
    let red = scores.get("红队");
    match red {
        Some(score) => println!("红队得分: {}", score),
        None => println!("找不到红队"),
    }

    // ---- 3. 遍历 ----
    println!("所有队伍:");
    for (team, score) in &scores {
        println!("  {} : {}", team, score);
    }

    // ---- 4. entry().or_insert() ----
    // 如果键不存在则插入默认值，存在则跳过
    scores.entry(String::from("黄队")).or_insert(5);
    scores.entry(String::from("红队")).or_insert(0); // 红队已存在，不会覆盖
    println!("entry 后: {:?}", scores);

    // ---- 5. 更新值的三种方式 ----
    // 方式 A：直接覆盖
    let mut map = HashMap::new();
    map.insert("计数", 1);
    map.insert("计数", 2);               // 覆盖旧值
    println!("覆盖后: {:?}", map);

    // 方式 B：entry API 只在键不存在时插入
    let mut map2 = HashMap::new();
    map2.entry("key").or_insert("first");
    map2.entry("key").or_insert("second"); // 不会覆盖
    println!("entry 不覆盖: {:?}", map2);

    // 方式 C：基于旧值更新
    let text = "hello world wonderful world";
    let mut word_count: HashMap<&str, i32> = HashMap::new();
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0); // 返回可变引用
        *count += 1;                                       // 就地修改
    }
    println!("词频统计: {:?}", word_count);

    // ---- 6. 所有权 ----
    let key = String::from("color");
    let value = String::from("red");
    let mut own_map = HashMap::new();
    own_map.insert(key, value);
    // println!("{}", key); // 错误！key 已被 move 进 map
    // 从 map 取回只能 clone 或借用
    let borrowed = own_map.get("color");
    println!("从 map 借用: {:?}", borrowed);

    // ---- 额外：从 Vec 构造 HashMap ----
    let teams = vec![String::from("A"), String::from("B")];
    let initial_scores = vec![10, 20];
    let team_map: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("从 zip 构造: {:?}", team_map);
}

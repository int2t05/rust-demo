// ============================================================
// 25-iterators: 迭代器与消费器
// ============================================================
// 学习点：
// 1. Iterator trait 与 next 方法
// 2. iter() / into_iter() / iter_mut() 三种迭代方式
// 3. 适配器（惰性）：map / filter / take / skip / enumerate / zip / flat_map
// 4. 消费器：sum / count / collect / any / all / find / max / min
// 5. 链式调用
// 6. 自定义迭代器（实现 Iterator trait）
// 7. 性能：零成本抽象
//
// 对应教程：D:\Obsidians\Myknows\原理\Rust\09-函数式特性\02-迭代器.md
// 运行方式：cargo run -p iterators_demo
// ============================================================

/// 自定义迭代器：生成从 start 开始的斐波那契数列
struct Fibonacci {
    prev: u64,
    curr: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { prev: 0, curr: 1 }
    }
}

/// 实现 Iterator trait 只需定义 next 方法
/// 迭代器是惰性的：不调用消费器就不会真正执行
impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let next_val = self.prev + self.curr;
        self.prev = self.curr;
        self.curr = next_val;
        Some(self.prev) // 返回当前项
    }
}

/// 另一个自定义迭代器：区间步进
struct RangeStep {
    current: i32,
    end: i32,
    step: i32,
}

impl Iterator for RangeStep {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.end {
            None
        } else {
            let val = self.current;
            self.current += self.step;
            Some(val)
        }
    }
}

fn main() {
    println!("===== 1. 三种迭代方式 =====");
    let v = vec![10, 20, 30];

    // iter()：借用元素，原 vec 仍可用
    for x in v.iter() {
        print!("{} ", x);
    }
    println!("（iter 后 vec 仍可用: {:?})", v);

    // into_iter()：消耗 vec，取得所有权
    let v2 = vec![10, 20, 30];
    for x in v2.into_iter() {
        print!("{} ", x);
    }
    println!();
    // println!("{:?}", v2); // 编译错误：v2 已被消耗

    // iter_mut()：可变借用，可修改元素
    let mut v3 = vec![10, 20, 30];
    for x in v3.iter_mut() {
        *x *= 2;
    }
    println!("iter_mut 后: {:?}", v3);

    println!("\n===== 2. 适配器（惰性，不消费不执行） =====");
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // map：映射每个元素
    let mapped: Vec<i32> = v.iter().map(|x| x * x).collect();
    println!("map 平方: {:?}", mapped);

    // filter：过滤满足条件的元素
    let evens: Vec<&i32> = v.iter().filter(|x| *x % 2 == 0).collect();
    println!("filter 偶数: {:?}", evens);

    // take：只取前 N 个
    let first3: Vec<&i32> = v.iter().take(3).collect();
    println!("take 3: {:?}", first3);

    // skip：跳过前 N 个
    let after_skip: Vec<&i32> = v.iter().skip(7).collect();
    println!("skip 7: {:?}", after_skip);

    // enumerate：带索引
    for (i, x) in v.iter().take(3).enumerate() {
        println!("  enumerate[{}] = {}", i, x);
    }

    // zip：把两个迭代器配对
    let names = vec!["alice", "bob", "carol"];
    let ages = vec![30, 25, 40];
    let pairs: Vec<(&str, i32)> = names.iter().zip(ages.iter()).map(|(n, a)| (*n, *a)).collect();
    println!("zip: {:?}", pairs);

    // flat_map：map 后再展平
    let nested = vec![vec![1, 2], vec![3, 4], vec![5]];
    let flat: Vec<i32> = nested.into_iter().flat_map(|sub| sub.into_iter()).collect();
    println!("flat_map: {:?}", flat);

    println!("\n===== 3. 消费器 =====");
    let v = vec![1, 2, 3, 4, 5];

    // sum：求和
    println!("sum = {}", v.iter().sum::<i32>());

    // count：计数
    println!("count = {}", v.iter().count());

    // collect：收集成集合
    let collected: Vec<i32> = v.iter().copied().collect();
    println!("collect = {:?}", collected);

    // any / all：判断是否存在 / 全部满足
    println!("any > 3: {}", v.iter().any(|x| *x > 3));
    println!("all > 0: {}", v.iter().all(|x| *x > 0));

    // find：找到第一个满足条件的
    if let Some(first_even) = v.iter().find(|x| *x % 2 == 0) {
        println!("find first even = {}", first_even);
    }

    // max / min：最大最小值
    println!("max = {:?}", v.iter().max());
    println!("min = {:?}", v.iter().min());

    println!("\n===== 4. 链式调用 =====");
    // 经典链式：1..=10 中偶数的平方和
    let result: i32 = (1..=10)
        .filter(|x| x % 2 == 0)   // 2,4,6,8,10
        .map(|x| x * x)           // 4,16,36,64,100
        .sum();                    // 220
    println!("1..=10 偶数平方和 = {}", result);

    // 复杂链式：单词长度统计
    let sentence = "the quick brown fox jumps";
    let word_count: usize = sentence.split_whitespace().count();
    let total_chars: usize = sentence.split_whitespace().map(|w| w.len()).sum();
    println!("词数 = {}, 总字符 = {}", word_count, total_chars);

    println!("\n===== 5. 自定义迭代器 =====");
    // 斐波那契前 10 项
    let fibs: Vec<u64> = Fibonacci::new().take(10).collect();
    println!("斐波那契前 10 项: {:?}", fibs);

    // 利用自定义迭代器做链式
    let fib_evens: Vec<u64> = Fibonacci::new()
        .take(15)
        .filter(|x| x % 2 == 0)
        .collect();
    println!("前 15 项中的偶数: {:?}", fib_evens);

    // RangeStep
    let steps: Vec<i32> = RangeStep { current: 0, end: 20, step: 3 }.collect();
    println!("步进 0..20 step 3: {:?}", steps);

    println!("\n===== 6. 零成本抽象说明 =====");
    // 迭代器是零成本抽象：编译期展开为等价的循环代码，无运行时开销
    // 下面的链式与手写 for 循环性能几乎一致
    let v: Vec<i32> = (1..=1000).collect();
    let sum: i32 = v.iter().filter(|x| *x % 3 == 0).map(|x| x / 3).sum();
    println!("1..=1000 中 3 的倍数除以 3 后求和 = {}", sum);
}

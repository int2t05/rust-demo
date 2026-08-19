// ============================================================
// 05-compound-types: 复合类型
// ============================================================
// 学习点：
// 1. 元组：多个不同类型的值组合，长度固定
// 2. 单元类型 ()
// 3. 数组：相同类型，栈上分配，长度固定
// 4. 数组越界 panic
// 5. 数组遍历与 len()
//
// 对应教程：D:\Obsidians\Myknows\原理\Rust\02-基础\03-compound-types.md
// 运行方式：cargo run -p 05-compound-types
// ============================================================

fn main() {
    // --- 1. 元组 ---
    // 元组用 () 包裹，可以包含不同类型，长度固定
    let tup: (i32, f64, char) = (500, 6.4, 'A');
    println!("元组: {:?}", tup);

    // 访问元组元素：用 .0 .1 .2 索引（从 0 开始）
    println!("元组第 0 个: {}", tup.0);
    println!("元组第 1 个: {}", tup.1);
    println!("元组第 2 个: {}", tup.2);

    // --- 2. 元组解构 ---
    // 把元组拆解成单独的变量
    let (a, b, c) = tup;
    println!("解构: a={} b={} c={}", a, b, c);

    // 部分解构：用 _ 忽略不需要的
    let (_, float_val, _) = tup;
    println!("只取中间值: {}", float_val);

    // --- 3. 单元类型 () ---
    // () 是空元组，只有一个值，也叫单元类型
    // 函数没有返回值时隐式返回 ()
    let unit: () = ();
    println!("单元类型值: {:?}", unit);

    // 单元类型常用于：
    // - 函数无返回值
    // - Vec<()> 之类的占位

    // --- 4. 数组 ---
    // 数组：相同类型，长度固定，栈上分配
    // 与 Vec 不同：数组长度不可变，Vec 可变
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("数组: {:?}", arr);

    // 访问数组元素：用 [index]
    println!("arr[0] = {}", arr[0]);
    println!("arr[4] = {}", arr[4]);

    // --- 5. 数组初始化语法 ---
    // [value; count] 创建 count 个 value 的数组
    let zeros = [0; 5];   // 5 个 0
    let ones = [1; 3];    // 3 个 1
    println!("zeros: {:?}", zeros);
    println!("ones: {:?}", ones);

    // --- 6. 数组长度 ---
    // .len() 返回数组长度（usize 类型）
    println!("arr 长度: {}", arr.len());

    // --- 7. 数组遍历 ---
    // .iter() 创建迭代器，for 循环遍历
    println!("遍历 arr:");
    for x in arr.iter() {
        print!("{} ", x);
    }
    println!();

    // 也可以直接用 for in 遍历数组引用（借用）
    println!("遍历 arr 引用:");
    for x in &arr {
        print!("{} ", x);
    }
    println!();

    // 使用索引遍历
    println!("索引遍历:");
    for i in 0..arr.len() {
        print!("arr[{}]={} ", i, arr[i]);
    }
    println!();

    // --- 8. 数组越界 ---
    // 访问越界索引会 panic（运行时错误）
    // 编译器无法检测所有越界，因为索引可能是运行时计算的
    //
    // 以下代码会 panic（注释掉）：
    // let invalid = arr[10];  // thread 'main' panicked at 'index out of bounds'

    // 安全的访问方式：get 返回 Option
    // 有值返回 Some，越界返回 None
    let safe = arr.get(10);
    println!("arr.get(10) = {:?}", safe);    // None
    let safe_ok = arr.get(2);
    println!("arr.get(2) = {:?}", safe_ok);  // Some(3)

    // --- 9. 多维数组 ---
    // 数组的元素也可以是数组
    let matrix: [[i32; 3]; 2] = [
        [1, 2, 3],
        [4, 5, 6],
    ];
    println!("矩阵: {:?}", matrix);
    println!("matrix[1][2] = {}", matrix[1][2]);  // 6
}

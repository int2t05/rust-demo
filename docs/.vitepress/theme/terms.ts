// 术语字典:全站唯一数据源
// 英文 key 用小写(匹配时含复数 s、大小写变体);中文 key 用原词
// zh: 中文对应词(英文词填);ipa: 音标(英文词填);def: 通俗解析
// 中文概念与英文 key 指向同一条目,释义只写一份
export interface TermEntry {
  zh?: string
  ipa?: string
  def: string
}

const entries: Record<string, TermEntry> = {
  // ---- 所有权系统 ----
  ownership: {
    zh: '所有权',
    ipa: '/ˈəʊnəʃɪp/',
    def: '每个值有唯一的所有者,所有者离开作用域时值自动释放——Rust 用这条规则代替垃圾回收保证内存安全。',
  },
  move: {
    zh: '移动',
    ipa: '/muːv/',
    def: '赋值或传参时所有权转移到新变量,原变量立即失效——从根上防止双重释放。',
  },
  borrow: {
    zh: '借用',
    ipa: '/ˈbɒrəʊ/',
    def: '通过引用临时使用数据而不获取所有权,用完归还,不改变数据的归属。',
  },
  'borrow checker': {
    zh: '借用检查器',
    ipa: '/ˈbɒrəʊ ˈtʃekə/',
    def: '编译期静态检查所有引用是否合法的组件,违反借用规则直接编译失败。',
  },
  reference: {
    zh: '引用',
    ipa: '/rɪˈfərəns/',
    def: '指向其他数据的访问通道:& 创建不可变引用,&mut 创建可变引用。',
  },
  'mutable reference': {
    zh: '可变引用',
    ipa: '/ˈmjuːtəbl ˈrɪfərəns/',
    def: '&mut T:允许修改被引用数据的引用,同一时刻只能存在一个。',
  },
  'dangling reference': {
    zh: '悬垂引用',
    ipa: '/ˈdæŋɡlɪŋ ˈrɪfərəns/',
    def: '指向已释放内存的引用,C/C++ 里是未定义行为的常见来源;Rust 在编译期直接拒绝。',
  },
  slice: {
    zh: '切片',
    ipa: '/slaɪs/',
    def: '对集合中一段连续元素的引用视图,不持有所有权,如 &str 和 &[T]。',
  },
  stack: {
    zh: '栈',
    ipa: '/stæk/',
    def: '后进先出的内存区,存大小固定、编译期已知的数据,分配释放极快。',
  },
  heap: {
    zh: '堆',
    ipa: '/hiːp/',
    def: '运行时动态分配的内存区,存大小可能变化的数据,如 String 的内容。',
  },
  scope: {
    zh: '作用域',
    ipa: '/skəʊp/',
    def: '变量有效的代码范围,从声明处到所在块结束,离开即释放。',
  },
  copy: {
    zh: '复制',
    ipa: '/ˈkɒpi/',
    def: 'Copy trait:实现它的类型(如 i32)赋值时按位复制,原变量继续可用。',
  },
  clone: {
    zh: '克隆',
    ipa: '/kləʊn/',
    def: 'Clone trait:显式深拷贝堆数据,成本高但意图清楚,必须写 .clone()。',
  },
  trait: {
    zh: '特征',
    ipa: '/treɪt/',
    def: 'Rust 对「接口」的叫法,定义一组类型可以具备的行为,如 Copy、Clone。',
  },
  'deref coercion': {
    zh: '解引用强制转换',
    ipa: '/ˈdiːref kəʊˈɜːʃn/',
    def: '&String 自动转成 &str 一类的自动类型转换,让函数参数更通用。',
  },
  数据竞争: {
    def: '两个指针同时写同一块内存且没加锁。借用规则在编译期就杜绝了它。',
  },
  // ---- 入门基础 ----
  macro: {
    zh: '宏',
    ipa: '/ˈmækroʊ/',
    def: '名字以 ! 结尾、在编译期展开成代码的语法,println! 就是一个宏,所以能接受任意个参数。',
  },
  crate: {
    zh: '箱(编译单元)',
    ipa: '/kreɪt/',
    def: 'Rust 的最小编译单位:binary crate(src/main.rs)或 library crate(src/lib.rs),cargo run -p 跑的就是它。',
  },
  'format specifier': {
    zh: '格式化说明符',
    ipa: '/fɔːrˈmæt ˈspesɪfaɪər/',
    def: '占位符里的控制指令,如 {} 走 Display、{:?} 走 Debug、{:>10} 控制对齐。',
  },
  stderr: {
    zh: '标准错误流',
    ipa: '/ˈstændəd ˈerə/',
    def: '与 stdout 分开的诊断输出通道,重定向正常输出时错误信息仍留在屏幕上。',
  },
  shadowing: {
    zh: '变量遮蔽',
    ipa: '/ˈʃædəʊɪŋ/',
    def: '用同名 let 重新声明,创建的新变量盖住旧变量,类型也可以不同。',
  },
  result: {
    zh: 'Result',
    ipa: '/rɪˈzʌlt/',
    def: '表示可能失败操作的枚举:Ok 装成功值,Err 装错误,必须处理两个分支才能取值。',
  },
  parse: {
    zh: '解析',
    ipa: '/pɑːrs/',
    def: '把字符串按目标类型转换的方法,转不动时返回 Err 而不是崩溃。',
  },
  dependency: {
    zh: '依赖',
    ipa: '/dɪˈpendənsi/',
    def: '项目要用到的外部 crate,在 Cargo.toml 的 [dependencies] 段声明后由 cargo 自动拉取。',
  },
  // ---- 核心语法 ----
  'immutable variable': {
    zh: '不可变变量',
    ipa: '/ɪˈmjuːtəbl ˈveəriəbl/',
    def: '用 let 声明、不能重新赋值的绑定,Rust 的默认行为。',
  },
  'mutable variable': {
    zh: '可变变量',
    ipa: '/ˈmjuːtəbl ˈveəriəbl/',
    def: '用 let mut 声明、允许原地重新赋值的绑定,类型不能变。',
  },
  constant: {
    zh: '常量',
    ipa: '/ˈkɒnstənt/',
    def: 'const 声明的编译期常量:必须显式标注类型、只能用常量表达式初始化,内联到使用处。',
  },
  'static variable': {
    zh: '静态变量',
    ipa: '/ˈstætɪk ˈveəriəbl/',
    def: 'static 声明、有固定内存地址、生命周期为整个程序的变量。',
  },
  'type inference': {
    zh: '类型推断',
    ipa: '/taɪp ˌɪnfəˈrəns/',
    def: '编译器根据上下文自动确定类型;整数字面量默认推断为 i32。',
  },
  'scalar type': {
    zh: '标量类型',
    ipa: '/ˈskeɪlə taɪp/',
    def: '表示单个值的类型:整数、浮点、布尔、字符。',
  },
  signed: {
    zh: '有符号',
    ipa: '/saɪnd/',
    def: 'i 系列整数,可表示负数。',
  },
  unsigned: {
    zh: '无符号',
    ipa: '/ʌnˈsaɪnd/',
    def: 'u 系列整数,只能表示非负数。',
  },
  'integer overflow': {
    zh: '整数溢出',
    ipa: '/ˈɪntɪdʒə ˌəʊvəˈfləʊ/',
    def: '运算结果超出类型范围;debug 构建 panic,release 构建回绕,可用 wrapping/checked/saturating/overflowing 显式指定语义。',
  },
  'Unicode scalar value': {
    zh: 'Unicode 标量值',
    ipa: '/ˈjuːnɪkəʊd ˈskeɪlə ˈvæljuː/',
    def: 'char 的实际内容,占 4 字节,覆盖全部 Unicode 字符。',
  },
  tuple: {
    zh: '元组',
    ipa: '/ˈtʌpl/',
    def: '长度固定、各位置类型可不同的复合类型,用 .0 .1 访问。',
  },
  array: {
    zh: '数组',
    ipa: '/ˈəreɪ/',
    def: '长度固定、元素同型、分配在栈上的复合类型,类型写作 [T; N]。',
  },
  'unit type': {
    zh: '单元类型',
    ipa: '/ˈjuːnɪt taɪp/',
    def: '空元组 (),只有一个值;函数没有返回值标注时隐式返回它。',
  },
  destructuring: {
    zh: '解构',
    ipa: '/diːˈstrʌktʃərɪŋ/',
    def: '用模式把复合值拆成多个绑定,如 let (a, b) = t。',
  },
  'multidimensional array': {
    zh: '多维数组',
    ipa: '/ˌmʌltidɪˈmenʃənl ˈəreɪ/',
    def: '元素本身是数组的数组,类型写作 [[T; 列数]; 行数]。',
  },
  statement: {
    zh: '语句',
    ipa: '/ˈsteɪtmənt/',
    def: '执行动作但不产生值的代码单元,以分号结尾,如 let 绑定。',
  },
  expression: {
    zh: '表达式',
    ipa: '/ɪkˈspreʃn/',
    def: '求值后产生值的代码单元;块、if、函数调用都是表达式。',
  },
  'block expression': {
    zh: '块表达式',
    ipa: '/blɒk ɪkˈspreʃn/',
    def: '花括号块作为表达式,其值是块内最后一个无分号表达式。',
  },
  'loop expression': {
    zh: 'loop 表达式',
    ipa: '/luːp ɪkˈspreʃn/',
    def: '无限循环;break 可携带值作为整个 loop 表达式的结果。',
  },
  range: {
    zh: '范围',
    ipa: '/reɪndʒ/',
    def: '1..5 左闭右开,1..=5 双闭;for 遍历计数的常用写法。',
  },
  // ---- 数据结构 ----
  struct: {
    zh: '结构体',
    ipa: '/strʌkt/',
    def: '把多个命名字段打包成一个自定义复合类型。',
  },
  field: {
    zh: '字段',
    ipa: '/fiːld/',
    def: '结构体内部的一个具名数据项。',
  },
  method: {
    zh: '方法',
    ipa: '/ˈmeθəd/',
    def: '定义在 impl 块中、第一个参数为 self 系列的函数,用 . 调用。',
  },
  'associated function': {
    zh: '关联函数',
    ipa: '/əˈsoʊʃieɪtɪd ˈfʌŋkʃn/',
    def: 'impl 块中不带 self 参数的函数,用 :: 调用,常作构造器。',
  },
  'unit struct': {
    zh: '单元结构体',
    ipa: '/ˈjuːnɪt strʌkt/',
    def: '没有任何字段的结构体,常作类型层面标记。',
  },
  enum: {
    zh: '枚举',
    ipa: '/ˈɪnuːm/',
    def: '列出类型所有合法变体的复合类型,变体可携带数据。',
  },
  variant: {
    zh: '变体',
    ipa: '/ˈveriənt/',
    def: '枚举的一种可能取值,可无数据也可携带任意类型数据。',
  },
  Option: {
    zh: 'Option 类型',
    ipa: '/ˈɒpʃn/',
    def: '标准库枚举 Some(T)/None,用类型系统替代 null。',
  },
  'sum type': {
    zh: '和类型',
    ipa: '/sʌm taɪp/',
    def: '一个值是若干带数据变体之一的类型构造方式,Rust 枚举即和类型。',
  },
  match: {
    zh: 'match 表达式',
    ipa: '/mætʃ/',
    def: '强制穷尽所有可能的分支表达式,每个分支为 模式=>表达式。',
  },
  'pattern matching': {
    zh: '模式匹配',
    ipa: '/ˈpætn ˈmætʃɪŋ/',
    def: '按数据的形状分支并同时绑定内部值。',
  },
  exhaustiveness: {
    zh: '穷尽性',
    ipa: '/ɪɡˈzɔːstɪvnəs/',
    def: 'match 必须覆盖所有可能取值,否则编译失败。',
  },
  'if let': {
    zh: 'if let',
    ipa: '/ɪf let/',
    def: '只匹配一种模式的条件简写,不要求穷尽。',
  },
  'wildcard pattern': {
    zh: '通配符',
    ipa: '/ˈwaɪldkɑːrd ˈpætərn/',
    def: '_ 模式,匹配一切但不绑定、不移动值。',
  },
  Vec: {
    zh: '动态数组',
    ipa: '/vek/',
    def: '堆上可增长的连续同类型元素集合。',
  },
  capacity: {
    zh: '容量',
    ipa: '/kəˈpæsəti/',
    def: 'Vec 已分配内存可容纳的元素数,超出即触发扩容搬迁。',
  },
  'iterator adapter': {
    zh: '迭代器适配器',
    ipa: '/ˈaɪtəreɪtər ˈædæptər/',
    def: 'map/filter 一类返回新迭代器的方法。',
  },
  panic: {
    zh: '不可恢复错误',
    ipa: '/ˈpænɪk/',
    def: '逻辑上不该发生的 bug 触发的终止机制:打印信息、展开栈、进程退出,索引越界即触发。',
  },
  String: {
    zh: '字符串(拥有型)',
    ipa: '/strɪŋ/',
    def: '拥有堆上 UTF-8 字节序列的可增长字符串类型。',
  },
  str: {
    zh: '字符串切片',
    ipa: '/str/',
    def: 'UTF-8 文本的借用视图类型,字符串字面量即 &str。',
  },
  'UTF-8': {
    zh: 'UTF-8 编码',
    ipa: '/ˌjuːtiːˈeɪt/',
    def: '变长字符编码,ASCII 1 字节、汉字 3 字节,String 按它存储。',
  },
  'char boundary': {
    zh: '字符边界',
    ipa: '/tʃɑːr ˈbaʊndri/',
    def: '字符起始的字节位置,切片必须落在边界上否则 panic。',
  },
  HashMap: {
    zh: '哈希映射',
    ipa: '/ˈhæʃmæp/',
    def: '哈希表实现的键值对集合,平均 O(1) 查找。',
  },
  'entry API': {
    zh: 'entry API',
    ipa: '/ˈentri/',
    def: 'entry(k).or_insert(v) 惯用法:一次哈希完成存在检查、插入并返回可变引用。',
  },
  hash: {
    zh: '哈希',
    ipa: '/hæʃ/',
    def: '把键映射成整数下标的函数,键类型必须实现 Hash + Eq。',
  },
  // ---- 模块与组织 ----
  module: {
    zh: '模块',
    ipa: '/ˈmɒdʒuːl/',
    def: '用 mod 声明的代码组织单元,是 crate 内的可见性与命名空间边界。',
  },
  package: {
    zh: '包',
    ipa: '/ˈpækɪdʒ/',
    def: '由 Cargo.toml 描述的交付单元,可含一个 library crate 和多个 binary crate。',
  },
  use: {
    zh: '路径引入',
    ipa: '/juːz/',
    def: '把模块路径提为本地名字的关键字,可用 as 重命名以避冲突。',
  },
  // ---- 错误处理 ----
  'error handling': {
    zh: '错误处理',
    ipa: '/ˈerər ˈhændlɪŋ/',
    def: 'Rust 用 panic! 与 Result<T,E> 取代异常,把失败编码进返回值,编译器强制处理。',
  },
  'recoverable error': {
    zh: '可恢复错误',
    ipa: '/rɪˈkʌvərəbl ˈerər/',
    def: '调用方有能力应对的失败(文件打不开、解析失败),用 Result<T,E> 表达。',
  },
  'error propagation': {
    zh: '错误传播',
    ipa: '/ˈerər ˌprɒpəˈɡeɪʃn/',
    def: '函数不处理错误,把失败编码进返回值交由上层接力处理。',
  },
  'question mark operator': {
    zh: '? 运算符',
    ipa: '/ˈkwestʃən mɑːrk ˈɒpəreɪtər/',
    def: 'Ok 取值继续、Err 提前 return 并经 From 转换的错误传播简写,只能用于返回 Result 的函数。',
  },
  // ---- 泛型 Trait 生命周期 ----
  generics: {
    zh: '泛型',
    ipa: '/dʒəˈnɛrɪks/',
    def: '用类型参数 <T> 编写适用于多种类型的代码,编译期单态化为具体版本。',
  },
  monomorphization: {
    zh: '单态化',
    ipa: '/ˌmɒnəˌmɔːfɪˈzeɪʃən/',
    def: '编译时为每个用到的具体类型生成泛型代码的专用副本,运行时零开销。',
  },
  'zero-cost abstraction': {
    zh: '零成本抽象',
    ipa: '/ˈzɪərəʊ kɒst ˈæbstrækʃən/',
    def: '抽象不带来运行时开销,代价转移到编译期与二进制体积。',
  },
  'trait bound': {
    zh: 'trait 约束',
    ipa: '/treɪt baʊnd/',
    def: '对泛型参数的要求:T 必须实现指定 trait,如 <T: Summary>。',
  },
  'default implementation': {
    zh: '默认实现',
    ipa: '/dɪˈfɔːlt ˌɪmplɪmenˈteɪʃən/',
    def: 'trait 中带方法体的方法,实现方可选择覆盖,不覆盖则使用默认。',
  },
  'blanket implementation': {
    zh: '覆盖实现',
    ipa: '/ˈblæŋkɪt ˌɪmplɪmenˈteɪʃən/',
    def: '为所有满足某 bound 的类型统一实现一个 trait,如 impl<T: Display> Shout for T。',
  },
  'orphan rule': {
    zh: '孤儿规则',
    ipa: '/ˈɔːfən ruːl/',
    def: '只能为本 crate 的类型实现外部 trait,或为外部类型实现本 crate 的 trait。',
  },
  'trait object': {
    zh: 'trait 对象',
    ipa: '/treɪt ˈɒbdʒɪkt/',
    def: 'dyn Trait 类型的值,由数据指针与虚表构成,运行时分发到具体实现。',
  },
  'static dispatch': {
    zh: '静态分发',
    ipa: '/ˈstætɪk dɪˈspætʃ/',
    def: '编译期经单态化确定调用目标,零运行时开销。',
  },
  'dynamic dispatch': {
    zh: '动态分发',
    ipa: '/daɪˈnæmɪk dɪˈspætʃ/',
    def: '运行时经虚表查找方法地址,多一次间接调用。',
  },
  vtable: {
    zh: '虚表',
    ipa: '/ˈviːteɪbəl/',
    def: 'trait 对象背后的方法地址表,动态分发的查找依据。',
  },
  'object safety': {
    zh: '对象安全',
    ipa: '/ˈɒbdʒɪkt ˈsiːfti/',
    def: 'trait 可作 dyn 使用的条件:方法不返回 Self、不带泛型参数。',
  },
  lifetime: {
    zh: '生命周期',
    ipa: '/ˈlaɪftaɪm/',
    def: '引用保持有效的范围;标注 \'a 描述引用间的存活关系,不改变真实寿命。',
  },
  'lifetime elision': {
    zh: '生命周期省略',
    ipa: '/ˈlaɪftaɪm ɪˈlɪʒən/',
    def: '三条规则让编译器自动补全常见情形的生命周期标注。',
  },
  'static lifetime': {
    zh: '静态生命周期',
    ipa: '/ˈstætɪk ˈlaɪftaɪm/',
    def: "'static,引用在整个程序运行期有效;字符串字面量默认如此。",
  },
  // ---- 测试 ----
  'unit test': {
    zh: '单元测试',
    ipa: '/ˈjuːnɪt test/',
    def: '与被测代码同文件、写在 #[cfg(test)] mod tests 里的测试,可访问私有函数。',
  },
  'integration test': {
    zh: '集成测试',
    ipa: '/ˌɪntɪˈɡreɪʃən test/',
    def: 'tests/ 目录下的独立测试,只能调用库的公开 API,从外部视角验证行为。',
  },
  'doc test': {
    zh: '文档测试',
    ipa: '/dɒk test/',
    def: '/// 文档注释里的代码块,cargo test 会编译运行它,保证文档示例不过时。',
  },
  attribute: {
    zh: '属性',
    ipa: '/ˈætrɪbjuːt/',
    def: '标注在项上的元数据标记(如 #[test]、#[ignore]),向编译器附加指令。',
  },
  // ---- 函数式特性 ----
  closure: {
    zh: '闭包',
    ipa: '/ˈkloʊʒər/',
    def: '匿名函数,可捕获定义处作用域中的变量;编译为匿名结构体并实现 Fn 系列 trait。',
  },
  capture: {
    zh: '捕获',
    ipa: '/ˈkæptʃər/',
    def: '闭包引用外部作用域变量的行为,分不可变借用、可变借用、move 转移所有权三种。',
  },
  'lazy evaluation': {
    zh: '惰性求值',
    ipa: '/ˈleɪzi ɪˌvæljuˈeɪʃən/',
    def: '迭代器适配器不立即处理元素,直到消费器调用 next 才计算。',
  },
  consumer: {
    zh: '消费器',
    ipa: '/kənˈsuːmər/',
    def: '消耗迭代器并产出最终结果的方法,如 sum、collect、find。',
  },
  'Fn traits': {
    zh: 'Fn 系列 trait',
    ipa: '/ˌɛfˈɛn treɪts/',
    def: 'Fn/FnMut/FnOnce 三个 trait,描述闭包对捕获值是只读、可变使用还是消耗;层级 Fn ⊂ FnMut ⊂ FnOnce。',
  },
  // ---- 智能指针 ----
  Box: {
    zh: '独占堆指针',
    ipa: '/bɒks/',
    def: '把值放到堆上、栈上只留一个固定大小指针的智能指针,独占所有权。',
  },
  Rc: {
    zh: '引用计数指针',
    ipa: '/ˌɑːrˈsiː/',
    def: '单线程引用计数智能指针,允许多个所有者共享只读数据。',
  },
  Arc: {
    zh: '原子引用计数指针',
    ipa: '/ˌɑːrˈsiː/',
    def: '线程安全的引用计数智能指针,计数用原子操作更新。',
  },
  RefCell: {
    zh: '运行时借用检查',
    ipa: '/rɪˈsel/',
    def: '单线程的内部可变容器:borrow 只读、borrow_mut 可写,冲突时 panic。',
  },
  'interior mutability': {
    zh: '内部可变性',
    ipa: '/ɪnˈtɪəriər ˌmjuːtəˈbɪləti/',
    def: '在共享不可变引用背后修改数据的模式,把借用检查从编译期移到运行时。',
  },
  // ---- 并发编程 ----
  thread: {
    zh: '线程',
    ipa: '/θrɛd/',
    def: 'OS 调度的最小执行单元;Rust 采用 1:1 模型,由 std::thread::spawn 创建并返回 JoinHandle。',
  },
  channel: {
    zh: '通道',
    ipa: '/ˈtʃænəl/',
    def: '线程间传递数据的管道,send 即转移所有权,避免共享内存。',
  },
  mpsc: {
    zh: '多生产者单消费者',
    ipa: '/ˌem piː ɛs ˈsiː/',
    def: '多生产者、单消费者的通道模型,即 std::sync::mpsc,发送端可 clone 多份,接收端只有一个。',
  },
  Mutex: {
    zh: '互斥锁',
    ipa: '/ˈmjuːtɛks/',
    def: '同一时刻只允许一个线程访问内部数据的锁;lock() 返回 MutexGuard,guard drop 自动解锁。',
  },
  Send: {
    zh: '可跨线程转移',
    ipa: '/sɛnd/',
    def: '标记 trait,表示类型的所有权可以安全地转移到另一个线程;Rc 不是,Arc 是。',
  },
  Sync: {
    zh: '可跨线程共享',
    ipa: '/sɪŋk/',
    def: '标记 trait,表示类型的引用 &T 可以安全地在多线程间共享;T: Sync 当且仅当 &T: Send。',
  },
  // ---- 高级特性 ----
  'raw pointer': {
    zh: '裸指针',
    ipa: '/ˌrɔː ˈpɔɪntər/',
    def: '直接指向内存地址、不受借用规则约束的指针类型(*const T / *mut T)。',
  },
  'unsafe block': {
    zh: 'unsafe 块',
    ipa: '/ˈʌnseɪf blɒk/',
    def: '放行五类编译器无法静态验证操作的代码块,其余检查仍然生效。',
  },
  'unsafe trait': {
    zh: '不安全 trait',
    ipa: '/ˈʌnseɪf treɪt/',
    def: '声明为 unsafe 的 trait,实现者需手动保证其不变量,如 Send/Sync。',
  },
  union: {
    zh: '联合类型',
    ipa: '/ˈjuːnjən/',
    def: '所有字段共享同一块内存的结构,读取字段是 unsafe 操作。',
  },
  FFI: {
    zh: '外部函数接口',
    ipa: '/ˌef ef ˈaɪ/',
    def: 'Foreign Function Interface,与 C 等外部语言互操作的接口。',
  },
  'declarative macro': {
    zh: '声明宏',
    ipa: '/dɪˈklærətɪv ˈmækroʊ/',
    def: '用 macro_rules! 定义、按模式匹配在编译期展开的宏。',
  },
  'fragment specifier': {
    zh: '片段说明符',
    ipa: '/ˈfræɡmənt ˈspesɪfaɪə/',
    def: '$x:expr 中的 expr,规定宏分支匹配的语法类别。',
  },
  'macro expansion': {
    zh: '宏展开',
    ipa: '/ɪkˈspænʃən/',
    def: '编译期把宏调用替换为其生成的代码的过程。',
  },
  hygiene: {
    zh: '宏卫生',
    ipa: '/ˈhaɪdʒiːn/',
    def: '宏内部生成的标识符不与调用方作用域混淆的性质。',
  },
  // ---- 实战项目 ----
  'command line argument': {
    zh: '命令行参数',
    ipa: '/kəˈmɑːnd ˈlaɪn ˈɑːɡjumənt/',
    def: '程序启动时由系统传入的字符串序列,std::env::args 以迭代器提供,首元素是程序名。',
  },
  'environment variable': {
    zh: '环境变量',
    ipa: '/ɪnˈvaɪərənmənt ˈveəriəbl/',
    def: '进程从父 shell 继承的键值对,std::env::var 读取,常作不改代码的配置开关。',
  },
  'thread pool': {
    zh: '线程池',
    ipa: '/θred puːl/',
    def: '预先创建固定数量常驻线程复用执行任务,把并发度变成常量而非随连接数增长。',
  },
  worker: {
    zh: '工作线程',
    ipa: '/ˈwɜːkə/',
    def: '线程池中一个常驻线程,循环从共享任务队列取任务执行。',
  },
  'graceful shutdown': {
    zh: '优雅关闭',
    ipa: '/ˈɡreɪsfl ˈʃʌtdaʊn/',
    def: '退出前发终止消息让每个 worker 跑完当前任务并 join,而非直接杀线程。',
  },
}

// 中文别名 → 英文 key,共享同一条目
const aliases: Record<string, string> = {
  所有权: 'ownership',
  移动语义: 'move',
  借用: 'borrow',
  引用: 'reference',
  悬垂引用: 'dangling reference',
  切片: 'slice',
  栈: 'stack',
  堆: 'heap',
  作用域: 'scope',
  深拷贝: 'clone',
}

export const terms: Record<string, TermEntry> = {
  ...entries,
  ...Object.fromEntries(Object.entries(aliases).map(([zh, en]) => [zh, entries[en]])),
}

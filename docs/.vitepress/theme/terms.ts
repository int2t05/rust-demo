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

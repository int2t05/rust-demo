// 文档站配置:base 指向 GH Pages 子路径;withMermaid 启用图表;termWrap 全局术语包裹
import { defineConfig } from 'vitepress'
import { withMermaid } from 'vitepress-plugin-mermaid'
import { termWrap } from './theme/term-plugin'

const section = (
  text: string,
  link: string,
  items: Array<[string, string]>,
) => ({
  text,
  items: [{ text: '总览', link: `${link}/` }, ...items.map(([t, l]) => ({ text: t, link: `${link}/${l}` }))],
})

export default withMermaid(
  defineConfig({
    lang: 'zh-CN',
    title: 'rust-demo',
    description: 'Rust 系统学习 wiki:31 个主题 crate 的配套知识库',
    base: '/rust-demo/',
    cleanUrls: true,
    markdown: {
      config(md) {
        md.use(termWrap)
      },
    },
    themeConfig: {
      nav: [
        { text: '首页', link: '/' },
        { text: '入门基础', link: '/notes/01-入门基础/' },
      ],
      sidebar: {
        '/notes/': [
          section('01 入门基础', '/notes/01-入门基础', [
            ['02 Hello Cargo', '02-Hello-Cargo'],
            ['03 猜数字游戏', '03-猜数字游戏'],
          ]),
          section('02 核心语法', '/notes/02-核心语法', [
            ['01 变量与可变性', '01-变量与可变性'],
            ['02 标量类型', '02-标量类型'],
            ['03 复合类型', '03-复合类型'],
            ['04 函数', '04-函数'],
            ['05 控制流', '05-控制流'],
          ]),
          section('03 所有权系统', '/notes/03-所有权系统', [
            ['01 所有权规则', '01-所有权规则'],
            ['02 引用与借用', '02-引用与借用'],
            ['03 切片类型', '03-切片类型'],
          ]),
          section('04 数据结构', '/notes/04-数据结构', [
            ['01 结构体', '01-结构体'],
            ['02 枚举与Option', '02-枚举与Option'],
            ['03 Match模式匹配', '03-Match模式匹配'],
            ['04 Vec集合', '04-Vec集合'],
            ['05 String字符串', '05-String字符串'],
            ['06 HashMap映射', '06-HashMap映射'],
          ]),
          section('05 模块与组织', '/notes/05-模块与组织', [
            ['01 Package-Crate-Module', '01-Package-Crate-Module'],
          ]),
          section('06 错误处理', '/notes/06-错误处理', [
            ['02 Result可恢复', '02-Result可恢复'],
          ]),
          section('07 泛型Trait生命周期', '/notes/07-泛型Trait生命周期', [
            ['01 泛型', '01-泛型'],
            ['02 Trait特征', '02-Trait特征'],
            ['03 Trait对象', '03-Trait对象'],
            ['04 生命周期标注', '04-生命周期标注'],
          ]),
          section('08 测试', '/notes/08-测试', [
            ['03 测试组织与文档测试', '03-测试组织与文档测试'],
          ]),
          section('09 函数式特性', '/notes/09-函数式特性', [
            ['01 闭包', '01-闭包'],
            ['02 迭代器', '02-迭代器'],
          ]),
          section('11 智能指针', '/notes/11-智能指针', [
            ['01 Box', '01-Box'],
          ]),
          section('12 并发编程', '/notes/12-并发编程', [
            ['01 线程模型', '01-线程模型'],
          ]),
          section('13 高级特性', '/notes/13-高级特性', [
            ['01 Unsafe Rust', '01-Unsafe-Rust'],
            ['03 宏', '03-宏'],
          ]),
          section('14 实战项目', '/notes/14-实战项目', [
            ['01 Cli工具minigrep', '01-Cli工具minigrep'],
            ['02 多线程Web服务器', '02-多线程Web服务器'],
          ]),
        ],
      },
      socialLinks: [{ icon: 'github', link: 'https://github.com/int2t05/rust-demo' }],
    },
  }),
)

// 文档站配置:base 指向 GH Pages 子路径;withMermaid 启用图表;termWrap 全局术语包裹
import { defineConfig } from 'vitepress'
import { withMermaid } from 'vitepress-plugin-mermaid'
import { termWrap } from './theme/term-plugin'

export default withMermaid(
  defineConfig({
    lang: 'zh-CN',
    title: 'rust-demo',
    description: 'Rust 系统学习 wiki:31 个主题 crate 的配套知识库',
    base: '/rust-demo/',
    markdown: {
      config(md) {
        md.use(termWrap)
      },
    },
    themeConfig: {
      nav: [
        { text: '首页', link: '/' },
        { text: '所有权系统', link: '/notes/03-所有权系统/' },
      ],
      sidebar: {
        '/notes/': [
          {
            text: '03 所有权系统',
            items: [
              { text: '总览', link: '/notes/03-所有权系统/' },
              { text: '01 所有权规则', link: '/notes/03-所有权系统/01-所有权规则' },
              { text: '02 引用与借用', link: '/notes/03-所有权系统/02-引用与借用' },
              { text: '03 切片类型', link: '/notes/03-所有权系统/03-切片类型' },
            ],
          },
        ],
      },
      socialLinks: [{ icon: 'github', link: 'https://github.com/int2t05/rust-demo' }],
    },
  }),
)

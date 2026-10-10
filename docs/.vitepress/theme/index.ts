// 主题入口:扩展默认主题,注册全局 Term 组件(markdown 中的 <Term> 由此解析)
import DefaultTheme from 'vitepress/theme'
import Term from './Term.vue'

export default {
  extends: DefaultTheme,
  enhanceApp({ app }) {
    app.component('Term', Term)
  },
}

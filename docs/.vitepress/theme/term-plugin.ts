// markdown-it 插件:正文中的字典术语自动包裹成 <Term k="key">原文</Term>
// 边界规则:
// - 只处理块级 inline token 的 text 子 token → 围栏代码块天然跳过
// - code_inline、链接内部(link_open..link_close)、标题(heading_open..close)显式跳过
// - 单一合并正则模块级编译一次;键按长度降序 → 长词优先(borrow checker 先于 borrow)
// - 英文 \b词s?\b(大小写不敏感,复数回退仅去 s,宁漏勿错);中文子串匹配
// - 产出 html_inline,不再被二次扫描 → 幂等
import type MarkdownIt from 'markdown-it'
import type Token from 'markdown-it/lib/token.mjs'
import { terms } from './terms'

const escapeRe = (s: string) => s.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')
const isEnglish = (k: string) => /^[a-zA-Z][a-zA-Z ]*$/.test(k)

// 长词优先,避免 borrow checker 被 borrow 截断
const alts = Object.keys(terms)
  .sort((a, b) => b.length - a.length)
  .map((k) => (isEnglish(k) ? `\\b${escapeRe(k)}s?\\b` : escapeRe(k)))
  .join('|')
const termRe = new RegExp(alts, 'gi')

// 由匹配文本反查字典 key:小写不敏感查找(兼容 Option/Vec 等大写原词 key),再试去尾 s(复数回退)
const lowerKeys = new Map(Object.keys(terms).map((k) => [k.toLowerCase(), k]))
function resolveKey(text: string): string | undefined {
  const lower = text.toLowerCase()
  const exact = lowerKeys.get(lower)
  if (exact) return exact
  const singular = lowerKeys.get(lower.replace(/s$/, ''))
  return singular
}

// 把一段纯文本切成 text / html_inline 混合片段;纯文本部分保持 text 以正常转义
function wrapText(content: string): Array<{ type: 'text' | 'html_inline'; content: string }> {
  const out: Array<{ type: 'text' | 'html_inline'; content: string }> = []
  let last = 0
  termRe.lastIndex = 0
  for (let m: RegExpExecArray | null; (m = termRe.exec(content)); ) {
    const key = resolveKey(m[0])
    if (!key) continue
    if (m.index > last) out.push({ type: 'text', content: content.slice(last, m.index) })
    out.push({ type: 'html_inline', content: `<Term k="${key}">${m[0]}</Term>` })
    last = m.index + m[0].length
  }
  if (last < content.length) out.push({ type: 'text', content: content.slice(last) })
  return out
}

function wrapChildren(state: { Token: typeof Token }, children: Token[]): void {
  const next: Token[] = []
  let linkDepth = 0
  for (const child of children) {
    // 链接内部原样保留;行内代码原样保留
    if (child.type === 'link_open') linkDepth++
    if (child.type === 'link_close') linkDepth--
    if (linkDepth > 0 || child.type === 'code_inline') {
      next.push(child)
      continue
    }
    if (child.type !== 'text') {
      next.push(child)
      continue
    }
    for (const seg of wrapText(child.content)) {
      const tok = new state.Token(seg.type, '', 0)
      tok.content = seg.content
      next.push(tok)
    }
  }
  children.length = 0
  children.push(...next)
}

export function termWrap(md: MarkdownIt): void {
  md.core.ruler.push('term_wrap', (state) => {
    let inHeading = false
    for (const tok of state.tokens) {
      if (tok.type === 'heading_open') inHeading = true
      if (tok.type === 'heading_close') inHeading = false
      if (tok.type === 'inline' && !inHeading && tok.children) {
        wrapChildren(state, tok.children)
      }
    }
    return true
  })
}

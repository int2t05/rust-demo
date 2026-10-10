<script setup lang="ts">
// 术语悬浮弹窗:纯 CSS hover 实现,无 JS 状态;--vp-c-* 变量自动适配暗色模式
import { computed } from 'vue'
import { terms } from './terms'

const props = defineProps<{ k: string }>()
const entry = computed(() => terms[props.k])
</script>

<template>
  <span v-if="entry" class="term-wrap">
    <slot />
    <span class="term-pop" role="tooltip">
      <span v-if="entry.ipa" class="term-ipa">{{ entry.ipa }}</span>
      <span v-if="entry.zh" class="term-zh">{{ entry.zh }}</span>
      <span class="term-def">{{ entry.def }}</span>
    </span>
  </span>
  <slot v-else />
</template>

<style scoped>
.term-wrap {
  position: relative;
  display: inline-block;
  text-decoration: underline dotted var(--vp-c-brand-1);
  text-decoration-thickness: 1px;
  text-underline-offset: 4px;
  cursor: help;
}

.term-pop {
  position: absolute;
  left: 50%;
  bottom: calc(100% + 8px);
  transform: translateX(-50%) translateY(4px);
  z-index: 30;
  width: max-content;
  max-width: 320px;
  padding: 10px 12px;
  border: 1px solid var(--vp-c-divider);
  border-radius: 8px;
  background: var(--vp-c-bg);
  box-shadow: var(--vp-shadow-2);
  font-size: 13px;
  line-height: 1.6;
  font-weight: 400;
  text-align: left;
  text-decoration: none;
  white-space: normal;
  opacity: 0;
  visibility: hidden;
  transition: opacity 0.15s, visibility 0.15s, transform 0.15s;
  pointer-events: none;
}

.term-wrap:hover .term-pop,
.term-wrap:focus-within .term-pop {
  opacity: 1;
  visibility: visible;
  transform: translateX(-50%) translateY(0);
}

.term-ipa {
  display: block;
  margin-bottom: 2px;
  color: var(--vp-c-brand-1);
  font-family: var(--vp-font-family-mono);
  font-size: 12px;
}

.term-zh {
  display: block;
  margin-bottom: 2px;
  color: var(--vp-c-text-1);
  font-weight: 600;
}

.term-def {
  display: block;
  color: var(--vp-c-text-2);
}
</style>

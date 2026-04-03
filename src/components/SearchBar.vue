<script setup lang="ts">
import { ref } from 'vue'

defineProps<{
  modelValue: string
}>()

const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void
  (e: 'select', plugin: any): void
  (e: 'navigate', direction: 'up' | 'down'): void
}>()

const inputRef = ref<HTMLInputElement | null>(null)

const handleInput = (e: Event) => {
  const target = e.target as HTMLInputElement
  emit('update:modelValue', target.value)
}

const handleKeydown = (e: KeyboardEvent) => {
  if (e.key === 'Escape') {
    emit('update:modelValue', '')
  } else if (e.key === 'Enter') {
    // 触发选择第一个插件的事件
    emit('select', null)
  } else if (e.key === 'ArrowDown') {
    // 向下导航到结果列表
    e.preventDefault()
    emit('navigate', 'down')
  } else if (e.key === 'ArrowUp') {
    // 向上导航到结果列表
    e.preventDefault()
    emit('navigate', 'up')
  }
}

defineExpose({
  focus: () => inputRef.value?.focus()
})
</script>

<template>
  <div class="search-bar">
    <span class="search-icon">🔍</span>
    <input
      ref="inputRef"
      type="text"
      :value="modelValue"
      @input="handleInput"
      @keydown="handleKeydown"
      @keydown.enter="emit('select', null)"
      placeholder="搜索工具... (计算器, 日历, 笔记, 取色器)"
      class="search-input"
      autofocus
    />
    <span v-if="modelValue" class="clear-btn" @click="emit('update:modelValue', '')">✕</span>
  </div>
</template>

<style scoped>
.search-bar {
  display: flex;
  align-items: center;
  background: rgba(255, 255, 255, 0.08);
  border-radius: 12px;
  padding: 12px 16px;
  border: 1px solid rgba(255, 255, 255, 0.1);
  transition: all 0.2s;
}

.search-bar:focus-within {
  border-color: rgba(100, 150, 255, 0.5);
  box-shadow: 0 0 0 3px rgba(100, 150, 255, 0.1);
}

.search-icon {
  font-size: 18px;
  margin-right: 12px;
  opacity: 0.7;
}

.search-input {
  flex: 1;
  background: transparent;
  border: none;
  outline: none;
  color: #fff;
  font-size: 16px;
}

.search-input::placeholder {
  color: rgba(255, 255, 255, 0.4);
}

.clear-btn {
  cursor: pointer;
  opacity: 0.5;
  padding: 4px;
  font-size: 12px;
}

.clear-btn:hover {
  opacity: 1;
}
</style>
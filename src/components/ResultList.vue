<script setup lang="ts">
import { watch } from 'vue'

interface ResultPlugin {
  id: string
  name: string
  nameZh: string
  icon: string
  keywords: string[]
  useCount: number
}

const props = defineProps<{
  plugins: ResultPlugin[]
  selectedIndex?: number
}>()

const emit = defineEmits<{
  (e: 'select', plugin: ResultPlugin): void
  (e: 'update:selectedIndex', index: number): void
}>()

// 插件列表变化时把选中索引重置回 0；索引以 App 为唯一数据源，避免回车打开非高亮项
watch(() => props.plugins, () => {
  emit('update:selectedIndex', 0)
})

const handleItemClick = (plugin: ResultPlugin) => {
  emit('select', plugin)
}
</script>

<template>
  <div class="result-list">
    <div v-for="(plugin, index) in plugins" :key="plugin.id"
      :class="['result-item', { selected: index === (selectedIndex ?? 0) }]" @click="handleItemClick(plugin)">
      <span class="plugin-icon">{{ plugin.icon }}</span>
      <div class="plugin-info">
        <div class="plugin-name">{{ plugin.nameZh }}</div>
        <div class="plugin-keywords">
          <span class="kw-text">{{ plugin.keywords.slice(0, 4).join(', ') }}</span>
          <span v-if="plugin.useCount > 0" class="use-count">🔥 {{ plugin.useCount }}</span>
        </div>
      </div>
    </div>
    <div v-if="plugins.length === 0" class="no-result">
      未找到相关工具
    </div>
  </div>
</template>

<style scoped>
.result-list {
  margin-top: 16px;
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
  min-height: 0;
}

.result-list::-webkit-scrollbar {
  width: 6px;
}

.result-list::-webkit-scrollbar-track {
  background: rgba(255, 255, 255, 0.05);
  border-radius: 3px;
}

.result-list::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.2);
  border-radius: 3px;
}

.result-list::-webkit-scrollbar-thumb:hover {
  background: rgba(255, 255, 255, 0.3);
}

.result-item {
  display: flex;
  align-items: center;
  padding: 14px 16px;
  margin-bottom: 8px;
  background: rgba(255, 255, 255, 0.05);
  border-radius: 10px;
  cursor: pointer;
  transition: all 0.2s;
  border: 1px solid transparent;
}

.result-item:hover {
  background: rgba(255, 255, 255, 0.1);
  border-color: rgba(100, 150, 255, 0.3);
  transform: translateX(4px);
}

.result-item.selected {
  background: rgba(100, 150, 255, 0.2);
  border-color: rgba(100, 150, 255, 0.5);
  transform: translateX(4px);
}



.plugin-icon {
  font-size: 28px;
  margin-right: 16px;
}

.plugin-info {
  flex: 1;
}

.plugin-name {
  color: #fff;
  font-size: 15px;
  font-weight: 500;
  margin-bottom: 4px;
}

.plugin-keywords {
  color: rgba(255, 255, 255, 0.4);
  font-size: 12px;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.kw-text {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.no-result {
  text-align: center;
  color: rgba(255, 255, 255, 0.4);
  padding: 40px;
}

.use-count {
  color: #ff6b6b;
  font-size: 11px;
  margin-left: 8px;
}
</style>
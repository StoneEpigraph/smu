<script setup lang="ts">
defineProps<{
  plugins: any[]
}>()

const emit = defineEmits<{
  (e: 'select', plugin: any): void
}>()
</script>

<template>
  <div class="result-list">
    <div 
      v-for="plugin in plugins" 
      :key="plugin.id"
      class="result-item"
      @click="emit('select', plugin)"
    >
      <span class="plugin-icon">{{ plugin.icon }}</span>
      <div class="plugin-info">
        <div class="plugin-name">{{ plugin.nameZh }}</div>
        <div class="plugin-keywords">{{ plugin.keywords.slice(0, 4).join(', ') }}</div>
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
  max-height: 320px;
  overflow-y: auto;
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
}

.no-result {
  text-align: center;
  color: rgba(255, 255, 255, 0.4);
  padding: 40px;
}
</style>
<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, nextTick } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { invoke } from '@tauri-apps/api/core'
import { register, unregister } from '@tauri-apps/plugin-global-shortcut'
import SearchBar from './components/SearchBar.vue'
import ResultList from './components/ResultList.vue'
import Calculator from './components/plugins/Calculator.vue'
import ColorPicker from './components/plugins/ColorPicker.vue'
import Calendar from './components/plugins/Calendar.vue'
import QuickNote from './components/plugins/QuickNote.vue'
import Encoder from './components/plugins/Encoder.vue'
import IdCard from './components/plugins/IdCard.vue'
import TimeConverter from './components/plugins/TimeConverter.vue'

interface Plugin {
  id: string
  name: string
  nameZh: string
  icon: string
  keywords: string[]
  component: any
}

const plugins: Plugin[] = [
  {
    id: 'calculator',
    name: 'Calculator',
    nameZh: '计算器',
    icon: '🔢',
    keywords: ['calc', '计算', '数学', '加', '减', '乘', '除'],
    component: Calculator
  },
  {
    id: 'colorpicker',
    name: 'Color Picker',
    nameZh: '取色器',
    icon: '🎨',
    keywords: ['color', '颜色', '取色', 'rgb', 'hex'],
    component: ColorPicker
  },
  {
    id: 'calendar',
    name: 'Calendar',
    nameZh: '日历',
    icon: '📅',
    keywords: ['calendar', '日历', '日期', '时间', 'cal'],
    component: Calendar
  },
  {
    id: 'quicknote',
    name: 'Quick Note',
    nameZh: '快捷笔记',
    icon: '📝',
    keywords: ['note', '笔记', '记录', 'memo', 'note'],
    component: QuickNote
  },
  {
    id: 'encoder',
    name: 'Encoder',
    nameZh: '编码工具',
    icon: '🔐',
    keywords: ['encode', '编码', 'md5', 'base64', 'sha', 'url', 'unicode', 'hex'],
    component: Encoder
  },
  {
    id: 'idcard',
    name: 'IdCard',
    nameZh: '身份证工具',
    icon: '🎫',
    keywords: ['idcard', '身份证', '验证', '生成', 'sfz'],
    component: IdCard
  },
  {
    id: 'time',
    name: 'TimeConverter',
    nameZh: '时间转换',
    icon: '⏰',
    keywords: ['时间', 'timestamp', '时间戳', '日期'],
    component: TimeConverter
  }
]

const searchQuery = ref('')
const selectedPlugin = ref<Plugin | null>(null)
const searchInputRef = ref<HTMLInputElement | null>(null)
const useCount = ref<Record<string, number>>({})
const selectedIndex = ref(0)

const loadUseCount = async () => {
  try {
    const counts = await invoke<[string, number][]>('get_use_counts')
    counts.forEach(([id, count]) => {
      useCount.value[id] = count
    })
  } catch (e) {
    console.log('Load use count failed:', e)
  }
}

const incrementUseCount = async (pluginId: string) => {
  try {
    await invoke('increment_use_count', { pluginId })
    useCount.value[pluginId] = (useCount.value[pluginId] || 0) + 1
  } catch (e) {
    console.log('Increment use count failed:', e)
  }
}

const handleKeydown = async (e: KeyboardEvent) => {
  const win = getCurrentWindow()
  
  // Ctrl + Q 隐藏窗口
  if (e.ctrlKey && e.key === 'q') {
    e.preventDefault()
    await win.hide()
    return
  }
  
  // 单次 Esc 或连续两次 Esc 都隐藏
  if (e.key === 'Escape') {
    e.preventDefault()
    await win.hide()
  }
}

const filteredPlugins = computed(() => {
  let result = [...plugins]
  
  if (searchQuery.value.trim()) {
    const query = searchQuery.value.toLowerCase()
    result = result.filter(plugin => {
      if (plugin.name.toLowerCase().includes(query)) return true
      if (plugin.nameZh.includes(query)) return true
      return plugin.keywords.some(kw => kw.toLowerCase().includes(query))
    })
  }
  
  result.sort((a, b) => {
    const countA = useCount.value[a.id] || 0
    const countB = useCount.value[b.id] || 0
    return countB - countA
  })
  
  return result.map(plugin => ({
    ...plugin,
    useCount: useCount.value[plugin.id] || 0
  }))
})

const handleSelectPlugin = async (plugin: Plugin | null) => {
  if (plugin) {
    selectedPlugin.value = plugin
    await incrementUseCount(plugin.id)
  } else if (filteredPlugins.value.length > 0) {
    // 直接回车，选择第一个插件
    const firstPlugin = filteredPlugins.value[0]
    selectedPlugin.value = firstPlugin
    await incrementUseCount(firstPlugin.id)
  }
}

const handleNavigate = (direction: 'up' | 'down') => {
  if (filteredPlugins.value.length === 0) return
  
  if (direction === 'down') {
    selectedIndex.value = (selectedIndex.value + 1) % filteredPlugins.value.length
  } else {
    selectedIndex.value = (selectedIndex.value - 1 + filteredPlugins.value.length) % filteredPlugins.value.length
  }
  
  // 聚焦到结果列表
  const resultList = document.querySelector('.result-list')
  if (resultList) {
    (resultList as HTMLElement).focus()
  }
}

const handleBack = () => {
  selectedPlugin.value = null
  nextTick(() => {
    searchInputRef.value?.focus()
  })
}

onMounted(async () => {
  document.addEventListener('keydown', handleKeydown)
  
  await loadUseCount()
  
  try {
    await register('Super+S', async () => {
      const win = getCurrentWindow()
      const isVisible = await win.isVisible()
      if (isVisible) {
        await win.hide()
      } else {
        await win.show()
        await win.setFocus()
        searchInputRef.value?.focus()
      }
    })
  } catch (e) {
    console.log('Failed to register shortcut:', e)
  }
})

onUnmounted(async () => {
  document.removeEventListener('keydown', handleKeydown)
  try {
    await unregister('Super+S')
  } catch (e) {
    console.log('Failed to unregister shortcut:', e)
  }
})
</script>

<template>
  <div class="app-container" @keydown="handleKeydown" tabindex="-1">
    <div class="main-window">
      <div v-if="!selectedPlugin" class="search-section">
        <SearchBar 
          v-model="searchQuery" 
          ref="searchInputRef"
          @select="handleSelectPlugin"
          @navigate="handleNavigate"
        />
        <ResultList 
          :plugins="filteredPlugins" 
          :selectedIndex="selectedIndex"
          @update:selectedIndex="(index) => selectedIndex = index"
          @select="handleSelectPlugin"
        />
      </div>
      <div v-else class="plugin-section">
        <div class="plugin-header">
          <button class="back-btn" @click="handleBack">← 返回</button>
          <span class="plugin-title">{{ selectedPlugin.icon }} {{ selectedPlugin.nameZh }}</span>
        </div>
        <component 
          :is="selectedPlugin.component"
        />
      </div>
    </div>
  </div>
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html, body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  background: transparent;
  overflow: hidden;
  height: 100%;
}

.app-container {
  width: 100%;
  height: 100vh;
  display: flex;
  justify-content: center;
  padding-top: 50px;
}

.main-window {
  width: 800px;
  height: 600px;
  background: rgba(30, 30, 35, 0.95);
  border-radius: 20px;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.5);
  overflow: hidden;
  border: 1px solid rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(20px);
  display: flex;
  flex-direction: column;
}

.search-section {
  padding: 24px;
  overflow-y: auto;
  flex: 1;
}

.plugin-section {
  height: 520px;
  display: flex;
  flex-direction: column;
}

.plugin-header {
  display: flex;
  align-items: center;
  padding: 16px 20px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.back-btn {
  background: rgba(255, 255, 255, 0.1);
  border: none;
  color: #fff;
  padding: 8px 16px;
  border-radius: 8px;
  cursor: pointer;
  font-size: 14px;
  margin-right: 16px;
  transition: background 0.2s;
}

.back-btn:hover {
  background: rgba(255, 255, 255, 0.2);
}

.plugin-title {
  color: #fff;
  font-size: 16px;
  font-weight: 500;
}
</style>
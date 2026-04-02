<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, nextTick } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { register, unregister } from '@tauri-apps/plugin-global-shortcut'
import SearchBar from './components/SearchBar.vue'
import ResultList from './components/ResultList.vue'
import Calculator from './components/plugins/Calculator.vue'
import ColorPicker from './components/plugins/ColorPicker.vue'
import Calendar from './components/plugins/Calendar.vue'
import QuickNote from './components/plugins/QuickNote.vue'
import Encoder from './components/plugins/Encoder.vue'

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
  }
]

const searchQuery = ref('')
const selectedPlugin = ref<Plugin | null>(null)
const searchInputRef = ref<HTMLInputElement | null>(null)

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
  if (!searchQuery.value.trim()) {
    return plugins
  }
  const query = searchQuery.value.toLowerCase()
  return plugins.filter(plugin => {
    if (plugin.name.toLowerCase().includes(query)) return true
    if (plugin.nameZh.includes(query)) return true
    return plugin.keywords.some(kw => kw.toLowerCase().includes(query))
  })
})

const handleSelectPlugin = (plugin: Plugin) => {
  selectedPlugin.value = plugin
}

const handleBack = () => {
  selectedPlugin.value = null
  nextTick(() => {
    searchInputRef.value?.focus()
  })
}

onMounted(async () => {
  document.addEventListener('keydown', handleKeydown)
  
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
        />
        <ResultList 
          :plugins="filteredPlugins" 
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
          :initialInput="searchQuery"
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
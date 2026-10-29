<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, nextTick } from 'vue'
import type { Component } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { invoke } from '@tauri-apps/api/core'
// 快捷键现在由后端全局处理，前端不再需要注册
import SearchBar from './components/SearchBar.vue'
import ResultList from './components/ResultList.vue'
import Settings from './components/Settings.vue'
import Calculator from './components/plugins/Calculator.vue'
import ColorPicker from './components/plugins/ColorPicker.vue'
import Calendar from './components/plugins/Calendar.vue'
import QuickNote from './components/plugins/QuickNote.vue'
import Encoder from './components/plugins/Encoder.vue'
import IdCard from './components/plugins/IdCard.vue'
import TimeConverter from './components/plugins/TimeConverter.vue'
import JsonFormatter from './components/plugins/JsonFormatter.vue'
import Sm2 from './components/plugins/Sm2.vue'
import type { AppSettings, PluginConfig } from './types/settings'

interface Plugin {
  id: string
  name: string
  nameZh: string
  icon: string
  keywords: string[]
  component: Component
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
    keywords: ['encode', '编码', 'md5', 'base64', 'sha', 'sm3', 'url', 'unicode', 'hex'],
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
  },
  {
    id: 'jsonformatter',
    name: 'JSON Formatter',
    nameZh: 'JSON格式化',
    icon: '📝',
    keywords: ['json', '格式化', 'format', 'parse'],
    component: JsonFormatter
  },
  {
    id: 'sm2',
    name: 'SM2',
    nameZh: 'SM2加密',
    icon: '🔐',
    keywords: ['sm2', '加密', '解密', '签名', '验签', '国密'],
    component: Sm2
  }
]

const searchQuery = ref('')
const selectedPlugin = ref<Plugin | null>(null)
const searchInputRef = ref<HTMLInputElement | null>(null)
const useCount = ref<Record<string, number>>({})
const selectedIndex = ref(0)
const appVersion = ref('1.0.0')
const showSettings = ref(false)
const appSettings = ref<AppSettings | null>(null)

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

const loadAppVersion = async () => {
  try {
    const version = await invoke<string>('get_app_version')
    appVersion.value = version
  } catch (e) {
    console.log('Load app version failed:', e)
  }
}

const handleOpenSettings = () => {
  showSettings.value = true
}

const handleCloseSettings = () => {
  showSettings.value = false
}

const handleSaveSettings = async (settings: AppSettings) => {
  try {
    console.log('Saving settings:', settings)
    const settingsJson = JSON.stringify(settings)
    console.log('Settings JSON:', settingsJson)
    await invoke('save_settings', { settingsJson })
    appSettings.value = settings
    console.log('Settings saved successfully:', settings)
    console.log('Current appSettings:', appSettings.value)
  } catch (e) {
    console.error('Failed to save settings:', e)
  }
  showSettings.value = false
}

const loadAppSettings = async () => {
  try {
    console.log('Loading settings...')
    const settingsJson = await invoke<string>('load_settings')
    console.log('Raw settings JSON:', settingsJson)
    if (settingsJson && settingsJson !== 'null') {
      appSettings.value = JSON.parse(settingsJson) as AppSettings
      console.log('Settings loaded successfully:', appSettings.value)
      console.log('Plugin settings:', appSettings.value?.plugins)
    } else {
      console.log('No settings found, using defaults')
    }
  } catch (e) {
    console.log('Failed to load settings:', e)
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

  // Alt + P 返回
  if (e.altKey && e.key === 'p') {
    e.preventDefault()
    handleBack()
    return
  }

  // Esc：插件内返回 → 清空搜索 → 隐藏窗口
  if (e.key === 'Escape') {
    e.preventDefault()
    if (selectedPlugin.value) {
      handleBack()
    } else if (searchQuery.value) {
      searchQuery.value = ''
    } else {
      await win.hide()
    }
  }
}

type ResizeDirectionType = 'East' | 'North' | 'NorthEast' | 'NorthWest' | 'South' | 'SouthEast' | 'SouthWest' | 'West'

const handleMinimize = async () => {
  await getCurrentWindow().minimize()
}

const handleHideWindow = async () => {
  await getCurrentWindow().hide()
}

const handleToggleMaximize = async () => {
  await getCurrentWindow().toggleMaximize()
}

const startResize = async (direction: ResizeDirectionType) => {
  try {
    await getCurrentWindow().startResizeDragging(direction)
  } catch (e) {
    console.log('Resize dragging failed:', e)
  }
}

const filteredPlugins = computed(() => {
  let result = [...plugins]

  // 过滤掉被禁用的插件
  const savedPlugins = appSettings.value?.plugins
  if (savedPlugins) {
    result = result.filter(plugin => {
      const pluginSettings = savedPlugins.find(p => p.id === plugin.id)
      return pluginSettings ? pluginSettings.enabled : true
    })
  }

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

const currentPluginConfig = computed(() => {
  if (!selectedPlugin.value || !appSettings.value?.plugins) {
    return null
  }
  const pluginSettings = appSettings.value.plugins.find((p: PluginConfig) => p.id === selectedPlugin.value?.id)
  return pluginSettings?.config || null
})

// 只依赖 id：ResultList 发出的插件对象不含 component 字段，从插件注册表按 id 取回完整定义
const handleSelectPlugin = async (selected: { id: string } | null) => {
  const target = selected ? plugins.find(p => p.id === selected.id) : null
  if (target) {
    selectedPlugin.value = target
    await incrementUseCount(target.id)
  } else if (selected === null && filteredPlugins.value.length > 0) {
    // 直接回车，选择当前选中的插件
    const selectedPluginItem = filteredPlugins.value[selectedIndex.value]
    if (selectedPluginItem) {
      selectedPlugin.value = selectedPluginItem
      await incrementUseCount(selectedPluginItem.id)
    }
  }
}

const handleNavigate = (direction: 'up' | 'down') => {
  if (filteredPlugins.value.length === 0) return

  if (direction === 'down') {
    selectedIndex.value = (selectedIndex.value + 1) % filteredPlugins.value.length
  } else {
    selectedIndex.value = (selectedIndex.value - 1 + filteredPlugins.value.length) % filteredPlugins.value.length
  }

  // 保持焦点在输入框
  searchInputRef.value?.focus()
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
  await loadAppVersion()
  await loadAppSettings()
})

onUnmounted(async () => {
  document.removeEventListener('keydown', handleKeydown)
})
</script>

<template>
  <div class="app-container">
    <div class="main-window">
      <div class="titlebar" data-tauri-drag-region>
        <span class="titlebar-title" data-tauri-drag-region>SMU</span>
        <div class="titlebar-actions">
          <button class="win-btn" title="最小化" @click="handleMinimize">─</button>
          <button class="win-btn" title="最大化" @click="handleToggleMaximize">□</button>
          <button class="win-btn win-btn-close" title="关闭" @click="handleHideWindow">✕</button>
        </div>
      </div>
      <div v-if="!selectedPlugin" class="main-search-section">
        <SearchBar v-model="searchQuery" ref="searchInputRef" @select="handleSelectPlugin" @navigate="handleNavigate"
          @openSettings="handleOpenSettings" />
        <ResultList :plugins="filteredPlugins" :selectedIndex="selectedIndex"
          @update:selectedIndex="(index) => selectedIndex = index" @select="handleSelectPlugin" />
      </div>
      <div v-else class="plugin-section">
        <div class="plugin-header">
          <button class="back-btn" @click="handleBack">← 返回</button>
          <span class="plugin-title">{{ selectedPlugin.icon }} {{ selectedPlugin.nameZh }}</span>
        </div>
        <component :is="selectedPlugin.component" :config="currentPluginConfig" />
      </div>
      <div class="footer">
        <span class="copyright">© 2025 StoneMind. All rights reserved.</span>
        <span class="version">Version {{ appVersion }}</span>
      </div>
    </div>
    <Settings v-if="showSettings" :initialSettings="appSettings" @close="handleCloseSettings"
      @saveSettings="handleSaveSettings" />
    <!-- 无边框窗口的四边/四角缩放热区 -->
    <div class="resize-handle rh-n" @mousedown.prevent="startResize('North')"></div>
    <div class="resize-handle rh-s" @mousedown.prevent="startResize('South')"></div>
    <div class="resize-handle rh-e" @mousedown.prevent="startResize('East')"></div>
    <div class="resize-handle rh-w" @mousedown.prevent="startResize('West')"></div>
    <div class="resize-handle rh-nw" @mousedown.prevent="startResize('NorthWest')"></div>
    <div class="resize-handle rh-ne" @mousedown.prevent="startResize('NorthEast')"></div>
    <div class="resize-handle rh-sw" @mousedown.prevent="startResize('SouthWest')"></div>
    <div class="resize-handle rh-se" @mousedown.prevent="startResize('SouthEast')"></div>
  </div>
</template>

<style scoped>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html,
body {
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
  align-items: center;
}

.main-window {
  min-width: 800px;
  min-height: 600px;
  width: 100%;
  height: 100%;
  background: rgba(30, 30, 35, 0.95);
  border-radius: 0;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.5);
  overflow: hidden;
  border: 1px solid rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(20px);
  display: flex;
  flex-direction: column;
}

.main-search-section {
  padding: 24px;
  overflow-y: auto;
  flex: 1;
  display: flex;
  flex-direction: column;
  height: 100%;
}

.plugin-section {
  flex: 1;
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: auto;
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

.footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 24px;
  border-top: 1px solid rgba(255, 255, 255, 0.1);
  background: rgba(0, 0, 0, 0.2);
  flex-shrink: 0;
}

.copyright {
  color: rgba(255, 255, 255, 0.5);
  font-size: 12px;
}

.version {
  color: rgba(255, 255, 255, 0.5);
  font-size: 12px;
}

/* ===== 自定义窄标题栏 ===== */
.titlebar {
  height: 32px;
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 4px 0 11px;
  background: rgba(255, 255, 255, 0.03);
  border-bottom: 1px solid rgba(255, 255, 255, 0.08);
  user-select: none;
}

.titlebar-title {
  font-size: 12px;
  font-weight: 600;
  color: rgba(255, 255, 255, 0.78);
  letter-spacing: 0.8px;
  pointer-events: none;
}

.titlebar-actions {
  display: flex;
  gap: 3px;
}

.win-btn {
  width: 26px;
  height: 22px;
  padding: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(255, 255, 255, 0.07);
  border: none;
  border-radius: 4px;
  color: rgba(255, 255, 255, 0.85);
  cursor: pointer;
  font-size: 13px;
  line-height: 1;
  transition: background 0.15s, color 0.15s;
}

.win-btn:hover {
  background: rgba(255, 255, 255, 0.22);
  color: #fff;
}

.win-btn-close:hover {
  background: #e81123;
  color: #fff;
}

/* ===== 无边框窗口缩放热区 ===== */
.resize-handle {
  position: fixed;
  z-index: 9999;
}

.rh-n {
  top: 0;
  left: 8px;
  right: 8px;
  height: 4px;
  cursor: n-resize;
}

.rh-s {
  bottom: 0;
  left: 8px;
  right: 8px;
  height: 4px;
  cursor: s-resize;
}

.rh-e {
  right: 0;
  top: 8px;
  bottom: 8px;
  width: 4px;
  cursor: e-resize;
}

.rh-w {
  left: 0;
  top: 8px;
  bottom: 8px;
  width: 4px;
  cursor: w-resize;
}

.rh-nw {
  top: 0;
  left: 0;
  width: 8px;
  height: 8px;
  cursor: nw-resize;
}

.rh-ne {
  top: 0;
  right: 0;
  width: 8px;
  height: 8px;
  cursor: ne-resize;
}

.rh-sw {
  bottom: 0;
  left: 0;
  width: 8px;
  height: 8px;
  cursor: sw-resize;
}

.rh-se {
  bottom: 0;
  right: 0;
  width: 8px;
  height: 8px;
  cursor: se-resize;
}
</style>
<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, nextTick } from 'vue'
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
const appSettings = ref<any>(null)

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

const handleSaveSettings = async (settings: any) => {
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
      appSettings.value = JSON.parse(settingsJson)
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

  // 单次 Esc 或连续两次 Esc 都隐藏
  if (e.key === 'Escape') {
    e.preventDefault()
    await win.hide()
  }
}

const filteredPlugins = computed(() => {
  let result = [...plugins]

  // 过滤掉被禁用的插件
  if (appSettings.value?.plugins) {
    result = result.filter(plugin => {
      const pluginSettings = appSettings.value.plugins.find((p: any) => p.id === plugin.id)
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
  const pluginSettings = appSettings.value.plugins.find((p: any) => p.id === selectedPlugin.value?.id)
  return pluginSettings?.config || null
})

const handleSelectPlugin = async (plugin: Plugin | null) => {
  if (plugin) {
    selectedPlugin.value = plugin
    await incrementUseCount(plugin.id)
  } else if (filteredPlugins.value.length > 0) {
    // 直接回车，选择当前选中的插件
    const selectedPluginItem = filteredPlugins.value[selectedIndex.value]
    selectedPlugin.value = selectedPluginItem
    await incrementUseCount(selectedPluginItem.id)
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
  <div class="app-container" @keydown="handleKeydown" tabindex="-1">
    <div class="main-window">
      <div v-if="!selectedPlugin" class="search-section">
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
  </div>
</template>

<style>
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
  overflow: auto;
  border: 1px solid rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(20px);
  display: flex;
  flex-direction: column;
}

.search-section {
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
</style>
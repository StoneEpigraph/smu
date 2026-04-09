<script setup lang="ts">
import { ref, watch, onMounted } from 'vue'

const props = defineProps<{
  initialSettings?: any
}>()

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'saveSettings', settings: any): void
}>()

interface PluginConfig {
  id: string
  name: string
  nameZh: string
  icon: string
  enabled: boolean
  config: Record<string, any>
}

const menuItems = [
  { id: 'system', name: '系统', icon: '⚙️' },
  { id: 'plugins', name: '插件', icon: '🔌' }
]

const pluginConfigs = ref<PluginConfig[]>([
  {
    id: 'calculator',
    name: 'Calculator',
    nameZh: '计算器',
    icon: '🔢',
    enabled: true,
    config: {
      precision: 8,
      autoCalculate: true
    }
  },
  {
    id: 'colorpicker',
    name: 'ColorPicker',
    nameZh: '取色器',
    icon: '🎨',
    enabled: true,
    config: {
      defaultFormat: 'hex',
      showAlpha: true
    }
  },
  {
    id: 'encoder',
    name: 'Encoder',
    nameZh: '编码工具',
    icon: '🔐',
    enabled: true,
    config: {
      enabledTypes: ['md5', 'sha256', 'base64', 'url'],
      defaultEncodeType: 'md5'
    }
  },
  {
    id: 'calendar',
    name: 'Calendar',
    nameZh: '日历',
    icon: '📅',
    enabled: true,
    config: {
      showLunar: true,
      defaultView: 'month'
    }
  },
  {
    id: 'quicknote',
    name: 'QuickNote',
    nameZh: '快捷笔记',
    icon: '📝',
    enabled: true,
    config: {
      autoSave: true,
      maxNotes: 100
    }
  },
  {
    id: 'idcard',
    name: 'IdCard',
    nameZh: '身份证工具',
    icon: '🎫',
    enabled: true,
    config: {
      validateAge: true
    }
  },
  {
    id: 'time',
    name: 'TimeConverter',
    nameZh: '时间转换',
    icon: '⏰',
    enabled: true,
    config: {
      defaultFormat: 'timestamp',
      showTimezone: true
    }
  },
  {
    id: 'jsonformatter',
    name: 'JsonFormatter',
    nameZh: 'JSON格式化',
    icon: '📝',
    enabled: true,
    config: {
      indentSize: 2,
      sortKeys: false
    }
  }
])

const selectedMenu = ref('plugins')
const selectedPlugin = ref<PluginConfig | null>(null)

const systemConfig = ref({
  theme: 'dark',
  language: 'zh-CN',
  autoStart: false,
  globalShortcut: 'Super+S'
})

const selectMenu = (menuId: string) => {
  selectedMenu.value = menuId
  selectedPlugin.value = null
}

const selectPlugin = (plugin: PluginConfig) => {
  selectedPlugin.value = plugin
}

const saveSettings = () => {
  const allSettings = {
    system: systemConfig.value,
    plugins: pluginConfigs.value
  }
  emit('saveSettings', allSettings)
  emit('close')
}

const initializeSettings = () => {
  if (props.initialSettings) {
    const settings = props.initialSettings

    if (settings.system) {
      Object.assign(systemConfig.value, settings.system)
    }

    if (settings.plugins && Array.isArray(settings.plugins)) {
      settings.plugins.forEach((savedPlugin: any) => {
        const plugin = pluginConfigs.value.find(p => p.id === savedPlugin.id)
        if (plugin) {
          plugin.enabled = savedPlugin.enabled
          if (savedPlugin.config) {
            Object.assign(plugin.config, savedPlugin.config)
          }
        }
      })
    }
  }
}

onMounted(() => {
  initializeSettings()
})

watch(() => props.initialSettings, () => {
  initializeSettings()
}, { deep: true })
</script>

<template>
  <div class="settings-overlay" @click.self="emit('close')">
    <div class="settings-container">
      <div class="settings-header">
        <h2>设置</h2>
        <button class="close-btn" @click="emit('close')">✕</button>
      </div>

      <div class="settings-content">
        <div class="settings-menu">
          <div v-for="item in menuItems" :key="item.id" :class="['menu-item', { active: selectedMenu === item.id }]"
            @click="selectMenu(item.id)">
            <span class="menu-icon">{{ item.icon }}</span>
            <span class="menu-label">{{ item.name }}</span>
          </div>
        </div>

        <div class="settings-panel">
          <div v-if="selectedMenu === 'system'" class="panel-content">
            <h3>系统设置</h3>

            <div class="setting-group">
              <label>主题</label>
              <select v-model="systemConfig.theme">
                <option value="dark">深色</option>
                <option value="light">浅色</option>
                <option value="auto">自动</option>
              </select>
            </div>

            <div class="setting-group">
              <label>语言</label>
              <select v-model="systemConfig.language">
                <option value="zh-CN">简体中文</option>
                <option value="en-US">English</option>
              </select>
            </div>

            <div class="setting-group">
              <label>
                <input type="checkbox" v-model="systemConfig.autoStart" />
                开机自启动
              </label>
            </div>

            <div class="setting-group">
              <label>全局快捷键</label>
              <input type="text" v-model="systemConfig.globalShortcut" readonly />
            </div>
          </div>

          <div v-else-if="selectedMenu === 'plugins'" class="panel-content">
            <div v-if="!selectedPlugin" class="plugin-list">
              <h3>插件设置</h3>
              <div class="plugin-item" v-for="plugin in pluginConfigs" :key="plugin.id">
                <div class="plugin-info" @click="selectPlugin(plugin)">
                  <span class="plugin-icon">{{ plugin.icon }}</span>
                  <span class="plugin-name">{{ plugin.nameZh }}</span>
                </div>
                <label class="switch">
                  <input type="checkbox" v-model="plugin.enabled" />
                  <span class="slider"></span>
                </label>
              </div>
            </div>

            <div v-else class="plugin-config">
              <button class="back-btn" @click="selectedPlugin = null">← 返回</button>
              <h3>{{ selectedPlugin.icon }} {{ selectedPlugin.nameZh }}</h3>

              <div v-if="selectedPlugin.id === 'encoder'" class="config-form">
                <div class="setting-group">
                  <label>默认编码类型</label>
                  <select v-model="selectedPlugin.config.defaultEncodeType">
                    <option value="md5">MD5</option>
                    <option value="sha256">SHA-256</option>
                    <option value="base64">Base64</option>
                    <option value="url">URL编码</option>
                  </select>
                </div>

                <div class="setting-group">
                  <label>启用的编码类型</label>
                  <div class="checkbox-group">
                    <label><input type="checkbox" v-model="selectedPlugin.config.enabledTypes" value="md5" />
                      MD5</label>
                    <label><input type="checkbox" v-model="selectedPlugin.config.enabledTypes" value="sha1" />
                      SHA-1</label>
                    <label><input type="checkbox" v-model="selectedPlugin.config.enabledTypes" value="sha256" />
                      SHA-256</label>
                    <label><input type="checkbox" v-model="selectedPlugin.config.enabledTypes" value="sha512" />
                      SHA-512</label>
                    <label><input type="checkbox" v-model="selectedPlugin.config.enabledTypes" value="base64" />
                      Base64</label>
                    <label><input type="checkbox" v-model="selectedPlugin.config.enabledTypes" value="url" />
                      URL编码</label>
                    <label><input type="checkbox" v-model="selectedPlugin.config.enabledTypes" value="hex" />
                      十六进制</label>
                  </div>
                </div>
              </div>

              <div v-else-if="selectedPlugin.id === 'calculator'" class="config-form">
                <div class="setting-group">
                  <label>计算精度</label>
                  <input type="number" v-model.number="selectedPlugin.config.precision" min="0" max="16" />
                </div>

                <div class="setting-group">
                  <label>
                    <input type="checkbox" v-model="selectedPlugin.config.autoCalculate" />
                    自动计算
                  </label>
                </div>
              </div>

              <div v-else-if="selectedPlugin.id === 'colorpicker'" class="config-form">
                <div class="setting-group">
                  <label>默认格式</label>
                  <select v-model="selectedPlugin.config.defaultFormat">
                    <option value="hex">HEX</option>
                    <option value="rgb">RGB</option>
                    <option value="hsl">HSL</option>
                  </select>
                </div>

                <div class="setting-group">
                  <label>
                    <input type="checkbox" v-model="selectedPlugin.config.showAlpha" />
                    显示透明度
                  </label>
                </div>
              </div>

              <div v-else class="config-form">
                <p class="no-config">该插件暂无配置选项</p>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div class="settings-footer">
        <button class="save-btn" @click="saveSettings">保存设置</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.settings-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.6);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
}

.settings-container {
  width: 90%;
  max-width: 900px;
  height: 80%;
  max-height: 700px;
  background: rgba(30, 30, 35, 0.98);
  border-radius: 16px;
  border: 1px solid rgba(255, 255, 255, 0.1);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.settings-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 24px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.settings-header h2 {
  color: #fff;
  font-size: 20px;
  font-weight: 600;
  margin: 0;
}

.close-btn {
  background: transparent;
  border: none;
  color: rgba(255, 255, 255, 0.6);
  font-size: 20px;
  cursor: pointer;
  padding: 4px 8px;
  border-radius: 4px;
  transition: all 0.2s;
}

.close-btn:hover {
  background: rgba(255, 255, 255, 0.1);
  color: #fff;
}

.settings-content {
  flex: 1;
  display: flex;
  overflow: hidden;
}

.settings-menu {
  width: 200px;
  background: rgba(0, 0, 0, 0.2);
  border-right: 1px solid rgba(255, 255, 255, 0.1);
  padding: 16px;
}

.menu-item {
  display: flex;
  align-items: center;
  padding: 12px 16px;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
  color: rgba(255, 255, 255, 0.7);
  margin-bottom: 4px;
}

.menu-item:hover {
  background: rgba(255, 255, 255, 0.05);
  color: #fff;
}

.menu-item.active {
  background: rgba(100, 150, 255, 0.2);
  color: #fff;
}

.menu-icon {
  font-size: 18px;
  margin-right: 12px;
}

.menu-label {
  font-size: 14px;
}

.settings-panel {
  flex: 1;
  padding: 24px;
  overflow-y: auto;
}

.panel-content h3 {
  color: #fff;
  font-size: 18px;
  font-weight: 600;
  margin: 0 0 24px 0;
}

.plugin-list .plugin-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  background: rgba(255, 255, 255, 0.05);
  border-radius: 8px;
  margin-bottom: 8px;
  cursor: pointer;
  transition: all 0.2s;
}

.plugin-item:hover {
  background: rgba(255, 255, 255, 0.08);
}

.plugin-info {
  display: flex;
  align-items: center;
}

.plugin-icon {
  font-size: 24px;
  margin-right: 12px;
}

.plugin-name {
  color: #fff;
  font-size: 14px;
}

.switch {
  position: relative;
  display: inline-block;
  width: 48px;
  height: 24px;
}

.switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

.slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(255, 255, 255, 0.2);
  border-radius: 24px;
  transition: 0.3s;
}

.slider:before {
  position: absolute;
  content: "";
  height: 18px;
  width: 18px;
  left: 3px;
  bottom: 3px;
  background: white;
  border-radius: 50%;
  transition: 0.3s;
}

input:checked+.slider {
  background: rgba(100, 150, 255, 0.6);
}

input:checked+.slider:before {
  transform: translateX(24px);
}

.plugin-config {
  max-width: 600px;
}

.plugin-config h3 {
  margin: 16px 0 24px 0;
}

.back-btn {
  background: rgba(255, 255, 255, 0.1);
  border: none;
  color: #fff;
  padding: 8px 16px;
  border-radius: 8px;
  cursor: pointer;
  font-size: 14px;
  margin-bottom: 16px;
  transition: background 0.2s;
}

.back-btn:hover {
  background: rgba(255, 255, 255, 0.2);
}

.setting-group {
  margin-bottom: 20px;
}

.setting-group>label {
  display: block;
  color: rgba(255, 255, 255, 0.8);
  font-size: 14px;
  margin-bottom: 8px;
}

.setting-group input[type="text"],
.setting-group input[type="number"],
.setting-group select {
  width: 100%;
  padding: 10px 12px;
  background: rgba(255, 255, 255, 0.08);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 8px;
  color: #fff;
  font-size: 14px;
  outline: none;
  transition: all 0.2s;
}

.setting-group input:focus,
.setting-group select:focus {
  border-color: rgba(100, 150, 255, 0.5);
  background: rgba(255, 255, 255, 0.1);
}

.setting-group input[type="checkbox"] {
  margin-right: 8px;
  cursor: pointer;
}

.checkbox-group {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
  gap: 12px;
}

.checkbox-group label {
  display: flex;
  align-items: center;
  color: rgba(255, 255, 255, 0.7);
  font-size: 14px;
  cursor: pointer;
  padding: 8px 12px;
  background: rgba(255, 255, 255, 0.05);
  border-radius: 6px;
  transition: all 0.2s;
}

.checkbox-group label:hover {
  background: rgba(255, 255, 255, 0.1);
}

.checkbox-group input[type="checkbox"] {
  margin-right: 8px;
}

.no-config {
  color: rgba(255, 255, 255, 0.5);
  text-align: center;
  padding: 40px;
  font-size: 14px;
}

.settings-footer {
  padding: 16px 24px;
  border-top: 1px solid rgba(255, 255, 255, 0.1);
  display: flex;
  justify-content: flex-end;
}

.save-btn {
  background: rgba(100, 150, 255, 0.8);
  border: none;
  color: #fff;
  padding: 10px 24px;
  border-radius: 8px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: all 0.2s;
}

.save-btn:hover {
  background: rgba(100, 150, 255, 1);
}
</style>
<template>
  <div class="json-formatter">
    <div class="header">
      <h2>JSON 格式化工具</h2>
      <div class="btn-group">
        <button class="btn format" @click="formatJson">🔧 格式化</button>
        <button class="btn minify" @click="minifyJson">📦 反格式化</button>
        <button class="btn clear" @click="clearInput">🗑️ 清空</button>
        <button class="btn copy" @click="copyInput">📋 复制</button>
      </div>
    </div>

    <div class="search-section">
      <input v-model="searchQuery" type="text" placeholder="🔍 搜索键名或值..." class="search-input"
        @input="filterStructure" />
      <div class="search-info" v-if="searchQuery">
        找到 {{ filteredStructure.length }} 个结果
      </div>
    </div>

    <div class="content">
      <div class="input-section">
        <h3>JSON 数据</h3>
        <textarea ref="inputRef" v-model="inputJson" placeholder="在此输入 JSON 数据..." class="json-input"
          @input="validateJson"></textarea>
        <div v-if="errorMessage" class="error-message">{{ errorMessage }}</div>
        <div v-else-if="isValidJson" class="success-message">✓ JSON 格式正确</div>
      </div>

      <div class="output-section">
        <h3>JSON 结构</h3>
        <div class="json-structure" v-if="filteredStructure.length > 0">
          <div class="structure-item" v-for="(item, index) in filteredStructure" :key="index"
            :class="{ 'highlight': item.isMatch }">
            <div class="structure-key" v-html="highlightText(item.key, searchQuery)"></div>
            <div class="structure-type">{{ item.type }}</div>
            <div class="structure-value" v-html="highlightText(item.value, searchQuery)"></div>
          </div>
        </div>
        <div v-else-if="searchQuery && jsonStructure.length > 0" class="no-results">
          未找到匹配的结果
        </div>
        <div v-else class="no-structure">请输入有效的 JSON 数据</div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const inputJson = ref('')
const errorMessage = ref('')
const isValidJson = ref(false)
const jsonStructure = ref<any[]>([])
const filteredStructure = ref<any[]>([])
const searchQuery = ref('')
const inputRef = ref<HTMLTextAreaElement | null>(null)

const incrementUseCount = async () => {
  try {
    await invoke('increment_use_count', {
      pluginId: 'JsonFormatter'
    })
  } catch (error) {
    console.error('Error incrementing use count:', error)
  }
}

const validateJson = () => {
  if (!inputJson.value.trim()) {
    errorMessage.value = ''
    isValidJson.value = false
    jsonStructure.value = []
    filteredStructure.value = []
    return
  }

  try {
    const parsed = JSON.parse(inputJson.value)
    isValidJson.value = true
    errorMessage.value = ''
    generateStructure(parsed)
    filterStructure()
  } catch (e) {
    isValidJson.value = false
    errorMessage.value = '❌ JSON 格式错误: ' + (e as Error).message
    jsonStructure.value = []
    filteredStructure.value = []
  }
}

const generateStructure = (obj: any, parentKey = '') => {
  const structure: any[] = []

  if (typeof obj === 'object' && obj !== null) {
    if (Array.isArray(obj)) {
      structure.push({
        key: parentKey || 'root',
        type: 'Array',
        value: `Array[${obj.length}]`
      })

      obj.forEach((item, index) => {
        structure.push(...generateStructure(item, `${parentKey || 'root'}[${index}]`))
      })
    } else {
      structure.push({
        key: parentKey || 'root',
        type: 'Object',
        value: `Object{${Object.keys(obj).length}}`
      })

      Object.entries(obj).forEach(([key, value]) => {
        structure.push(...generateStructure(value, `${parentKey ? parentKey + '.' : ''}${key}`))
      })
    }
  } else {
    structure.push({
      key: parentKey || 'root',
      type: typeof obj,
      value: typeof obj === 'string' ? `"${obj}"` : String(obj)
    })
  }

  jsonStructure.value = structure
  return structure
}

const filterStructure = () => {
  if (!searchQuery.value.trim()) {
    filteredStructure.value = jsonStructure.value.map(item => ({ ...item, isMatch: false }))
    return
  }

  const query = searchQuery.value.toLowerCase()
  filteredStructure.value = jsonStructure.value
    .map(item => ({
      ...item,
      isMatch: item.key.toLowerCase().includes(query) ||
        item.value.toLowerCase().includes(query) ||
        item.type.toLowerCase().includes(query)
    }))
    .filter(item => item.isMatch)
}

const highlightText = (text: string, query: string) => {
  if (!query || !text) return text

  const regex = new RegExp(`(${query.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')})`, 'gi')
  return text.replace(regex, '<span class="highlight-text">$1</span>')
}

const formatJson = async () => {
  if (!inputJson.value.trim()) {
    errorMessage.value = '请输入 JSON 数据'
    return
  }

  try {
    const result = await invoke<string>('format_json', {
      json: inputJson.value
    })
    inputJson.value = result
    validateJson()
  } catch (e) {
    errorMessage.value = '格式化失败: ' + (e as Error).message
  }
}

const minifyJson = async () => {
  if (!inputJson.value.trim()) {
    errorMessage.value = '请输入 JSON 数据'
    return
  }

  try {
    const result = await invoke<string>('minify_json', {
      json: inputJson.value
    })
    inputJson.value = result
    validateJson()
  } catch (e) {
    errorMessage.value = '反格式化失败: ' + (e as Error).message
  }
}

const clearInput = () => {
  inputJson.value = ''
  errorMessage.value = ''
  isValidJson.value = false
  jsonStructure.value = []
  filteredStructure.value = []
  searchQuery.value = ''
  inputRef.value?.focus()
}

const copyInput = async () => {
  if (inputJson.value) {
    try {
      await navigator.clipboard.writeText(inputJson.value)
    } catch (e) {
      console.error('复制失败:', e)
    }
  }
}

onMounted(() => {
  incrementUseCount()
  inputRef.value?.focus()
})
</script>

<style scoped>
.json-formatter {
  flex: 1;
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 8px;
  overflow: hidden;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding-bottom: 8px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.header h2 {
  color: white;
  font-size: 18px;
  font-weight: 500;
  margin: 0;
}

.btn-group {
  display: flex;
  gap: 8px;
}

.btn {
  padding: 8px 16px;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: all 0.2s ease;
}

.btn.format {
  background: #4CAF50;
  color: white;
}

.btn.minify {
  background: #2196F3;
  color: white;
}

.btn.clear {
  background: #f44336;
  color: white;
}

.btn.copy {
  background: #9C27B0;
  color: white;
}

.btn:hover {
  opacity: 0.8;
  transform: translateY(-1px);
}

.search-section {
  display: flex;
  flex-direction: column;
  gap: 2px;
  padding: 4px;
  background: rgba(255, 255, 255, 0.05);
  border-radius: 8px;
  height: 32px;
  min-height: 32px;
}

.search-input {
  width: 100%;
  padding: 4px 10px;
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 4px;
  background: rgba(255, 255, 255, 0.1);
  color: white;
  font-size: 12px;
  transition: all 0.2s ease;
}

.search-input:focus {
  outline: none;
  border-color: rgba(100, 150, 255, 0.5);
  background: rgba(255, 255, 255, 0.15);
}

.search-input::placeholder {
  color: rgba(255, 255, 255, 0.4);
}

.search-info {
  font-size: 10px;
  color: rgba(255, 255, 255, 0.4);
  text-align: right;
}

.content {
  display: flex;
  gap: 16px;
  flex: 1;
  min-height: 0;
}

.input-section,
.output-section {
  display: flex;
  flex-direction: column;
  gap: 8px;
  flex: 1;
  min-width: 0;
}

.input-section h3,
.output-section h3 {
  color: rgba(255, 255, 255, 0.8);
  font-size: 14px;
  font-weight: 500;
  margin: 0;
}

.json-input {
  flex: 1;
  min-height: 0;
  padding: 12px;
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 8px;
  background: rgba(255, 255, 255, 0.1);
  color: white;
  font-size: 14px;
  font-family: monospace;
  resize: vertical;
  overflow-y: auto;
}

.json-input:focus {
  outline: none;
  border-color: rgba(100, 150, 255, 0.5);
}

.json-structure {
  flex: 1;
  min-height: 0;
  padding: 12px;
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 8px;
  background: rgba(255, 255, 255, 0.1);
  overflow-y: auto;
}

.structure-item {
  display: grid;
  grid-template-columns: 1fr auto auto;
  gap: 8px;
  padding: 6px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.05);
}

.structure-item:last-child {
  border-bottom: none;
}

.structure-item.highlight {
  background: rgba(100, 150, 255, 0.1);
  border-left: 3px solid rgba(100, 150, 255, 0.5);
}

.structure-key {
  color: #82b1ff;
  font-family: monospace;
  font-size: 12px;
  word-break: break-all;
}

.structure-type {
  color: #ffca28;
  font-size: 11px;
  background: rgba(255, 202, 40, 0.1);
  padding: 2px 6px;
  border-radius: 4px;
}

.structure-value {
  color: #a5e844;
  font-family: monospace;
  font-size: 12px;
  word-break: break-all;
}

.highlight-text {
  background: rgba(255, 235, 59, 0.3);
  color: #ffeb3b;
  padding: 0 2px;
  border-radius: 2px;
  font-weight: bold;
}

.error-message {
  color: #ff5252;
  font-size: 12px;
  padding: 8px;
  background: rgba(255, 82, 82, 0.1);
  border-radius: 4px;
  border-left: 3px solid #ff5252;
}

.success-message {
  color: #4caf50;
  font-size: 12px;
  padding: 8px;
  background: rgba(76, 175, 80, 0.1);
  border-radius: 4px;
  border-left: 3px solid #4caf50;
}

.no-structure,
.no-results {
  flex: 1;
  min-height: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  color: rgba(255, 255, 255, 0.4);
  font-size: 14px;
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 8px;
}

@media (max-width: 768px) {
  .content {
    flex-direction: column;
  }

  .json-input,
  .json-structure,
  .no-structure,
  .no-results {
    min-height: 200px;
  }

  .btn-group {
    flex-wrap: wrap;
  }
}
</style>
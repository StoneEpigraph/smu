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
    
    <div class="content">
      <div class="input-section">
        <h3>JSON 数据</h3>
        <textarea
          ref="inputRef"
          v-model="inputJson"
          placeholder="在此输入 JSON 数据..."
          class="json-input"
          @input="validateJson"
        ></textarea>
        <div v-if="errorMessage" class="error-message">{{ errorMessage }}</div>
        <div v-else-if="isValidJson" class="success-message">✓ JSON 格式正确</div>
      </div>
      
      <div class="output-section">
        <h3>JSON 结构</h3>
        <div class="json-structure" v-if="jsonStructure">
          <div class="structure-item" v-for="(item, index) in jsonStructure" :key="index">
            <div class="structure-key">{{ item.key }}</div>
            <div class="structure-type">{{ item.type }}</div>
            <div class="structure-value">{{ item.value }}</div>
          </div>
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
    return
  }
  
  try {
    const parsed = JSON.parse(inputJson.value)
    isValidJson.value = true
    errorMessage.value = ''
    generateStructure(parsed)
  } catch (e) {
    isValidJson.value = false
    errorMessage.value = '❌ JSON 格式错误: ' + (e as Error).message
    jsonStructure.value = []
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
  gap: 16px;
  overflow-y: auto;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding-bottom: 12px;
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

.content {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
  flex: 1;
}

.input-section,
.output-section {
  display: flex;
  flex-direction: column;
  gap: 8px;
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
  padding: 12px;
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 8px;
  background: rgba(255, 255, 255, 0.1);
  color: white;
  font-size: 14px;
  font-family: monospace;
  resize: none;
  min-height: 300px;
}

.json-input:focus {
  outline: none;
  border-color: rgba(100, 150, 255, 0.5);
}

.json-structure {
  flex: 1;
  padding: 12px;
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 8px;
  background: rgba(255, 255, 255, 0.1);
  overflow-y: auto;
  min-height: 300px;
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

.no-structure {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  color: rgba(255, 255, 255, 0.4);
  font-size: 14px;
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 8px;
  min-height: 300px;
}



@media (max-width: 768px) {
  .content {
    grid-template-columns: 1fr;
  }
  
  .json-input,
  .json-structure,
  .no-structure {
    min-height: 200px;
  }
  
  .btn-group {
    flex-wrap: wrap;
  }
}
</style>
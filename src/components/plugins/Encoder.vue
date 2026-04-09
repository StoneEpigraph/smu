<script setup lang="ts">
import { ref, watch, onMounted, nextTick } from 'vue'
import { invoke } from '@tauri-apps/api/core'

interface EncodeType {
  id: string
  name: string
  label: string
  isDecode?: boolean
}

const props = defineProps<{
  initialInput?: string
  config?: any
}>()

const inputRef = ref<HTMLTextAreaElement | null>(null)
const inputText = ref('')

const encodeTypes: EncodeType[] = [
  { id: 'md5', name: 'MD5', label: 'MD5' },
  { id: 'md5_decode', name: 'MD5Decode', label: 'MD5解码', isDecode: true },
  { id: 'sha1', name: 'SHA1', label: 'SHA-1' },
  { id: 'sha256', name: 'SHA256', label: 'SHA-256' },
  { id: 'sha512', name: 'SHA512', label: 'SHA-512' },
  { id: 'base64', name: 'Base64', label: 'Base64' },
  { id: 'base64_decode', name: 'Base64Decode', label: 'Base64解码', isDecode: true },
  { id: 'url', name: 'URL', label: 'URL编码' },
  { id: 'url_decode', name: 'URLDecode', label: 'URL解码', isDecode: true },
  { id: 'unicode', name: 'Unicode', label: 'Unicode' },
  { id: 'html', name: 'HTML', label: 'HTML实体' },
  { id: 'hex', name: 'Hex', label: '十六进制' },
  { id: 'hex_decode', name: 'HexDecode', label: 'Hex解码', isDecode: true },
  { id: 'rot13', name: 'ROT13', label: 'ROT13' }
]

const selectedTypes = ref<string[]>(['md5', 'base64'])
const results = ref<{ id: string; label: string; value: string }[]>([])
const isLoading = ref(false)

const toggleType = (id: string) => {
  const index = selectedTypes.value.indexOf(id)
  if (index > -1) {
    selectedTypes.value.splice(index, 1)
  } else {
    selectedTypes.value.push(id)
  }
  computeResults()
}

const isSelected = (id: string) => selectedTypes.value.includes(id)

const initializeConfig = () => {
  if (props.config) {
    if (props.config.enabledTypes && Array.isArray(props.config.enabledTypes)) {
      selectedTypes.value = [...props.config.enabledTypes]
    }
    if (props.config.defaultEncodeType) {
      if (!selectedTypes.value.includes(props.config.defaultEncodeType)) {
        selectedTypes.value.push(props.config.defaultEncodeType)
      }
    }
  }
}

const rot13 = (text: string): string => {
  return text.replace(/[a-zA-Z]/g, c => {
    const base = c <= 'Z' ? 65 : 97
    return String.fromCharCode((c.charCodeAt(0) - base + 13) % 26 + base)
  })
}

const htmlEncode = (text: string): string => {
  return text.replace(/[&<>"']/g, m => ({
    '&': '&amp;',
    '<': '&lt;',
    '>': '&gt;',
    '"': '&quot;',
    "'": '&#39;'
  }[m] || m))
}

const unicodeEncode = (text: string): string => {
  return text.split('').map(c => '\\u' + c.charCodeAt(0).toString(16).padStart(4, '0')).join('')
}

const computeResults = async () => {
  results.value = []
  const text = inputText.value

  if (!text || selectedTypes.value.length === 0) {
    return
  }

  isLoading.value = true

  for (const type of selectedTypes.value) {
    try {
      let value = ''
      switch (type) {
        case 'md5':
          value = await invoke<string>('encode_md5', { input: text })
          break
        case 'md5_decode':
          const md5Result = await invoke<string | null>('decode_md5', { hash: text })
          value = md5Result || '未找到对应明文'
          break
        case 'sha1':
          value = await invoke<string>('encode_sha1', { input: text })
          break
        case 'sha256':
          value = await invoke<string>('encode_sha256', { input: text })
          break
        case 'sha512':
          value = await invoke<string>('encode_sha512', { input: text })
          break
        case 'base64':
          value = await invoke<string>('encode_base64', { input: text })
          break
        case 'base64_decode':
          value = await invoke<string>('decode_base64', { input: text })
          break
        case 'url':
          value = await invoke<string>('encode_url', { input: text })
          break
        case 'url_decode':
          value = await invoke<string>('decode_url', { input: text })
          break
        case 'hex':
          value = await invoke<string>('encode_hex', { input: text })
          break
        case 'hex_decode':
          value = await invoke<string>('decode_hex', { input: text })
          break
        case 'unicode':
          value = unicodeEncode(text)
          break
        case 'html':
          value = htmlEncode(text)
          break
        case 'rot13':
          value = rot13(text)
          break
      }
      const typeInfo = encodeTypes.find(t => t.id === type)
      results.value.push({ id: type, label: typeInfo?.label || type, value })
    } catch (e: any) {
      results.value.push({
        id: type,
        label: encodeTypes.find(t => t.id === type)?.label || type,
        value: 'Error: ' + (e.toString() || '未知错误')
      })
    }
  }

  isLoading.value = false
}

const copyResult = async (value: string) => {
  try {
    await navigator.clipboard.writeText(value)
  } catch (e) {
    console.log('Copy failed:', e)
  }
}

watch(() => props.initialInput, (newVal) => {
  if (newVal) {
    inputText.value = newVal
    computeResults()
  }
})

watch(inputText, () => {
  computeResults()
})

watch(() => props.config, () => {
  initializeConfig()
  computeResults()
}, { deep: true })

onMounted(async () => {
  initializeConfig()
  if (props.initialInput) {
    inputText.value = props.initialInput
  }
  await nextTick()
  inputRef.value?.focus()
  if (inputText.value) {
    computeResults()
  }
})
</script>

<template>
  <div class="encoder">
    <div class="input-section">
      <textarea ref="inputRef" v-model="inputText" placeholder="输入要编码/解码的内容..." class="input-textarea"></textarea>
    </div>

    <div class="types-section">
      <div class="types-grid">
        <label v-for="type in encodeTypes" :key="type.id" :class="['type-item', { selected: isSelected(type.id) }]">
          <input type="checkbox" :checked="isSelected(type.id)" @change="toggleType(type.id)" />
          <span>{{ type.label }}</span>
        </label>
      </div>
    </div>

    <div class="results-section">
      <div v-for="result in results" :key="result.id" class="result-item">
        <div class="result-header">
          <span class="result-label">{{ result.label }}</span>
          <button class="copy-btn" @click="copyResult(result.value)">📋 复制</button>
        </div>
        <div class="result-value">{{ result.value }}</div>
      </div>
      <div v-if="results.length === 0" class="no-results">
        选择编码类型并输入内容
      </div>
    </div>
  </div>
</template>

<style scoped>
.encoder {
  flex: 1;
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 12px;
  overflow-y: auto;
}

.input-section {
  flex-shrink: 0;
}

.input-textarea {
  width: 100%;
  height: 80px;
  background: rgba(255, 255, 255, 0.1);
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 10px;
  padding: 12px;
  color: #fff;
  font-size: 14px;
  resize: none;
  outline: none;
  font-family: monospace;
}

.input-textarea:focus {
  border-color: rgba(100, 150, 255, 0.5);
}

.input-textarea::placeholder {
  color: rgba(255, 255, 255, 0.4);
}

.types-section {
  flex-shrink: 0;
}

.types-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 8px;
}

.type-item {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 10px;
  background: rgba(255, 255, 255, 0.05);
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.15s;
  font-size: 12px;
  color: rgba(255, 255, 255, 0.6);
}

.type-item:hover {
  background: rgba(255, 255, 255, 0.1);
}

.type-item.selected {
  background: rgba(100, 150, 255, 0.3);
  color: #fff;
}

.type-item input {
  display: none;
}

.results-section {
  flex: 1;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.result-item {
  background: rgba(255, 255, 255, 0.05);
  border-radius: 10px;
  padding: 10px 12px;
}

.result-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 6px;
}

.result-label {
  color: rgba(255, 255, 255, 0.6);
  font-size: 12px;
  font-weight: 500;
}

.copy-btn {
  background: rgba(255, 255, 255, 0.1);
  border: none;
  color: #fff;
  padding: 4px 8px;
  border-radius: 6px;
  cursor: pointer;
  font-size: 11px;
  transition: background 0.15s;
}

.copy-btn:hover {
  background: rgba(255, 255, 255, 0.2);
}

.result-value {
  color: #fff;
  font-size: 12px;
  font-family: monospace;
  word-break: break-all;
  line-height: 1.4;
}

.no-results {
  text-align: center;
  color: rgba(255, 255, 255, 0.4);
  padding: 20px;
}
</style>
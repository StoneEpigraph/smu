<template>
  <div class="time-converter">
    <div class="header">
      <span class="current-time" v-if="formats.dateTime">{{ formats.dateTime }}</span>
      <div class="btn-group">
        <button class="now-btn" @click="refreshNow">🔄 当前时间</button>
        <button class="clear-time-btn" @click="clearTime">⚡ 清空时分秒</button>
      </div>
    </div>
    <div class="time-inputs">
      <div class="input-group">
        <label>日期:</label>
        <select v-model="selectedYear" @change="updateDate">
          <option v-for="y in 100" :key="y" :value="1950 + y">{{ 1950 + y }}</option>
        </select>
        <span class="separator">-</span>
        <select v-model="selectedMonth" @change="updateDate">
          <option v-for="m in 12" :key="m" :value="m">{{ m.toString().padStart(2, '0') }}</option>
        </select>
        <span class="separator">-</span>
        <select v-model="selectedDay" @change="updateDate">
          <option v-for="d in 31" :key="d" :value="d">{{ d.toString().padStart(2, '0') }}</option>
        </select>
      </div>
      <div class="input-group">
        <label>时间:</label>
        <select v-model="selectedHour" @change="updateDate">
          <option v-for="h in 23" :key="h" :value="h">{{ h.toString().padStart(2, '0') }}</option>
        </select>
        <span class="separator">:</span>
        <select v-model="selectedMinute" @change="updateDate">
          <option v-for="m in 59" :key="m" :value="m">{{ m.toString().padStart(2, '0') }}</option>
        </select>
        <span class="separator">:</span>
        <select v-model="selectedSecond" @change="updateDate">
          <option v-for="s in 59" :key="s" :value="s">{{ s.toString().padStart(2, '0') }}</option>
        </select>
      </div>
    </div>
    <div class="outputs">
      <div class="output-item" v-for="(value, key) in formats" :key="key" @click="copyValue(value)">
        <div class="output-label">{{ getLabel(key) }}</div>
        <div class="output-value">{{ value }}</div>
      </div>
    </div>
    <div class="copy-instruction">
      💡 点击上方时间格式区域可复制对应值
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const incrementUseCount = async (toolName: string) => {
  try {
    await invoke('increment_use_count', {
      pluginId: toolName
    })
  } catch (error) {
    console.error('Error incrementing use count:', error)
  }
}

const selectedYear = ref(2024)
const selectedMonth = ref(1)
const selectedDay = ref(1)
const selectedHour = ref(0)
const selectedMinute = ref(0)
const selectedSecond = ref(0)
const formats = ref({
  timestamp: '',
  timestampSec: '',
  dateTime: '',
  dateDash: '',
  dateSlash: '',
  time: '',
  iso: '',
  utc: '',
  chinese: '',
  currentTime: ''
})

const updateDate = () => {
  const date = new Date(selectedYear.value, selectedMonth.value - 1, selectedDay.value, selectedHour.value, selectedMinute.value, selectedSecond.value, 0)
  const now = new Date()
  
  formats.value = {
    timestamp: date.getTime().toString(),
    timestampSec: Math.floor(date.getTime() / 1000).toString(),
    dateTime: date.toLocaleString('zh-CN'),
    dateDash: date.toISOString().split('T')[0],
    dateSlash: `${date.getFullYear()}/${String(date.getMonth() + 1).padStart(2, '0')}/${String(date.getDate()).padStart(2, '0')}`,
    time: `${String(date.getHours()).padStart(2, '0')}:${String(date.getMinutes()).padStart(2, '0')}:${String(date.getSeconds()).padStart(2, '0')}`,
    iso: date.toISOString(),
    utc: date.toUTCString(),
    chinese: formatChineseDate(date),
    currentTime: now.toLocaleString('zh-CN')
  }
}

const refreshNow = () => {
  const now = new Date()
  selectedYear.value = now.getFullYear()
  selectedMonth.value = now.getMonth() + 1
  selectedDay.value = now.getDate()
  selectedHour.value = now.getHours()
  selectedMinute.value = now.getMinutes()
  selectedSecond.value = now.getSeconds()
  updateDate()
}

const clearTime = () => {
  selectedHour.value = 0
  selectedMinute.value = 0
  selectedSecond.value = 0
  updateDate()
}

const copyValue = (value: string) => {
  navigator.clipboard.writeText(value)
}

const formatChineseDate = (date: Date) => {
  const year = date.getFullYear()
  const month = date.getMonth() + 1
  const day = date.getDate()
  const hour = date.getHours()
  const minute = date.getMinutes()
  const second = date.getSeconds()
  
  return `${year}年${month}月${day}日 ${hour}时${minute}分${second}秒`
}

const getLabel = (key: string) => {
  const labels: Record<string, string> = {
    timestamp: '时间戳(毫秒)',
    timestampSec: '时间戳(秒)',
    dateTime: '日期时间',
    dateDash: '日期(-)',
    dateSlash: '日期(/)',
    time: '时间',
    iso: 'ISO 8601',
    utc: 'UTC时间',
    chinese: '中文格式',
    currentTime: '当前时间'
  }
  return labels[key] || key
}

onMounted(() => {
  refreshNow()
  incrementUseCount('TimeConverter')
})
</script>

<style scoped>
.time-converter {
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
  margin-bottom: 8px;
}

.current-time {
  font-size: 16px;
  font-weight: 500;
  color: rgba(255, 255, 255, 0.8);
}

.btn-group {
  display: flex;
  gap: 8px;
}

.now-btn, .clear-time-btn {
  padding: 8px 16px;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: all 0.2s ease;
}

.now-btn {
  background: rgba(255, 255, 255, 0.1);
  color: white;
}

.now-btn:hover {
  background: rgba(255, 255, 255, 0.2);
}

.clear-time-btn {
  background: #FF9800;
  color: white;
}

.clear-time-btn:hover {
  background: #F57C00;
}

.time-inputs {
  display: flex;
  align-items: center;
  gap: 20px;
  flex-wrap: wrap;
}

.input-group {
  display: flex;
  align-items: center;
  gap: 10px;
}

.input-group label {
  width: 60px;
  font-weight: 500;
  color: rgba(255, 255, 255, 0.8);
}

.input-group select {
  padding: 0 12px;
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 8px;
  font-size: 14px;
  background: rgba(255, 255, 255, 0.1);
  color: red;
  font-weight: bold;
  text-align: center;
  min-width: 80px;
  height: 36px;
  line-height: 34px;
  box-sizing: border-box;
}

.separator {
  font-size: 16px;
  font-weight: bold;
  color: rgba(255, 255, 255, 0.8);
}

.outputs {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 12px;
  flex: 1;
}

.output-item {
  background: rgba(255, 255, 255, 0.05);
  padding: 12px;
  border-radius: 10px;
  border: 1px solid rgba(255, 255, 255, 0.1);
  transition: all 0.2s ease;
}

.output-item:hover {
  background: rgba(255, 255, 255, 0.1);
  border-color: rgba(100, 150, 255, 0.3);
}

.copy-instruction {
  text-align: center;
  font-size: 12px;
  color: rgba(255, 255, 255, 0.6);
  padding: 12px;
  background: rgba(255, 255, 255, 0.05);
  border-radius: 8px;
  border: 1px solid rgba(255, 255, 255, 0.1);
  margin-top: 8px;
}

.output-label {
  font-size: 12px;
  color: rgba(255, 255, 255, 0.6);
  margin-bottom: 6px;
  font-weight: 500;
}

.output-value {
  font-size: 14px;
  font-weight: 500;
  color: white;
  margin-bottom: 8px;
  word-break: break-all;
  min-height: 20px;
  font-family: monospace;
}

.output-item {
  background: rgba(255, 255, 255, 0.05);
  padding: 12px;
  border-radius: 10px;
  border: 1px solid rgba(255, 255, 255, 0.1);
  transition: all 0.2s ease;
  cursor: pointer;
}

.output-item:hover {
  background: rgba(255, 255, 255, 0.1);
  border-color: rgba(100, 150, 255, 0.3);
}

@media (max-width: 768px) {
  .time-converter {
    padding: 12px;
  }
  
  .header {
    flex-direction: column;
    align-items: flex-start;
    gap: 8px;
  }
  
  .btn-group {
    align-self: flex-end;
  }
  
  .time-inputs {
    gap: 8px;
  }
  
  .input-group {
    flex-wrap: wrap;
  }
  
  .outputs {
    grid-template-columns: 1fr;
  }
}
</style>
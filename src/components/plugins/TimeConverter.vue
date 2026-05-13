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
    <div class="time-calc">
      <div class="input-group">
        <select v-model="calcOperation" @change="calculateResult">
          <option value="add">加上</option>
          <option value="subtract">减去</option>
        </select>
      </div>
      <div class="input-group">
        <select v-model.number="calcYears" @change="calculateResult">
          <option v-for="y in 50" :key="y" :value="y">{{ y.toString().padStart(2, '0') }}</option>
        </select>
        <span class="separator">-</span>
        <select v-model.number="calcMonths" @change="calculateResult">
          <option v-for="m in 12" :key="m" :value="m">{{ m.toString().padStart(2, '0') }}</option>
        </select>
        <span class="separator">-</span>
        <select v-model.number="calcDays" @change="calculateResult">
          <option v-for="d in 31" :key="d" :value="d">{{ d.toString().padStart(2, '0') }}</option>
        </select>
      </div>
      <div class="input-group">
        <select v-model.number="calcHours" @change="calculateResult">
          <option v-for="h in 23" :key="h" :value="h">{{ h.toString().padStart(2, '0') }}</option>
        </select>
        <span class="separator">:</span>
        <select v-model.number="calcMinutes" @change="calculateResult">
          <option v-for="m in 59" :key="m" :value="m">{{ m.toString().padStart(2, '0') }}</option>
        </select>
        <span class="separator">:</span>
        <select v-model.number="calcSeconds" @change="calculateResult">
          <option v-for="s in 59" :key="s" :value="s">{{ s.toString().padStart(2, '0') }}</option>
        </select>
        <button class="clear-calc-btn" @click="clearCalc">🗑️ 清空计算</button>
      </div>
    </div>
    <div class="outputs">
      <div class="output-section">
        <div class="section-title">📅 原始日期时间</div>
        <div class="output-grid">
          <div class="output-item" v-for="(value, key) in formats" :key="key" @click="copyValue(value)">
            <div class="output-label">{{ getLabel(key) }}</div>
            <div class="output-value">{{ value }}</div>
          </div>
        </div>
      </div>
      <div class="output-section" v-if="calcResultDate">
        <div class="section-title">📅 {{ calcOperation === 'add' ? '加上' : '减去' }}后的结果</div>
        <div class="output-grid">
          <div class="output-item" @click="copyValue(calcResultDate.getTime().toString())">
            <div class="output-label">时间戳(毫秒)</div>
            <div class="output-value">{{ calcResultDate.getTime() }}</div>
          </div>
          <div class="output-item" @click="copyValue(Math.floor(calcResultDate.getTime() / 1000).toString())">
            <div class="output-label">时间戳(秒)</div>
            <div class="output-value">{{ Math.floor(calcResultDate.getTime() / 1000) }}</div>
          </div>
          <div class="output-item" @click="copyValue(calcResultDate.toLocaleString('zh-CN'))">
            <div class="output-label">日期时间</div>
            <div class="output-value">{{ calcResultDate.toLocaleString('zh-CN') }}</div>
          </div>
          <div class="output-item" @click="copyValue(calcResultDate.toISOString().split('T')[0])">
            <div class="output-label">日期(-)</div>
            <div class="output-value">{{ calcResultDate.toISOString().split('T')[0] }}</div>
          </div>
          <div class="output-item"
            @click="copyValue(`${calcResultDate.getFullYear()}/${String(calcResultDate.getMonth() + 1).padStart(2, '0')}/${String(calcResultDate.getDate()).padStart(2, '0')}`)">
            <div class="output-label">日期(/)</div>
            <div class="output-value">{{ calcResultDate.getFullYear() }}/{{ String(calcResultDate.getMonth() +
              1).padStart(2, '0') }}/{{ String(calcResultDate.getDate()).padStart(2, '0') }}</div>
          </div>
          <div class="output-item"
            @click="copyValue(`${String(calcResultDate.getHours()).padStart(2, '0')}:${String(calcResultDate.getMinutes()).padStart(2, '0')}:${String(calcResultDate.getSeconds()).padStart(2, '0')}`)">
            <div class="output-label">时间</div>
            <div class="output-value">{{ String(calcResultDate.getHours()).padStart(2, '0') }}:{{
              String(calcResultDate.getMinutes()).padStart(2, '0') }}:{{ String(calcResultDate.getSeconds()).padStart(2,
                '0') }}</div>
          </div>
          <div class="output-item" @click="copyValue(calcResultDate.toISOString())">
            <div class="output-label">ISO 8601</div>
            <div class="output-value">{{ calcResultDate.toISOString() }}</div>
          </div>
          <div class="output-item" @click="copyValue(calcResultDate.toUTCString())">
            <div class="output-label">UTC时间</div>
            <div class="output-value">{{ calcResultDate.toUTCString() }}</div>
          </div>
          <div class="output-item" @click="copyValue(formatChineseDate(calcResultDate))">
            <div class="output-label">中文格式</div>
            <div class="output-value">{{ formatChineseDate(calcResultDate) }}</div>
          </div>
        </div>
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
const calcOperation = ref('add')
const calcYears = ref(0)
const calcMonths = ref(0)
const calcDays = ref(0)
const calcHours = ref(0)
const calcMinutes = ref(0)
const calcSeconds = ref(0)
const calcResultDate = ref<Date | null>(null)
const resultYear = ref(2024)
const resultMonth = ref(1)
const resultDay = ref(1)
const resultHour = ref(0)
const resultMinute = ref(0)
const resultSecond = ref(0)

const clearCalc = () => {
  calcOperation.value = 'add'
  calcYears.value = 0
  calcMonths.value = 0
  calcDays.value = 0
  calcHours.value = 0
  calcMinutes.value = 0
  calcSeconds.value = 0
  calcResultDate.value = null
  resultYear.value = 2024
  resultMonth.value = 1
  resultDay.value = 1
  resultHour.value = 0
  resultMinute.value = 0
  resultSecond.value = 0
}
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

  calculateResult()
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

const calculateResult = () => {
  const baseDate = new Date(selectedYear.value, selectedMonth.value - 1, selectedDay.value, selectedHour.value, selectedMinute.value, selectedSecond.value, 0)
  const result = new Date(baseDate)

  const years = calcYears.value || 0
  const months = calcMonths.value || 0
  const days = calcDays.value || 0
  const hours = calcHours.value || 0
  const minutes = calcMinutes.value || 0
  const seconds = calcSeconds.value || 0

  if (calcOperation.value === 'add') {
    result.setFullYear(result.getFullYear() + years)
    result.setMonth(result.getMonth() + months)
    result.setDate(result.getDate() + days)
    result.setHours(result.getHours() + hours)
    result.setMinutes(result.getMinutes() + minutes)
    result.setSeconds(result.getSeconds() + seconds)
  } else {
    result.setFullYear(result.getFullYear() - years)
    result.setMonth(result.getMonth() - months)
    result.setDate(result.getDate() - days)
    result.setHours(result.getHours() - hours)
    result.setMinutes(result.getMinutes() - minutes)
    result.setSeconds(result.getSeconds() - seconds)
  }

  calcResultDate.value = result
  resultYear.value = result.getFullYear()
  resultMonth.value = result.getMonth() + 1
  resultDay.value = result.getDate()
  resultHour.value = result.getHours()
  resultMinute.value = result.getMinutes()
  resultSecond.value = result.getSeconds()
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

.now-btn,
.clear-time-btn {
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

.time-calc {
  display: flex;
  align-items: center;
  gap: 20px;
  flex-wrap: wrap;
}

.time-calc .input-group {
  display: flex;
  align-items: center;
  gap: 10px;
}

.time-calc .input-group select {
  padding: 0 12px;
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 8px;
  font-size: 14px;
  background: rgba(255, 255, 255, 0.1);
  color: #4FC3F7;
  font-weight: bold;
  text-align: center;
  min-width: 80px;
  height: 36px;
  line-height: 34px;
  box-sizing: border-box;
}

.time-calc .input-group select option {
  color: white;
  background: #1a1a2e;
}

.clear-calc-btn {
  padding: 8px 16px;
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 8px;
  background: rgba(255, 255, 255, 0.1);
  color: rgba(255, 255, 255, 0.8);
  cursor: pointer;
  font-size: 14px;
  transition: all 0.2s ease;
  margin-left: 8px;
}

.clear-calc-btn:hover {
  background: rgba(255, 82, 82, 0.2);
  border-color: rgba(255, 82, 82, 0.5);
  color: #FF5252;
}

.outputs {
  display: flex;
  flex-direction: column;
  gap: 20px;
  flex: 1;
}

.output-section {
  background: rgba(255, 255, 255, 0.03);
  padding: 16px;
  border-radius: 12px;
  border: 1px solid rgba(255, 255, 255, 0.08);
}

.section-title {
  font-size: 14px;
  font-weight: 500;
  color: rgba(255, 255, 255, 0.8);
  margin-bottom: 12px;
  padding-bottom: 8px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.result-inputs {
  display: flex;
  align-items: center;
  gap: 20px;
  flex-wrap: wrap;
  margin-bottom: 16px;
  padding-bottom: 16px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.result-inputs .input-group {
  display: flex;
  align-items: center;
  gap: 10px;
}

.result-inputs .input-group label {
  width: 60px;
  font-weight: 500;
  color: rgba(255, 255, 255, 0.8);
}

.result-inputs .input-group select {
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

.result-inputs {
  display: flex;
  align-items: center;
  gap: 20px;
  flex-wrap: wrap;
  margin-bottom: 16px;
  padding-bottom: 16px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.result-inputs .input-group {
  display: flex;
  align-items: center;
  gap: 10px;
}

.result-inputs .input-group label {
  width: 60px;
  font-weight: 500;
  color: rgba(255, 255, 255, 0.8);
}

.result-inputs .input-group select {
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

.output-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 12px;
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
  word-break: break-all;
  min-height: 20px;
  font-family: monospace;
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
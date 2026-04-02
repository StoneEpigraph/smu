<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'

const props = defineProps<{
  initialInput?: string
}>()

const now = ref<Date | null>(null)
const copyMsg = ref('')

const years = computed(() => {
  const arr = []
  for (let y = 1970; y <= 2100; y++) arr.push(y)
  return arr
})

const months = computed(() => Array.from({length: 12}, (_, i) => i + 1))
const days = computed(() => {
  if (!now.value) return Array.from({length: 31}, (_, i) => i + 1)
  const d = new Date(now.value.getFullYear(), now.value.getMonth() + 1, 0).getDate()
  return Array.from({length: d}, (_, i) => i + 1)
})
const hours = computed(() => Array.from({length: 24}, (_, i) => i))
const minutes = computed(() => Array.from({length: 60}, (_, i) => i))
const seconds = computed(() => Array.from({length: 60}, (_, i) => i))

const selectedYear = ref(2024)
const selectedMonth = ref(1)
const selectedDay = ref(1)
const selectedHour = ref(0)
const selectedMinute = ref(0)
const selectedSecond = ref(0)

const formats = computed(() => {
  if (!now.value) return {}
  const d = now.value
  return {
    timestamp: Math.floor(d.getTime() / 1000).toString(),
    timestampMs: d.getTime().toString(),
    dateTime: formatDateTime(d),
    date: formatDate(d),
    time: formatTime(d),
    iso: d.toISOString(),
    utc: d.toUTCString(),
    cn: formatCN(d),
    unix: Math.floor(d.getTime() / 1000).toString(),
  }
})

const formatDateTime = (d: Date): string => {
  const y = d.getFullYear()
  const m = String(d.getMonth() + 1).padStart(2, '0')
  const dd = String(d.getDate()).padStart(2, '0')
  const hh = String(d.getHours()).padStart(2, '0')
  const mm = String(d.getMinutes()).padStart(2, '0')
  const ss = String(d.getSeconds()).padStart(2, '0')
  return `${y}-${m}-${dd} ${hh}:${mm}:${ss}`
}

const formatDate = (d: Date): string => `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}-${String(d.getDate()).padStart(2, '0')}`

const formatTime = (d: Date): string => `${String(d.getHours()).padStart(2, '0')}:${String(d.getMinutes()).padStart(2, '0')}:${String(d.getSeconds()).padStart(2, '0')}`

const formatCN = (d: Date): string => {
  const y = d.getFullYear()
  const m = d.getMonth() + 1
  const dd = d.getDate()
  const wd = ['星期日', '星期一', '星期二', '星期三', '星期四', '星期五', '星期六'][d.getDay()]
  return `${y}年${m}月${dd}日 ${wd}`
}

const updateDate = () => {
  if (selectedYear.value && selectedMonth.value && selectedDay.value) {
    now.value = new Date(
      selectedYear.value,
      selectedMonth.value - 1,
      selectedDay.value,
      selectedHour.value,
      selectedMinute.value,
      selectedSecond.value
    )
  }
}

const refreshNow = () => {
  const d = new Date()
  now.value = d
  selectedYear.value = d.getFullYear()
  selectedMonth.value = d.getMonth() + 1
  selectedDay.value = d.getDate()
  selectedHour.value = d.getHours()
  selectedMinute.value = d.getMinutes()
  selectedSecond.value = d.getSeconds()
}

const copyToClipboard = async (value: string, label: string) => {
  try {
    await navigator.clipboard.writeText(value)
    copyMsg.value = `✅ 已复制: ${label}`
    setTimeout(() => copyMsg.value = '', 2000)
  } catch (e) {
    copyMsg.value = '❌ 复制失败'
  }
}

watch(() => props.initialInput, (val) => {
  if (val) {
    const timestamp = parseInt(val)
    if (!isNaN(timestamp)) {
      const d = timestamp > 1e12 ? new Date(timestamp) : new Date(timestamp * 1000)
      now.value = d
      selectedYear.value = d.getFullYear()
      selectedMonth.value = d.getMonth() + 1
      selectedDay.value = d.getDate()
      selectedHour.value = d.getHours()
      selectedMinute.value = d.getMinutes()
      selectedSecond.value = d.getSeconds()
    }
  }
})

onMounted(() => {
  refreshNow()
})
</script>

<template>
  <div class="time-converter">
    <div class="header">
      <span class="current-time" v-if="formats.dateTime">{{ formats.dateTime }}</span>
      <button class="now-btn" @click="refreshNow">🔄 当前时间</button>
    </div>
    
    <div class="datetime-selectors">
      <div class="selector-group">
        <label>年</label>
        <select v-model="selectedYear" @change="updateDate">
          <option v-for="y in years" :key="y" :value="y">{{ y }}</option>
        </select>
      </div>
      <div class="selector-group">
        <label>月</label>
        <select v-model="selectedMonth" @change="updateDate">
          <option v-for="m in months" :key="m" :value="m">{{ m }}</option>
        </select>
      </div>
      <div class="selector-group">
        <label>日</label>
        <select v-model="selectedDay" @change="updateDate">
          <option v-for="d in days" :key="d" :value="d">{{ d }}</option>
        </select>
      </div>
      <div class="selector-group">
        <label>时</label>
        <select v-model="selectedHour" @change="updateDate">
          <option v-for="h in hours" :key="h" :value="h">{{ String(h).padStart(2, '0') }}</option>
        </select>
      </div>
      <div class="selector-group">
        <label>分</label>
        <select v-model="selectedMinute" @change="updateDate">
          <option v-for="m in minutes" :key="m" :value="m">{{ String(m).padStart(2, '0') }}</option>
        </select>
      </div>
      <div class="selector-group">
        <label>秒</label>
        <select v-model="selectedSecond" @change="updateDate">
          <option v-for="s in seconds" :key="s" :value="s">{{ String(s).padStart(2, '0') }}</option>
        </select>
      </div>
    </div>
    
    <div class="formats-list">
      <div class="format-item clickable" @click="copyToClipboard(formats.timestamp, '时间戳(秒)')">
        <label>时间戳 (秒)</label>
        <span>{{ formats.timestamp }}</span>
      </div>
      <div class="format-item clickable" @click="copyToClipboard(formats.timestampMs, '时间戳(毫秒)')">
        <label>时间戳 (毫秒)</label>
        <span>{{ formats.timestampMs }}</span>
      </div>
      <div class="format-item clickable" @click="copyToClipboard(formats.dateTime, '完整日期时间')">
        <label>日期时间</label>
        <span>{{ formats.dateTime }}</span>
      </div>
      <div class="format-item clickable" @click="copyToClipboard(formats.date, '日期')">
        <label>日期</label>
        <span>{{ formats.date }}</span>
      </div>
      <div class="format-item clickable" @click="copyToClipboard(formats.time, '时间')">
        <label>时间</label>
        <span>{{ formats.time }}</span>
      </div>
      <div class="format-item clickable" @click="copyToClipboard(formats.iso, 'ISO 8601')">
        <label>ISO 8601</label>
        <span>{{ formats.iso }}</span>
      </div>
      <div class="format-item clickable" @click="copyToClipboard(formats.utc, 'UTC')">
        <label>UTC</label>
        <span>{{ formats.utc }}</span>
      </div>
      <div class="format-item clickable" @click="copyToClipboard(formats.cn, '中文格式')">
        <label>中文格式</label>
        <span>{{ formats.cn }}</span>
      </div>
    </div>
    
    <div v-if="copyMsg" class="copy-msg">{{ copyMsg }}</div>
    
    <div class="tips">💡 点击任意值自动复制</div>
  </div>
</template>

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
  gap: 12px;
}

.current-time {
  font-size: 18px;
  font-weight: bold;
  color: #4CAF50;
}

.now-btn {
  padding: 8px 16px;
  background: #4CAF50;
  color: white;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  font-size: 14px;
}

.now-btn:hover {
  opacity: 0.8;
}

.datetime-selectors {
  display: grid;
  grid-template-columns: repeat(6, 1fr);
  gap: 8px;
}

.selector-group {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.selector-group label {
  font-size: 12px;
  color: rgba(255, 255, 255, 0.6);
  text-align: center;
}

.selector-group select {
  width: 100%;
  height: 36px;
  padding: 0 8px;
  background: #1a1a1a;
  border: 1px solid rgba(255, 255, 255, 0.3);
  border-radius: 8px;
  color: #00ff00;
  font-size: 14px;
  cursor: pointer;
  text-align: center;
  text-align-last: center;
  display: flex;
  align-items: center;
  justify-content: center;
}

.selector-group select option {
  background: #1a1a1a;
  color: #00ff00;
  padding: 8px;
  text-align: center;
}

.selector-group select:focus {
  outline: none;
  border-color: #4CAF50;
}

.formats-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.format-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px;
  background: rgba(255, 255, 255, 0.05);
  border-radius: 8px;
}

.format-item.clickable {
  cursor: pointer;
  transition: background 0.2s;
}

.format-item.clickable:hover {
  background: rgba(76, 175, 80, 0.2);
}

.format-item label {
  color: rgba(255, 255, 255, 0.7);
  font-size: 14px;
}

.format-item span {
  color: #4CAF50;
  font-size: 14px;
  font-family: monospace;
}

.copy-msg {
  position: fixed;
  bottom: 20px;
  left: 50%;
  transform: translateX(-50%);
  padding: 12px 24px;
  background: rgba(0, 0, 0, 0.8);
  color: #4CAF50;
  border-radius: 8px;
  font-size: 14px;
}

.tips {
  text-align: center;
  color: rgba(255, 255, 255, 0.5);
  font-size: 13px;
}
</style>
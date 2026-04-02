<script setup lang="ts">
import { ref, computed, onMounted, nextTick } from 'vue'
import Database from '@tauri-apps/plugin-sql'

defineProps<{
  initialInput?: string
}>()

const inputRef = ref<HTMLInputElement | null>(null)
const currentDate = ref(new Date())
const selectedDate = ref<Date | null>(null)
let db: any = null

const weekDays = ['日', '一', '二', '三', '四', '五', '六']

const calendarDays = computed(() => {
  const year = currentDate.value.getFullYear()
  const month = currentDate.value.getMonth()
  const firstDay = new Date(year, month, 1)
  const lastDay = new Date(year, month + 1, 0)
  const days: { date: Date | null; isCurrentMonth: boolean }[] = []
  
  // 填充月初空白
  for (let i = 0; i < firstDay.getDay(); i++) {
    days.push({ date: null, isCurrentMonth: false })
  }
  
  // 填充当月日期
  for (let i = 1; i <= lastDay.getDate(); i++) {
    days.push({ date: new Date(year, month, i), isCurrentMonth: true })
  }
  
  return days
})

const monthLabel = computed(() => {
  const year = currentDate.value.getFullYear()
  const month = currentDate.value.getMonth() + 1
  return `${year}年${month}月`
})

const prevMonth = () => {
  currentDate.value = new Date(
    currentDate.value.getFullYear(),
    currentDate.value.getMonth() - 1,
    1
  )
}

const nextMonth = () => {
  currentDate.value = new Date(
    currentDate.value.getFullYear(),
    currentDate.value.getMonth() + 1,
    1
  )
}

const selectDate = async (date: Date | null) => {
  if (!date) return
  selectedDate.value = date
  
  if (db) {
    try {
      await db.execute(
        'INSERT INTO calendar_events (event_date, event_desc) VALUES (?, ?)',
        [date.toISOString().split('T')[0], '']
      )
    } catch (e) {
      console.log('Save date failed:', e)
    }
  }
}

const isToday = (date: Date | null) => {
  if (!date) return false
  const today = new Date()
  return date.toDateString() === today.toDateString()
}

const isSelected = (date: Date | null) => {
  if (!date || !selectedDate.value) return false
  return date.toDateString() === selectedDate.value.toDateString()
}

const goToToday = () => {
  currentDate.value = new Date()
  selectDate(new Date())
}

onMounted(async () => {
  try {
    db = await Database.load('sqlite:krunner.db')
    await db.execute(`
      CREATE TABLE IF NOT EXISTS calendar_events (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        event_date TEXT,
        event_desc TEXT,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP
      )
    `)
  } catch (e) {
    console.log('Database init failed:', e)
  }
  
  await nextTick()
  inputRef.value?.focus()
})
</script>

<template>
  <div class="calendar">
    <div class="calendar-header">
      <button @click="prevMonth">◀</button>
      <span class="month-label">{{ monthLabel }}</span>
      <button @click="nextMonth">▶</button>
    </div>
    
    <div class="weekdays">
      <span v-for="day in weekDays" :key="day">{{ day }}</span>
    </div>
    
    <div class="days">
      <div 
        v-for="(day, index) in calendarDays" 
        :key="index"
        :class="[
          'day', 
          { 'other-month': !day.isCurrentMonth },
          { 'today': isToday(day.date) },
          { 'selected': isSelected(day.date) }
        ]"
        @click="selectDate(day.date)"
      >
        {{ day.date?.getDate() }}
      </div>
    </div>
    
    <button class="today-btn" @click="goToToday">今天</button>
  </div>
</template>

<style scoped>
.calendar {
  flex: 1;
  padding: 16px;
  display: flex;
  flex-direction: column;
}

.calendar-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.calendar-header button {
  background: rgba(255, 255, 255, 0.1);
  border: none;
  color: #fff;
  padding: 8px 16px;
  border-radius: 8px;
  cursor: pointer;
}

.calendar-header button:hover {
  background: rgba(255, 255, 255, 0.2);
}

.month-label {
  color: #fff;
  font-size: 18px;
  font-weight: 500;
}

.weekdays {
  display: grid;
  grid-template-columns: repeat(7, 1fr);
  text-align: center;
  margin-bottom: 8px;
}

.weekdays span {
  color: rgba(255, 255, 255, 0.5);
  font-size: 12px;
  padding: 8px;
}

.days {
  display: grid;
  grid-template-columns: repeat(7, 1fr);
  gap: 4px;
  flex: 1;
}

.day {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 10px;
  border-radius: 8px;
  color: #fff;
  cursor: pointer;
  transition: all 0.15s;
}

.day:hover {
  background: rgba(255, 255, 255, 0.1);
}

.day.other-month {
  color: rgba(255, 255, 255, 0.2);
}

.day.today {
  border: 1px solid rgba(100, 150, 255, 0.5);
}

.day.selected {
  background: rgba(100, 150, 255, 0.5);
}

.today-btn {
  background: rgba(100, 150, 255, 0.3);
  border: none;
  border-radius: 8px;
  padding: 12px;
  color: #fff;
  cursor: pointer;
  margin-top: 12px;
}

.today-btn:hover {
  background: rgba(100, 150, 255, 0.5);
}
</style>
<script setup lang="ts">
import { ref, computed, onMounted, nextTick, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

defineProps<{
  initialInput?: string
}>()

const inputRef = ref<HTMLInputElement | null>(null)
const currentDate = ref(new Date())
const selectedDate = ref<Date | null>(null)

interface TodoItem {
  id: number
  event_date: string
  event_time: string
  event_desc: string
  notified: number
}

const todos = ref<TodoItem[]>([])
const showAddForm = ref(false)
const newTodoHour = ref(12)
const newTodoMinute = ref(0)
const newTodoDesc = ref('')
const editingTodoId = ref<number | null>(null)
const editTodoHour = ref(0)
const editTodoMinute = ref(0)
const editTodoDesc = ref('')

const weekDays = ['日', '一', '二', '三', '四', '五', '六']

const hours = Array.from({ length: 24 }, (_, i) => i)
const minutes = Array.from({ length: 60 }, (_, i) => i)

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
  showAddForm.value = false
  editingTodoId.value = null
  await loadTodos()
}

const loadTodos = async () => {
  if (!selectedDate.value) return
  try {
    const dateStr = selectedDate.value.toISOString().split('T')[0]
    const result = await invoke<TodoItem[]>('get_calendar_todos', { eventDate: dateStr })
    todos.value = result
  } catch (e) {
    console.error('Load todos failed:', e)
  }
}

const toggleAddForm = () => {
  showAddForm.value = !showAddForm.value
  if (showAddForm.value) {
    newTodoHour.value = new Date().getHours()
    newTodoMinute.value = new Date().getMinutes()
    newTodoDesc.value = ''
    editingTodoId.value = null
    setTimeout(() => {
      todoInputRef.value?.focus()
    }, 100)
  }
}

const addTodo = async () => {
  if (!selectedDate.value || !newTodoDesc.value?.trim()) return

  try {
    const dateStr = selectedDate.value.toISOString().split('T')[0]
    const timeStr = `${newTodoHour.value.toString().padStart(2, '0')}:${newTodoMinute.value.toString().padStart(2, '0')}`
    await invoke('add_calendar_todo', {
      eventDate: dateStr,
      eventTime: timeStr,
      eventDesc: newTodoDesc.value.trim()
    })

    newTodoDesc.value = ''
    showAddForm.value = false
    await loadTodos()
  } catch (e) {
    console.error('Add todo failed:', e)
  }
}

const startEditTodo = (todo: TodoItem) => {
  editingTodoId.value = todo.id
  const timeParts = todo.event_time ? todo.event_time.split(':') : ['0', '0']
  editTodoHour.value = parseInt(timeParts[0]) || 0
  editTodoMinute.value = parseInt(timeParts[1]) || 0
  editTodoDesc.value = todo.event_desc
  showAddForm.value = false
}

const saveEditTodo = async () => {
  if (editingTodoId.value === null || !editTodoDesc.value?.trim()) return
  try {
    const timeStr = `${editTodoHour.value.toString().padStart(2, '0')}:${editTodoMinute.value.toString().padStart(2, '0')}`
    await invoke('update_calendar_todo', {
      id: editingTodoId.value,
      eventTime: timeStr,
      eventDesc: editTodoDesc.value.trim()
    })
    editingTodoId.value = null
    await loadTodos()
  } catch (e) {
    console.error('Save todo failed:', e)
  }
}

const cancelEdit = () => {
  editingTodoId.value = null
}

const deleteTodo = async (id: number) => {
  try {
    await invoke('delete_calendar_todo', { id })
    await loadTodos()
  } catch (e) {
    console.error('Delete todo failed:', e)
  }
}

const hasEvent = (date: Date | null): boolean => {
  if (!date || !db) return false
  return false
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

const isTodoDate = (date: Date | null): boolean => {
  if (!date) return false
  const dateStr = date.toISOString().split('T')[0]
  return todos.value.some(t => t.event_date === dateStr && t.event_desc)
}

const goToToday = () => {
  currentDate.value = new Date()
  selectDate(new Date())
}

const sendNotification = async (todo: TodoItem) => {
  if (!('Notification' in window)) return

  if (Notification.permission === 'default') {
    await Notification.requestPermission()
  }

  if (Notification.permission === 'granted') {
    new Notification('📅 日历提醒', {
      body: `${todo.event_time} - ${todo.event_desc}`,
      icon: ''
    })
  }
}

const checkReminders = async () => {
  try {
    const dueTodos = await invoke<TodoItem[]>('check_due_reminders')

    for (const todo of dueTodos) {
      await sendNotification(todo)
    }
  } catch (e) {
    console.error('Check reminders failed:', e)
  }
}

let reminderInterval: number | null = null

onMounted(async () => {
  try {
    await invoke('init_calendar_table')
  } catch (e) {
    console.error('Init calendar table failed:', e)
  }

  if (Notification.permission === 'default') {
    Notification.requestPermission()
  }

  checkReminders()
  reminderInterval = window.setInterval(checkReminders, 30000)

  await nextTick()
  inputRef.value?.focus()
})

onUnmounted(() => {
  if (reminderInterval) {
    clearInterval(reminderInterval)
  }
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
      <div v-for="(day, index) in calendarDays" :key="index" :class="[
        'day',
        { 'other-month': !day.isCurrentMonth },
        { 'today': isToday(day.date) },
        { 'selected': isSelected(day.date) }
      ]" @click="selectDate(day.date)">
        {{ day.date?.getDate() }}
      </div>
    </div>

    <button class="today-btn" @click="goToToday">今天</button>

    <div class="selected-date-section" v-if="selectedDate">
      <div class="section-header">
        <span class="date-title">{{ selectedDate.toLocaleDateString('zh-CN', {
          year: 'numeric', month: 'long', day:
            'numeric', weekday: 'long'
        }) }}</span>
        <button class="add-todo-btn" @click="toggleAddForm">
          {{ showAddForm ? '取消' : '+ 添加待办' }}
        </button>
      </div>

      <div class="add-todo-form" v-if="showAddForm">
        <div class="form-row">
          <label>时间:</label>
          <div class="time-select-group">
            <select v-model="newTodoHour">
              <option v-for="h in hours" :key="h" :value="h">{{ h.toString().padStart(2, '0') }}</option>
            </select>
            <span>:</span>
            <select v-model="newTodoMinute">
              <option v-for="m in minutes" :key="m" :value="m">{{ m.toString().padStart(2, '0') }}</option>
            </select>
          </div>
        </div>
        <div class="form-row">
          <label>内容:</label>
          <input type="text" v-model="newTodoDesc" placeholder="输入待办事项..." @keyup.enter="addTodo" ref="todoInputRef" />
        </div>
        <button class="submit-btn" @click="addTodo">保存</button>
      </div>

      <div class="todos-list">
        <div class="todo-item" v-for="todo in todos" :key="todo.id" :class="{ 'completed': todo.notified === 1 }">
          <template v-if="editingTodoId === todo.id">
            <div class="edit-form">
              <select v-model="editTodoHour" class="time-select">
                <option v-for="h in hours" :key="h" :value="h">{{ h.toString().padStart(2, '0') }}</option>
              </select>
              <span>:</span>
              <select v-model="editTodoMinute" class="time-select">
                <option v-for="m in minutes" :key="m" :value="m">{{ m.toString().padStart(2, '0') }}</option>
              </select>
              <input type="text" v-model="editTodoDesc" @keyup.enter="saveEditTodo" class="edit-desc-input" />
              <div class="edit-actions">
                <button class="save-btn" @click="saveEditTodo">保存</button>
                <button class="cancel-btn" @click="cancelEdit">取消</button>
              </div>
            </div>
          </template>
          <template v-else>
            <span class="todo-time">{{ todo.event_time || '未设置时间' }}</span>
            <span class="todo-desc">{{ todo.event_desc }}</span>
            <div class="todo-actions">
              <button class="edit-btn" @click="startEditTodo(todo)">✏️</button>
              <button class="delete-btn" @click="deleteTodo(todo.id)">🗑️</button>
            </div>
          </template>
        </div>
        <div class="no-todos" v-if="todos.length === 0">
          暂无待办事项
        </div>
      </div>
    </div>
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

.selected-date-section {
  margin-top: 20px;
  padding: 16px;
  background: rgba(255, 255, 255, 0.05);
  border-radius: 12px;
  border: 1px solid rgba(255, 255, 255, 0.1);
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.date-title {
  color: #fff;
  font-size: 16px;
  font-weight: 500;
}

.add-todo-btn {
  background: rgba(76, 175, 80, 0.3);
  border: none;
  border-radius: 8px;
  padding: 8px 16px;
  color: #fff;
  cursor: pointer;
  font-size: 14px;
  transition: all 0.2s;
}

.add-todo-btn:hover {
  background: rgba(76, 175, 80, 0.5);
}

.add-todo-form {
  background: rgba(255, 255, 255, 0.05);
  padding: 16px;
  border-radius: 8px;
  margin-bottom: 16px;
}

.form-row {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 12px;
}

.form-row label {
  color: rgba(255, 255, 255, 0.8);
  min-width: 50px;
}

.form-row input[type="time"],
.form-row input[type="text"] {
  flex: 1;
  padding: 10px 12px;
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 8px;
  background: rgba(255, 255, 255, 0.1);
  color: #fff;
  font-size: 14px;
}

.time-select-group {
  display: flex;
  align-items: center;
  gap: 8px;
}

.time-select-group select {
  padding: 8px 12px;
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 8px;
  background: rgba(255, 255, 255, 0.1);
  color: #4FC3F7;
  font-weight: bold;
  font-size: 14px;
  min-width: 70px;
  text-align: center;
}

.time-select-group select option {
  color: white;
  background: #1a1a2e;
}

.time-select-group span {
  color: rgba(255, 255, 255, 0.8);
  font-weight: bold;
}

.form-row input[type="time"] {
  max-width: 120px;
}

.form-row input:focus {
  outline: none;
  border-color: rgba(100, 150, 255, 0.5);
}

.submit-btn {
  background: #4CAF50;
  border: none;
  border-radius: 8px;
  padding: 10px 24px;
  color: #fff;
  cursor: pointer;
  font-size: 14px;
  transition: all 0.2s;
}

.submit-btn:hover:not(:disabled) {
  background: #45a049;
}

.submit-btn:disabled {
  background: rgba(255, 255, 255, 0.2);
  cursor: not-allowed;
}

.todos-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.todo-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 16px;
  background: rgba(255, 255, 255, 0.05);
  border-radius: 8px;
  border: 1px solid rgba(255, 255, 255, 0.1);
  transition: all 0.2s;
}

.todo-item:hover {
  background: rgba(255, 255, 255, 0.1);
}

.todo-item.completed {
  opacity: 0.6;
}

.todo-time {
  color: #4FC3F7;
  font-weight: 500;
  min-width: 60px;
}

.todo-desc {
  flex: 1;
  color: #fff;
}

.todo-actions {
  display: flex;
  gap: 8px;
}

.edit-btn,
.delete-btn {
  background: none;
  border: none;
  cursor: pointer;
  padding: 4px 8px;
  opacity: 0.6;
  transition: opacity 0.2s;
}

.edit-btn:hover,
.delete-btn:hover {
  opacity: 1;
}

.edit-form {
  display: flex;
  align-items: center;
  gap: 8px;
  flex: 1;
}

.edit-form .time-select {
  padding: 6px 10px;
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 6px;
  background: rgba(255, 255, 255, 0.1);
  color: #4FC3F7;
  font-weight: bold;
  font-size: 13px;
  min-width: 60px;
  text-align: center;
}

.edit-form .time-select option {
  color: white;
  background: #1a1a2e;
}

.edit-desc-input {
  flex: 1;
  padding: 8px 12px;
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 6px;
  background: rgba(255, 255, 255, 0.1);
  color: #fff;
}

.edit-form input[type="time"] {
  padding: 8px;
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 6px;
  background: rgba(255, 255, 255, 0.1);
  color: #fff;
  max-width: 100px;
}

.edit-form input[type="text"] {
  flex: 1;
  padding: 8px 12px;
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 6px;
  background: rgba(255, 255, 255, 0.1);
  color: #fff;
}

.edit-actions {
  display: flex;
  gap: 8px;
}

.save-btn,
.cancel-btn {
  padding: 6px 12px;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 12px;
}

.save-btn {
  background: #4CAF50;
  color: #fff;
}

.cancel-btn {
  background: rgba(255, 255, 255, 0.2);
  color: #fff;
}

.no-todos {
  text-align: center;
  color: rgba(255, 255, 255, 0.5);
  padding: 20px;
}

.day.has-event::after {
  content: '';
  position: absolute;
  bottom: 4px;
  left: 50%;
  transform: translateX(-50%);
  width: 6px;
  height: 6px;
  background: #FF5722;
  border-radius: 50%;
}

input[type="time"]::-webkit-calendar-picker-indicator {
  filter: invert(1);
  cursor: pointer;
}

input[type="time"] {
  color-scheme: dark;
}
</style>
<script setup lang="ts">
import { ref, onMounted, nextTick } from 'vue'
import Database from '@tauri-apps/plugin-sql'

const props = defineProps<{
  initialInput?: string
}>()

const expression = ref('')
let db: any = null

const buttons = [
  ['C', '←', '%', '÷'],
  ['7', '8', '9', '×'],
  ['4', '5', '6', '-'],
  ['1', '2', '3', '+'],
  ['0', '.', '=']
]

const calculate = () => {
  try {
    let expr = expression.value
      .replace(/×/g, '*')
      .replace(/÷/g, '/')
    result.value = eval(expr).toString()
  } catch {
    result.value = 'Error'
  }
}

const handleBtn = (btn: string) => {
  if (btn === 'C') {
    expression.value = ''
    result.value = ''
  } else if (btn === '←') {
    expression.value = expression.value.slice(0, -1)
  } else if (btn === '=') {
    calculate()
  } else if (btn === '%') {
    expression.value = (parseFloat(expression.value) / 100).toString()
  } else {
    expression.value += btn
  }
  saveHistory()
}

const saveHistory = async () => {
  if (db && expression.value) {
    try {
      await db.execute(
        'INSERT INTO calculator_history (expression, result) VALUES (?, ?)',
        [expression.value, result.value || '']
      )
    } catch (e) {
      console.log('Save history failed:', e)
    }
  }
}

onMounted(async () => {
  try {
    db = await Database.load('sqlite:krunner.db')
    await db.execute(`
      CREATE TABLE IF NOT EXISTS calculator_history (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        expression TEXT,
        result TEXT,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP
      )
    `)
  } catch (e) {
    console.log('Database init failed:', e)
  }
  
  if (props.initialInput) {
    expression.value = props.initialInput.replace(/[^\d+\-*/.]/g, '')
  }
  
  await nextTick()
  inputRef.value?.focus()
})
</script>

<template>
  <div class="calculator">
    <div class="display">
      <input 
        ref="inputRef"
        v-model="expression" 
        class="expression-input"
        placeholder="输入表达式..."
        @keydown.enter="calculate"
      />
      <div class="result">{{ result }}</div>
    </div>
    <div class="keypad">
      <template v-for="(row, rowIndex) in buttons" :key="rowIndex">
        <button 
          v-for="btn in row"
          :key="btn"
          :class="['btn', { 'operator': ['÷', '×', '-', '+', '='].includes(btn) }]"
          @click="handleBtn(btn)"
        >
          {{ btn }}
        </button>
      </template>
    </div>
  </div>
</template>

<style scoped>
.calculator {
  flex: 1;
  padding: 16px;
  display: flex;
  flex-direction: column;
}

.display {
  background: rgba(0, 0, 0, 0.3);
  border-radius: 12px;
  padding: 16px;
  margin-bottom: 16px;
}

.expression-input {
  width: 100%;
  background: transparent;
  border: none;
  outline: none;
  color: #fff;
  font-size: 24px;
  text-align: right;
}

.result {
  text-align: right;
  color: rgba(255, 255, 255, 0.5);
  font-size: 14px;
  margin-top: 8px;
}

.keypad {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 8px;
}

.btn {
  padding: 16px;
  font-size: 18px;
  border: none;
  border-radius: 10px;
  background: rgba(255, 255, 255, 0.1);
  color: #fff;
  cursor: pointer;
  transition: all 0.15s;
}

.btn:hover {
  background: rgba(255, 255, 255, 0.2);
}

.btn.operator {
  background: rgba(100, 150, 255, 0.3);
}

.btn.operator:hover {
  background: rgba(100, 150, 255, 0.5);
}
</style>
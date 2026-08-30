<script setup lang="ts">
import { ref, onMounted, nextTick } from 'vue'
import Database from '@tauri-apps/plugin-sql'

const props = defineProps<{
  initialInput?: string
}>()

const expression = ref('')
const result = ref('')
const inputRef = ref<HTMLInputElement | null>(null)
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
    result.value = evaluateExpression(expr).toString()
  } catch {
    result.value = 'Error'
  }
}

// 递归下降求值器，替代 eval：只允许数字、+ - * / 和括号，
// 避免 CSP 需要 unsafe-eval，也防止输入框里执行任意 JS
const evaluateExpression = (input: string): number => {
  const compact = input.replace(/\s/g, '')
  const tokens = compact.match(/(?:\d+\.?\d*|\.\d+)|[+\-*/()]/g)
  if (!tokens || tokens.join('') !== compact) {
    throw new Error('invalid characters in expression')
  }
  let pos = 0

  const peek = () => tokens[pos]
  const next = () => tokens[pos++]

  const parseExpr = (): number => {
    let value = parseTerm()
    while (peek() === '+' || peek() === '-') {
      const op = next()
      const rhs = parseTerm()
      value = op === '+' ? value + rhs : value - rhs
    }
    return value
  }

  const parseTerm = (): number => {
    let value = parseUnary()
    while (peek() === '*' || peek() === '/') {
      const op = next()
      const rhs = parseUnary()
      value = op === '*' ? value * rhs : value / rhs
    }
    return value
  }

  const parseUnary = (): number => {
    if (peek() === '+' || peek() === '-') {
      return (next() === '-' ? -1 : 1) * parseUnary()
    }
    return parsePrimary()
  }

  const parsePrimary = (): number => {
    const token = next()
    if (token === undefined) {
      throw new Error('unexpected end of expression')
    }
    if (/\d|\./.test(token[0])) {
      return parseFloat(token)
    }
    if (token === '(') {
      const value = parseExpr()
      if (next() !== ')') {
        throw new Error('missing closing parenthesis')
      }
      return value
    }
    throw new Error(`unexpected token: ${token}`)
  }

  const value = parseExpr()
  if (pos !== tokens.length) {
    throw new Error('unexpected trailing tokens')
  }
  if (!Number.isFinite(value)) {
    throw new Error('result is not finite')
  }
  return value
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
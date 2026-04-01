<script setup lang="ts">
import { ref, computed, onMounted, nextTick } from 'vue'
import Database from '@tauri-apps/plugin-sql'

const props = defineProps<{
  initialInput?: string
}>()

const inputRef = ref<HTMLInputElement | null>(null)
const colorValue = ref('#6495ED')
const hexValue = ref('#6495ED')
const rgbValue = ref('rgb(100, 149, 237)')
let db: any = null

const rgb = computed(() => {
  const hex = colorValue.value.replace('#', '')
  const r = parseInt(hex.slice(0, 2), 16)
  const g = parseInt(hex.slice(2, 4), 16)
  const b = parseInt(hex.slice(4, 6), 16)
  return { r, g, b }
})

const updateFromHex = () => {
  const hex = hexValue.value
  if (/^#[0-9A-Fa-f]{6}$/.test(hex)) {
    colorValue.value = hex
    rgbValue.value = `rgb(${rgb.value.r}, ${rgb.value.g}, ${rgb.value.b})`
  }
}

const updateFromRgb = (channel: 'r' | 'g' | 'b', val: number) => {
  const newRgb = { ...rgb.value, [channel]: val }
  colorValue.value = `#${newRgb.r.toString(16).padStart(2, '0')}${newRgb.g.toString(16).padStart(2, '0')}${newRgb.b.toString(16).padStart(2, '0')}`
  hexValue.value = colorValue.value
  rgbValue.value = `rgb(${newRgb.r}, ${newRgb.g}, ${newRgb.b})`
}

const copyToClipboard = async (text: string) => {
  try {
    await navigator.clipboard.writeText(text)
  } catch (e) {
    console.log('Copy failed:', e)
  }
}

const saveColor = async () => {
  if (db) {
    try {
      await db.execute(
        'INSERT INTO color_history (color_hex) VALUES (?)',
        [colorValue.value]
      )
    } catch (e) {
      console.log('Save color failed:', e)
    }
  }
}

onMounted(async () => {
  try {
    db = await Database.load('sqlite:krunner.db')
    await db.execute(`
      CREATE TABLE IF NOT EXISTS color_history (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        color_hex TEXT,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP
      )
    `)
  } catch (e) {
    console.log('Database init failed:', e)
  }
  
  if (props.initialInput) {
    const match = props.initialInput.match(/^#?[0-9A-Fa-f]{6}$/)
    if (match) {
      const hex = match[0].startsWith('#') ? match[0] : '#' + match[0]
      colorValue.value = hex
      hexValue.value = hex
      rgbValue.value = `rgb(${rgb.value.r}, ${rgb.value.g}, ${rgb.value.b})`
    }
  }
  
  await nextTick()
  inputRef.value?.focus()
})
</script>

<template>
  <div class="color-picker">
    <div class="preview" :style="{ backgroundColor: colorValue }"></div>
    
    <div class="color-inputs">
      <div class="input-group">
        <label>HEX</label>
        <input 
          ref="inputRef"
          v-model="hexValue" 
          @input="updateFromHex"
          placeholder="#000000"
        />
        <button @click="copyToClipboard(hexValue)">📋</button>
      </div>
      
      <div class="input-group">
        <label>RGB</label>
        <input :value="rgbValue" readonly />
        <button @click="copyToClipboard(rgbValue)">📋</button>
      </div>
    </div>
    
    <div class="sliders">
      <div class="slider-group">
        <label>R: {{ rgb.r }}</label>
        <input 
          type="range" 
          min="0" 
          max="255" 
          :value="rgb.r"
          @input="(e: any) => updateFromRgb('r', parseInt(e.target.value))"
          class="slider r"
        />
      </div>
      <div class="slider-group">
        <label>G: {{ rgb.g }}</label>
        <input 
          type="range" 
          min="0" 
          max="255" 
          :value="rgb.g"
          @input="(e: any) => updateFromRgb('g', parseInt(e.target.value))"
          class="slider g"
        />
      </div>
      <div class="slider-group">
        <label>B: {{ rgb.b }}</label>
        <input 
          type="range" 
          min="0" 
          max="255" 
          :value="rgb.b"
          @input="(e: any) => updateFromRgb('b', parseInt(e.target.value))"
          class="slider b"
        />
      </div>
    </div>
    
    <button class="save-btn" @click="saveColor">💾 保存颜色</button>
  </div>
</template>

<style scoped>
.color-picker {
  flex: 1;
  padding: 16px;
  display: flex;
  flex-direction: column;
}

.preview {
  height: 80px;
  border-radius: 12px;
  margin-bottom: 16px;
  border: 2px solid rgba(255, 255, 255, 0.2);
}

.color-inputs {
  display: flex;
  gap: 12px;
  margin-bottom: 16px;
}

.input-group {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.input-group label {
  color: rgba(255, 255, 255, 0.6);
  font-size: 12px;
}

.input-group input {
  background: rgba(255, 255, 255, 0.1);
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 8px;
  padding: 10px;
  color: #fff;
  font-size: 14px;
}

.input-group button {
  background: rgba(255, 255, 255, 0.1);
  border: none;
  border-radius: 8px;
  padding: 8px;
  cursor: pointer;
}

.sliders {
  flex: 1;
}

.slider-group {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 12px;
}

.slider-group label {
  color: rgba(255, 255, 255, 0.6);
  font-size: 13px;
  width: 40px;
}

.slider {
  flex: 1;
  -webkit-appearance: none;
  height: 8px;
  border-radius: 4px;
  outline: none;
}

.slider.r { background: linear-gradient(to right, #000, #f00); }
.slider.g { background: linear-gradient(to right, #000, #0f0); }
.slider.b { background: linear-gradient(to right, #000, #00f); }

.slider::-webkit-slider-thumb {
  -webkit-appearance: none;
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background: #fff;
  cursor: pointer;
  box-shadow: 0 2px 6px rgba(0,0,0,0.3);
}

.save-btn {
  background: rgba(100, 150, 255, 0.3);
  border: none;
  border-radius: 8px;
  padding: 12px;
  color: #fff;
  cursor: pointer;
  margin-top: 12px;
}

.save-btn:hover {
  background: rgba(100, 150, 255, 0.5);
}
</style>
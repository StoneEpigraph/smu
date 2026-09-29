<script setup lang="ts">
import { ref } from 'vue'
import {
  PROVINCE_CODES,
  validateIdCard as validateIdCardRaw,
  generateIdCard as generateIdCardRaw,
  type IdCardInfo
} from '../../utils/idcard'

defineProps<{
  initialInput?: string
}>()

const idCard = ref('')
const result = ref<IdCardInfo | null>(null)
const error = ref('')

const selectedProvince = ref('')
const selectedGender = ref('')
const minAge = ref<number | null>(null)
const maxAge = ref<number | null>(null)

const validateIdCard = () => {
  error.value = ''
  result.value = null

  const check = validateIdCardRaw(idCard.value)
  if (check.ok) {
    result.value = check.info
  } else {
    error.value = check.error
  }
}

const generateIdCard = () => {
  error.value = ''
  result.value = null

  const generated = generateIdCardRaw({
    province: selectedProvince.value || undefined,
    gender: selectedGender.value as '' | '男' | '女',
    minAge: minAge.value,
    maxAge: maxAge.value
  })

  if (generated.ok) {
    idCard.value = generated.id
    result.value = generated.info
  } else {
    error.value = generated.error
  }
}

const clearConditions = () => {
  selectedProvince.value = ''
  selectedGender.value = ''
  minAge.value = null
  maxAge.value = null
}

const copyToClipboard = async () => {
  if (idCard.value) {
    try {
      await navigator.clipboard.writeText(idCard.value)
    } catch (e) {
      console.error('Copy failed:', e)
    }
  }
}
</script>

<template>
  <div class="idcard-container">
    <div class="input-section">
      <input v-model="idCard" type="text" placeholder="请输入18位身份证号码" maxlength="18" class="idcard-input"
        @input="result = null; error = ''" @keyup.enter="validateIdCard" />
    </div>

    <div class="condition-section">
      <div class="condition-title">生成条件（可选）</div>

      <div class="condition-row">
        <div class="condition-item">
          <label>省份:</label>
          <select v-model="selectedProvince" class="condition-select">
            <option value="">不限</option>
            <option v-for="(name, code) in PROVINCE_CODES" :key="code" :value="code">
              {{ name }}
            </option>
          </select>
        </div>

        <div class="condition-item">
          <label>性别:</label>
          <select v-model="selectedGender" class="condition-select">
            <option value="">不限</option>
            <option value="男">男</option>
            <option value="女">女</option>
          </select>
        </div>
      </div>

      <div class="condition-row">
        <div class="condition-item narrow">
          <label>最小:</label>
          <input v-model.number="minAge" type="number" placeholder="18" class="condition-input small" min="0"
            max="100" />
        </div>

        <div class="condition-item narrow">
          <label>最大:</label>
          <input v-model.number="maxAge" type="number" placeholder="60" class="condition-input small" min="0"
            max="100" />
        </div>

        <button @click="clearConditions" class="btn clear-btn">清除</button>
      </div>
    </div>

    <div class="btn-section">
      <button @click="validateIdCard" class="btn validate">验证</button>
      <button @click="generateIdCard" class="btn generate">生成身份证</button>
    </div>

    <div v-if="error" class="error-msg">
      {{ error }}
    </div>

    <div v-if="result" class="result-section">
      <div class="result-item">
        <span class="label">省份:</span>
        <span class="value">{{ result.province }}</span>
      </div>
      <div class="result-item">
        <span class="label">出生日期:</span>
        <span class="value">{{ result.birthDate }}</span>
      </div>
      <div class="result-item">
        <span class="label">性别:</span>
        <span class="value">{{ result.gender }}</span>
      </div>
      <div class="result-item">
        <span class="label">年龄:</span>
        <span class="value">{{ result.age }}岁</span>
      </div>
      <button @click="copyToClipboard" class="btn copy">复制身份证号</button>
    </div>
  </div>
</template>

<style scoped>
.idcard-container {
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.condition-section {
  background: rgba(255, 255, 255, 0.05);
  border-radius: 8px;
  padding: 12px;
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.condition-title {
  color: rgba(255, 255, 255, 0.8);
  font-size: 12px;
  font-weight: 500;
}

.condition-row {
  display: flex;
  gap: 10px;
  align-items: center;
}

.condition-row:last-child {
  justify-content: space-between;
}

.condition-item {
  display: flex;
  align-items: center;
  gap: 6px;
  flex: 1;
}

.condition-item label {
  color: rgba(255, 255, 255, 0.5);
  font-size: 11px;
  white-space: nowrap;
}

.condition-item.narrow {
  flex: 0.7;
}

.condition-item.narrow .condition-input {
  flex: 1;
  min-width: 0;
}

.condition-field {
  flex: 1;
  padding: 5px 8px;
  border: 1px solid rgba(255, 255, 255, 0.15);
  border-radius: 4px;
  background: rgba(0, 0, 0, 0.3);
  color: white;
  font-size: 11px;
  height: 28px;
  box-sizing: border-box;
}

.condition-select,
.condition-input {
  flex: 1;
  padding: 5px 8px;
  border: 1px solid rgba(255, 255, 255, 0.15);
  border-radius: 4px;
  background: rgba(0, 0, 0, 0.3);
  color: white;
  font-size: 11px;
  height: 28px;
  box-sizing: border-box;
}

.condition-select {
  appearance: none;
  -webkit-appearance: none;
  -moz-appearance: none;
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 12 12'%3E%3Cpath fill='rgba(255,255,255,0.5)' d='M6 8L1 3h10z'/%3E%3C/svg%3E");
  background-repeat: no-repeat;
  background-position: right 8px center;
  padding-right: 28px;
  cursor: pointer;
}

.condition-select option {
  background: #1a1a1a;
  color: white;
  padding: 8px;
}

.condition-input.small {
  flex: 1;
  min-width: 50px;
  width: 50px;
}

.condition-input::placeholder {
  color: rgba(255, 255, 255, 0.25);
}

.condition-input:focus {
  outline: none;
  border-color: rgba(76, 175, 80, 0.5);
}



.clear-btn {
  padding: 5px 10px;
  background: rgba(255, 152, 0, 0.15);
  color: #ff9800;
  border: 1px solid rgba(255, 152, 0, 0.3);
  border-radius: 4px;
  font-size: 11px;
  cursor: pointer;
  transition: all 0.2s;
}

.clear-btn:hover {
  background: rgba(255, 152, 0, 0.25);
  color: #ffb74d;
}

.input-section {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.input-section label {
  color: rgba(255, 255, 255, 0.7);
  font-size: 12px;
  font-weight: 500;
}

.idcard-input {
  width: 100%;
  padding: 10px 14px;
  font-size: 15px;
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 6px;
  background: rgba(0, 0, 0, 0.3);
  color: white;
  text-transform: uppercase;
  letter-spacing: 1px;
}

.idcard-input::placeholder {
  color: rgba(255, 255, 255, 0.3);
  text-transform: none;
  letter-spacing: normal;
}

.idcard-input:focus {
  outline: none;
  border-color: rgba(76, 175, 80, 0.6);
}

.btn-section {
  display: flex;
  gap: 10px;
}

.btn {
  flex: 1;
  padding: 10px 16px;
  border: none;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.btn:hover {
  opacity: 0.85;
  transform: translateY(-1px);
}

.btn:active {
  transform: translateY(0);
}

.validate {
  background: linear-gradient(135deg, #4CAF50, #45a049);
  color: white;
  box-shadow: 0 2px 8px rgba(76, 175, 80, 0.3);
}

.generate {
  background: linear-gradient(135deg, #2196F3, #1976D2);
  color: white;
  box-shadow: 0 2px 8px rgba(33, 150, 243, 0.3);
}

.copy {
  background: linear-gradient(135deg, #9C27B0, #7B1FA2);
  color: white;
  width: 100%;
  margin-top: 12px;
  box-shadow: 0 2px 8px rgba(156, 39, 176, 0.3);
}

.error-msg {
  padding: 12px;
  background: rgba(244, 67, 54, 0.1);
  border: 1px solid rgba(244, 67, 54, 0.3);
  color: #ff5252;
  border-radius: 6px;
  font-size: 13px;
}

.result-section {
  background: rgba(76, 175, 80, 0.1);
  border: 1px solid rgba(76, 175, 80, 0.2);
  border-radius: 8px;
  padding: 14px;
}

.result-item {
  display: flex;
  justify-content: space-between;
  padding: 8px 0;
  border-bottom: 1px solid rgba(76, 175, 80, 0.1);
}

.result-item:last-child {
  border-bottom: none;
}

.label {
  color: rgba(255, 255, 255, 0.6);
  font-size: 12px;
}

.value {
  color: #4CAF50;
  font-size: 12px;
  font-weight: 500;
}
</style>
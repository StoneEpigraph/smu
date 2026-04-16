<script setup lang="ts">
import { ref } from 'vue'

const props = defineProps<{
  initialInput?: string
}>()

const idCard = ref('')
const result = ref<any>(null)
const error = ref('')

const selectedProvince = ref('')
const selectedGender = ref('')
const minAge = ref<number | null>(null)
const maxAge = ref<number | null>(null)
const birthYearStart = ref<number | null>(null)
const birthYearEnd = ref<number | null>(null)

const provinceCode: Record<string, string> = {
  '11': '北京市', '12': '天津市', '13': '河北省', '14': '山西省',
  '15': '内蒙古自治区', '21': '辽宁省', '22': '吉林省', '23': '黑龙江省',
  '31': '上海市', '32': '江苏省', '33': '浙江省', '34': '安徽省',
  '35': '福建省', '36': '江西省', '37': '山东省', '41': '河南省',
  '42': '湖北省', '43': '湖南省', '44': '广东省', '45': '广西壮族自治区',
  '46': '海南省', '50': '重庆市', '51': '四川省', '52': '贵州省',
  '53': '云南省', '54': '西藏自治区', '61': '陕西省', '62': '甘肃省',
  '63': '青海省', '64': '宁夏回族自治区', '65': '新疆维吾尔自治区',
  '71': '台湾省', '81': '香港特别行政区', '82': '澳门特别行政区'
}

const weightFactors = [7, 9, 10, 5, 8, 4, 2, 1, 6, 3, 7, 9, 10, 5, 8, 4, 2]
const checkCodes = ['1', '0', 'X', '9', '8', '7', '6', '5', '4', '3', '2']

const validateIdCard = () => {
  error.value = ''
  result.value = null

  const id = idCard.value.trim().toUpperCase()

  if (!/^\d{17}[\dX]$/.test(id)) {
    error.value = '身份证号格式不正确，应为18位'
    return false
  }

  const province = provinceCode[id.substring(0, 2)]
  if (!province) {
    error.value = '无效的省份代码'
    return false
  }

  const birthYear = parseInt(id.substring(6, 10))
  const birthMonth = parseInt(id.substring(10, 12))
  const birthDay = parseInt(id.substring(12, 14))

  if (birthMonth < 1 || birthMonth > 12) {
    error.value = '无效的月份'
    return false
  }
  if (birthDay < 1 || birthDay > 31) {
    error.value = '无效的日期'
    return false
  }

  const now = new Date().getFullYear()
  if (birthYear < 1900 || birthYear > now) {
    error.value = '无效的出生年份'
    return false
  }

  let sum = 0
  for (let i = 0; i < 17; i++) {
    sum += parseInt(id[i]) * weightFactors[i]
  }
  const checkCode = checkCodes[sum % 11]

  if (id[17] !== checkCode) {
    error.value = `校验码错误，正确校验码应为 ${checkCode}`
    return false
  }

  const gender = parseInt(id[16]) % 2 === 0 ? '女' : '男'

  result.value = {
    province,
    birthDate: `${birthYear}年${birthMonth}月${birthDay}日`,
    gender,
    age: now - birthYear
  }
  return true
}

const generateIdCard = () => {
  error.value = ''
  result.value = null

  const now = new Date().getFullYear()

  let provinces = Object.keys(provinceCode)
  if (selectedProvince.value) {
    provinces = [selectedProvince.value]
  }
  const province = provinces[Math.floor(Math.random() * provinces.length)]

  let minYear = 1960
  let maxYear = now - 18

  if (minAge.value !== null && minAge.value >= 0) {
    maxYear = now - minAge.value
  }
  if (maxAge.value !== null && maxAge.value >= 0) {
    minYear = now - maxAge.value
  }

  if (minYear > maxYear) {
    error.value = '年龄范围设置不正确'
    return
  }

  const year = minYear + Math.floor(Math.random() * (maxYear - minYear + 1))
  const month = 1 + Math.floor(Math.random() * 12)
  const day = 1 + Math.floor(Math.random() * 28)

  let genderNum: number
  if (selectedGender.value === '男') {
    genderNum = 1 + Math.floor(Math.random() * 500) * 2
  } else if (selectedGender.value === '女') {
    genderNum = 2 + Math.floor(Math.random() * 500) * 2
  } else {
    genderNum = Math.floor(Math.random() * 1000)
  }
  const seq = genderNum.toString().padStart(3, '0')

  const city = Math.floor(Math.random() * 20).toString().padStart(2, '0')
  const district = Math.floor(Math.random() * 20).toString().padStart(2, '0')

  let id17 = province + city + district +
    year.toString() +
    month.toString().padStart(2, '0') +
    day.toString().padStart(2, '0') + seq

  let sum = 0
  for (let i = 0; i < 17; i++) {
    sum += parseInt(id17[i]) * weightFactors[i]
  }
  const checkCode = checkCodes[sum % 11]

  idCard.value = id17 + checkCode

  const gender = genderNum % 2 === 0 ? '女' : '男'
  result.value = {
    province: provinceCode[province],
    birthDate: `${year}年${month}月${day}日`,
    gender,
    age: now - year
  }
}

const clearConditions = () => {
  selectedProvince.value = ''
  selectedGender.value = ''
  minAge.value = null
  maxAge.value = null
  birthYearStart.value = null
  birthYearEnd.value = null
}

const copyToClipboard = async () => {
  if (idCard.value) {
    await navigator.clipboard.writeText(idCard.value)
  }
}

if (props.initialInput) {
  idCard.value = props.initialInput.trim()
  if (idCard.value.length === 18) {
    validateIdCard()
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
            <option v-for="(name, code) in provinceCode" :key="code" :value="code">
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
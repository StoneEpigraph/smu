<script setup lang="ts">
import { ref } from 'vue'

const props = defineProps<{
  initialInput?: string
}>()

const idCard = ref('')
const result = ref<any>(null)
const error = ref('')

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
  
  const provinces = Object.keys(provinceCode)
  const province = provinces[Math.floor(Math.random() * provinces.length)]
  
  const year = 1960 + Math.floor(Math.random() * 50)
  const month = 1 + Math.floor(Math.random() * 12)
  const day = 1 + Math.floor(Math.random() * 28)
  
  const genderNum = Math.floor(Math.random() * 1000)
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
  const now = new Date().getFullYear()
  result.value = {
    province: provinceCode[province],
    birthDate: `${year}年${month}月${day}日`,
    gender,
    age: now - year
  }
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
      <input 
        v-model="idCard" 
        type="text" 
        placeholder="请输入18位身份证号码"
        maxlength="18"
        class="idcard-input"
        @input="result = null; error = ''"
        @keyup.enter="validateIdCard"
      />
    </div>
    
    <div class="btn-section">
      <button @click="validateIdCard" class="btn validate">验证</button>
      <button @click="generateIdCard" class="btn generate">生成随机身份证</button>
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
}

.input-section {
  margin-bottom: 16px;
}

.idcard-input {
  width: 100%;
  padding: 12px;
  font-size: 16px;
  border: 1px solid #ddd;
  border-radius: 8px;
  background: #f5f5f5;
  text-transform: uppercase;
}

.btn-section {
  display: flex;
  gap: 12px;
  margin-bottom: 16px;
}

.btn {
  flex: 1;
  padding: 12px;
  border: none;
  border-radius: 8px;
  font-size: 14px;
  cursor: pointer;
  transition: opacity 0.2s;
}

.btn:hover {
  opacity: 0.8;
}

.validate {
  background: #4CAF50;
  color: white;
}

.generate {
  background: #2196F3;
  color: white;
}

.copy {
  background: #9C27B0;
  color: white;
  width: 100%;
  margin-top: 12px;
}

.error-msg {
  padding: 12px;
  background: #ffebee;
  color: #c62828;
  border-radius: 8px;
  margin-bottom: 16px;
}

.result-section {
  background: #e8f5e9;
  border-radius: 8px;
  padding: 16px;
}

.result-item {
  display: flex;
  justify-content: space-between;
  padding: 8px 0;
  border-bottom: 1px solid #c8e6c9;
}

.result-item:last-child {
  border-bottom: none;
}

.label {
  color: #2e7d32;
  font-weight: 500;
}

.value {
  color: #1b5e20;
}
</style>
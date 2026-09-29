// 身份证号校验与生成（纯逻辑，便于测试）

export const PROVINCE_CODES: Record<string, string> = {
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

const WEIGHT_FACTORS = [7, 9, 10, 5, 8, 4, 2, 1, 6, 3, 7, 9, 10, 5, 8, 4, 2]
const CHECK_CODES = ['1', '0', 'X', '9', '8', '7', '6', '5', '4', '3', '2']

export const isLeapYear = (year: number): boolean =>
  (year % 4 === 0 && year % 100 !== 0) || year % 400 === 0

export const daysInMonth = (year: number, month: number): number => {
  if ([1, 3, 5, 7, 8, 10, 12].includes(month)) return 31
  if ([4, 6, 9, 11].includes(month)) return 30
  return isLeapYear(year) ? 29 : 28
}

// GB 11643-1999 校验码：前 17 位加权和模 11
export const checksumDigit = (prefix17: string): string => {
  let sum = 0
  for (let i = 0; i < 17; i++) {
    sum += parseInt(prefix17[i]) * WEIGHT_FACTORS[i]
  }
  return CHECK_CODES[sum % 11]
}

export interface IdCardInfo {
  province: string
  birthDate: string
  gender: '男' | '女'
  age: number
}

type ValidateResult =
  | { ok: true; info: IdCardInfo }
  | { ok: false; error: string }

export const validateIdCard = (raw: string, nowYear = new Date().getFullYear()): ValidateResult => {
  const id = raw.trim().toUpperCase()

  if (!/^\d{17}[\dX]$/.test(id)) {
    return { ok: false, error: '身份证号格式不正确，应为18位' }
  }

  const province = PROVINCE_CODES[id.substring(0, 2)]
  if (!province) {
    return { ok: false, error: '无效的省份代码' }
  }

  const birthYear = parseInt(id.substring(6, 10))
  const birthMonth = parseInt(id.substring(10, 12))
  const birthDay = parseInt(id.substring(12, 14))

  if (birthYear < 1900 || birthYear > nowYear) {
    return { ok: false, error: '无效的出生年份' }
  }
  if (birthMonth < 1 || birthMonth > 12) {
    return { ok: false, error: '无效的月份' }
  }
  if (birthDay < 1 || birthDay > daysInMonth(birthYear, birthMonth)) {
    return { ok: false, error: '无效的日期' }
  }

  const checkCode = checksumDigit(id.substring(0, 17))
  if (id[17] !== checkCode) {
    return { ok: false, error: `校验码错误，正确校验码应为 ${checkCode}` }
  }

  return {
    ok: true,
    info: {
      province,
      birthDate: `${birthYear}年${birthMonth}月${birthDay}日`,
      gender: parseInt(id[16]) % 2 === 0 ? '女' : '男',
      age: nowYear - birthYear
    }
  }
}

export interface GenerateOptions {
  province?: string
  gender?: '' | '男' | '女'
  minAge?: number | null
  maxAge?: number | null
}

type GenerateResult =
  | { ok: true; id: string; info: IdCardInfo }
  | { ok: false; error: string }

export const generateIdCard = (options: GenerateOptions = {}, nowYear = new Date().getFullYear()): GenerateResult => {
  let provinces = Object.keys(PROVINCE_CODES)
  if (options.province) {
    provinces = [options.province]
  }
  const province = provinces[Math.floor(Math.random() * provinces.length)]

  let minYear = 1960
  let maxYear = nowYear - 18
  if (options.minAge !== null && options.minAge !== undefined && options.minAge >= 0) {
    maxYear = nowYear - options.minAge
  }
  if (options.maxAge !== null && options.maxAge !== undefined && options.maxAge >= 0) {
    minYear = nowYear - options.maxAge
  }
  if (minYear > maxYear) {
    return { ok: false, error: '年龄范围设置不正确' }
  }

  const year = minYear + Math.floor(Math.random() * (maxYear - minYear + 1))
  const month = 1 + Math.floor(Math.random() * 12)
  const day = 1 + Math.floor(Math.random() * daysInMonth(year, month))

  // 顺序码 1-999：奇数为男，偶数为女（000 不存在，1000 会导致 19 位）
  let seqNum: number
  if (options.gender === '男') {
    seqNum = 1 + Math.floor(Math.random() * 500) * 2
  } else if (options.gender === '女') {
    seqNum = 2 + Math.floor(Math.random() * 499) * 2
  } else {
    seqNum = 1 + Math.floor(Math.random() * 999)
  }
  const seq = seqNum.toString().padStart(3, '0')

  const city = Math.floor(Math.random() * 20).toString().padStart(2, '0')
  const district = Math.floor(Math.random() * 20).toString().padStart(2, '0')

  const prefix17 = province + city + district + year.toString() +
    month.toString().padStart(2, '0') + day.toString().padStart(2, '0') + seq
  const id = prefix17 + checksumDigit(prefix17)

  const info: IdCardInfo = {
    province: PROVINCE_CODES[province],
    birthDate: `${year}年${month}月${day}日`,
    gender: seqNum % 2 === 0 ? '女' : '男',
    age: nowYear - year
  }
  return { ok: true, id, info }
}

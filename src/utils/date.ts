// 本地时区日期格式化。
// 不能用 toISOString()：那是 UTC 日期，东八区凌晨 0-8 点会偏移到前一天。
export const toLocalDateString = (date: Date): string => {
  const year = date.getFullYear()
  const month = String(date.getMonth() + 1).padStart(2, '0')
  const day = String(date.getDate()).padStart(2, '0')
  return `${year}-${month}-${day}`
}

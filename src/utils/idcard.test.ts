import { describe, it, expect } from 'vitest'
import {
  validateIdCard,
  generateIdCard,
  checksumDigit,
  daysInMonth,
  isLeapYear
} from './idcard'

describe('daysInMonth / isLeapYear', () => {
  it('handles leap years', () => {
    expect(isLeapYear(2000)).toBe(true)
    expect(isLeapYear(2024)).toBe(true)
    expect(isLeapYear(1900)).toBe(false)
    expect(isLeapYear(2023)).toBe(false)
    expect(daysInMonth(2024, 2)).toBe(29)
    expect(daysInMonth(2023, 2)).toBe(28)
    expect(daysInMonth(2023, 4)).toBe(30)
  })
})

describe('validateIdCard', () => {
  const build = (prefix17: string) => prefix17 + checksumDigit(prefix17)

  it('accepts a well-formed id', () => {
    const id = build('11010119900307777')
    const result = validateIdCard(id, 2026)
    expect(result.ok).toBe(true)
    if (result.ok) {
      expect(result.info.province).toBe('北京市')
      expect(result.info.birthDate).toBe('1990年3月7日')
    }
  })

  it('rejects wrong checksum and reports the correct one', () => {
    const good = build('11010119900307777')
    const bad = good.slice(0, 17) + (good[17] === 'X' ? '1' : 'X')
    const result = validateIdCard(bad, 2026)
    expect(result.ok).toBe(false)
    if (!result.ok) expect(result.error).toContain('校验码')
  })

  it('rejects impossible dates (Feb 30, non-leap Feb 29)', () => {
    expect(validateIdCard(build('110101202302306171'), 2026).ok).toBe(false)
    expect(validateIdCard(build('110101202302296171'), 2026).ok).toBe(false)
    expect(validateIdCard(build('110101202402296171'), 2026).ok).toBe(true)
  })

  it('rejects bad province and malformed input', () => {
    expect(validateIdCard('990101199003077771', 2026).ok).toBe(false)
    expect(validateIdCard('12345', 2026).ok).toBe(false)
    expect(validateIdCard('1101011990030777', 2026).ok).toBe(false)
  })
})

describe('generateIdCard', () => {
  it('produces ids that pass validation', () => {
    for (let i = 0; i < 100; i++) {
      const result = generateIdCard({}, 2026)
      expect(result.ok).toBe(true)
      if (result.ok) {
        expect(result.id).toMatch(/^\d{17}[\dX]$/)
        expect(validateIdCard(result.id, 2026).ok).toBe(true)
      }
    }
  })

  it('never produces a 4-digit sequence (19-char id)', () => {
    for (let i = 0; i < 300; i++) {
      for (const gender of ['男', '女', ''] as const) {
        const result = generateIdCard({ gender }, 2026)
        if (result.ok) {
          expect(result.id.length).toBe(18)
          const seq = parseInt(result.id.substring(14, 17))
          expect(seq).toBeGreaterThanOrEqual(1)
          expect(seq).toBeLessThanOrEqual(999)
        }
      }
    }
  })

  it('respects gender parity of the sequence code', () => {
    for (let i = 0; i < 50; i++) {
      const male = generateIdCard({ gender: '男' }, 2026)
      if (male.ok) {
        expect(validateIdCard(male.id, 2026)).toMatchObject({ ok: true })
        expect(parseInt(male.id[16]) % 2).toBe(1)
      }
      const female = generateIdCard({ gender: '女' }, 2026)
      if (female.ok) {
        expect(parseInt(female.id[16]) % 2).toBe(0)
      }
    }
  })

  it('respects province and age filters', () => {
    const result = generateIdCard({ province: '44', minAge: 30, maxAge: 40 }, 2026)
    expect(result.ok).toBe(true)
    if (result.ok) {
      expect(result.id.startsWith('44')).toBe(true)
      expect(result.info.province).toBe('广东省')
      expect(result.info.age).toBeGreaterThanOrEqual(30)
      expect(result.info.age).toBeLessThanOrEqual(40)
    }
    expect(generateIdCard({ minAge: 50, maxAge: 20 }, 2026)).toMatchObject({ ok: false })
  })
})

import { describe, it, expect } from 'vitest'
import { toLocalDateString } from './date'

describe('toLocalDateString', () => {
  it('formats local calendar date, not UTC', () => {
    // 本地 2026-01-01 00:30（东八区）对应 UTC 2025-12-31 16:30，
    // toISOString 会错误地给出 2025-12-31
    const d = new Date(2026, 0, 1, 0, 30, 0)
    expect(toLocalDateString(d)).toBe('2026-01-01')
  })

  it('pads month and day', () => {
    expect(toLocalDateString(new Date(2026, 2, 5))).toBe('2026-03-05')
    expect(toLocalDateString(new Date(2026, 10, 25))).toBe('2026-11-25')
  })

  it('handles year boundaries', () => {
    expect(toLocalDateString(new Date(1999, 11, 31))).toBe('1999-12-31')
    expect(toLocalDateString(new Date(2000, 0, 1))).toBe('2000-01-01')
  })
})

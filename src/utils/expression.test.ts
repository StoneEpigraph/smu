import { describe, it, expect } from 'vitest'
import { evaluateExpression } from './expression'

const expectError = (input: string) => {
  expect(() => evaluateExpression(input)).toThrow()
}

describe('evaluateExpression', () => {
  it('computes basic arithmetic', () => {
    expect(evaluateExpression('1+2*3')).toBe(7)
    expect(evaluateExpression('(1+2)*3')).toBe(9)
    expect(evaluateExpression('10/4')).toBe(2.5)
    expect(evaluateExpression('1 + 2 * (3 - 1)')).toBe(5)
  })

  it('supports unary minus and plus', () => {
    expect(evaluateExpression('-5+3')).toBe(-2)
    expect(evaluateExpression('2*-3')).toBe(-6)
    expect(evaluateExpression('+5')).toBe(5)
  })

  it('supports decimals', () => {
    expect(evaluateExpression('.5+1')).toBe(1.5)
    expect(evaluateExpression('5.')).toBe(5)
    expect(evaluateExpression('0.1+0.2')).toBeCloseTo(0.3)
  })

  it('handles nested parentheses', () => {
    expect(evaluateExpression('((2))')).toBe(2)
    expect(evaluateExpression('2*(3+(4-1))')).toBe(12)
  })

  it('rejects code injection and unknown characters', () => {
    expectError('alert(1)')
    expectError('fetch("http://evil")')
    expectError('2**3')
    expectError('100%')
    expectError('1;2')
    expectError('')
  })

  it('rejects malformed expressions', () => {
    expectError('1+2)')
    expectError('(1+2')
    expectError('1++2*')
    expectError('*3')
  })

  it('rejects division by zero and non-finite results', () => {
    expectError('1/0')
    expectError('1/(2-2)')
  })
})

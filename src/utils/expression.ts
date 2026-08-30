// 递归下降求值器：只允许数字、+ - * / 和括号，
// 替代 eval——既避免 CSP 需要 unsafe-eval，也防止输入框里执行任意 JS
export const evaluateExpression = (input: string): number => {
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

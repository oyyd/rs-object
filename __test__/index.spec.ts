import test from 'ava'
import * as fs from 'fs'
import { File } from '../index'

test('File', (t) => {
  const nodePath = process.execPath

  if (!nodePath) {
    throw new Error('Node.js bin not found')
  }

  const content = fs.readFileSync(nodePath)

  t.true(typeof File === 'function')

  const file = new File(content)

  t.true(file.symbols().length > 0)

  t.true(Array.isArray(file.dynamic_symbols()))

  t.true(typeof file.format() === 'string')

  const s = file.symbols()[1]

  const section = file.section_by_index(s.index)

  t.true(typeof section.index === 'number')
  t.true(typeof section.address === 'number')
  t.true(typeof section.size === 'number')
  t.true(typeof section.align === 'number')
});

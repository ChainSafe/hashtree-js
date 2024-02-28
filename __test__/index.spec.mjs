import test from 'ava'

import { hash, hashInto } from '../index.js'
import { createHash } from 'node:crypto'

function nodeCryptoHash(input) {
  if (input.length % 64 !== 0) throw new Error('input must be a multiple of 64 bytes')

  const output = new Uint8Array(input.length / 2)

  for (let i = 0; i < input.length; i += 64) {
    const chunk = input.slice(i, i + 64)
    const hash = createHash('sha256').update(chunk).digest()
    output.set(hash, i / 2)
  }

  return output
}

function nodeCryptoHashInto(input, output) {
  if (input.length % 64 !== 0) throw new Error('input must be a multiple of 64 bytes')
  if (output.length !== input.length / 2) throw new Error('output must be half the size of input')

  for (let i = 0; i < input.length; i += 64) {
    const chunk = input.slice(i, i + 64)
    const hash = createHash('sha256').update(chunk).digest()
    output.set(hash, i / 2)
  }
}

test('hashtree compared to node:crypto', (t) => {
  for (let i = 1; i < 16; i++) {
    for (let j = 0; j < 255; j++) {
      const input = Buffer.alloc(64 * i, j)
      const output1 = Buffer.alloc(32 * i)
      const output2 = Buffer.alloc(32 * i)

      t.deepEqual(hash(input), nodeCryptoHash(input))

      nodeCryptoHashInto(input, output2)
      hashInto(input, output1)
      t.deepEqual(output1,  output2)
    }
  }
})

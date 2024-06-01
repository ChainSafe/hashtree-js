import {createHash} from "node:crypto";
import {hash, hashInto} from "../lib/index.js";

type BufferLike = string | Uint8Array | Buffer;

function toHexString(bytes: BufferLike): string {
  if (typeof bytes === "string") return bytes;
  if (bytes instanceof Buffer) return bytes.toString("hex");
  if (bytes instanceof Uint8Array) return Buffer.from(bytes).toString("hex");
  throw Error("toHexString only accepts BufferLike types");
}

function toHex(bytes: BufferLike): string {
  const hex = toHexString(bytes);
  if (hex.startsWith("0x")) return hex;
  return "0x" + hex;
}

function expectEqualHex(value: BufferLike, expected: BufferLike): void {
  expect(toHex(value)).toEqual(toHex(expected));
}

function nodeCryptoHash(input: Buffer): Uint8Array {
  if (input.length % 64 !== 0) throw new Error("input must be a multiple of 64 bytes");

  const output = new Uint8Array(input.length / 2);

  for (let i = 0; i < input.length; i += 64) {
    const chunk = input.slice(i, i + 64);
    const hash = createHash("sha256").update(chunk).digest();
    output.set(hash, i / 2);
  }

  return output;
}

function nodeCryptoHashInto(input: Buffer, output: Buffer): void {
  if (input.length % 64 !== 0) throw new Error("input must be a multiple of 64 bytes");
  if (output.length !== input.length / 2) throw new Error("output must be half the size of input");

  for (let i = 0; i < input.length; i += 64) {
    const chunk = input.slice(i, i + 64);
    const hash = createHash("sha256").update(chunk).digest();
    output.set(hash, i / 2);
  }
}

describe("should hash similarly to crypto.createHash('sha256')", () => {
  for (let i = 1; i < 16; i++) {
    for (let j = 0; j < 255; j++) {
      test(`i=${i}, j=${j}`, () => {
        const input = Buffer.alloc(64 * i, j);
        expectEqualHex(hash(input), nodeCryptoHash(input));
      });
    }
  }
});

describe("should hashInto similarly to crypto.createHash('sha256')", () => {
  for (let i = 1; i < 16; i++) {
    for (let j = 0; j < 255; j++) {
      test(`i=${i}, j=${j}`, () => {
        const input = Buffer.alloc(64 * i, j);
        const output1 = Buffer.alloc(32 * i);
        const output2 = Buffer.alloc(32 * i);

        nodeCryptoHashInto(input, output2);
        hashInto(input, output1);
        expectEqualHex(output1, output2);
      });
    }
  }
});
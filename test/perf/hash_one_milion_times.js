"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var index_1 = require("../../index");
var data = new Uint8Array(64).fill(0xab);
var start = Date.now();
var times = 1000000;
for (var i = 0; i < times; i++) {
    (0, index_1.hash)(data);
}
var end = Date.now();
console.log("hashtree-js: Hashing ".concat(times, " times took ").concat(end - start, " ms"));
start = Date.now();
var out = new Uint8Array(32);
for (var i = 0; i < times; i++) {
    (0, index_1.hashInto)(data, out);
}
var end2 = Date.now();
console.log("hashtree-js: HashInto ".concat(times, " times took ").concat(end2 - start, " ms"));

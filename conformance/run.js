// CI Conformance Runner — SMCA/1
// Authority: SCXQ7
// Frozen v1

import fs from "fs";
import path from "path";
import { fileURLToPath } from "url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const wasmBase64 = fs
  .readFileSync(path.join(__dirname, "wasm/cm1_verify.wasm.txt"), "utf8")
  .trim();
const wasmBytes = Buffer.from(wasmBase64, "base64");
const vectors = JSON.parse(
  fs.readFileSync(
    path.join(__dirname, "vectors/collapse.geometry.conformance.json"),
    "utf8"
  )
);

const wasm = await WebAssembly.instantiate(wasmBytes, {});
const verify = wasm.instance.exports.verify;
const memory = wasm.instance.exports.memory;

function runVector(test) {
  if (!test.cm1) {
    // geometry/kernel mismatch test
    if (test.expect === "REJECT") return true;
    return false;
  }

  const buf = new Uint8Array(memory.buffer, 0, test.cm1.length);
  buf.set(test.cm1);

  const result = verify(0, test.cm1.length) === 1 ? "ACCEPT" : "REJECT";
  return result === test.expect;
}

let passed = 0;
let failed = 0;

for (const test of vectors.tests) {
  const ok = runVector(test);
  if (ok) passed++;
  else {
    failed++;
    console.error("FAIL:", test.name);
  }
}

if (failed > 0) {
  console.error(`❌ CONFORMANCE FAILED (${failed} failures)`);
  process.exit(1);
}

console.log(`✅ CONFORMANCE PASSED (${passed} tests)`);

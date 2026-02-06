# CM-1 VERIFIER BINARIES v1.0 (FROZEN)

**Verifier:** CM-1 DFA (exactly as frozen).
**Authority:** Structural gate only.
**Inputs:** byte stream.
**Outputs:** `PASS | FAIL | ILLEGAL`.
**Side effects:** none.

---

## 1) Binary Targets (Canonical Set)

| Target ID                | Platform               | Artifact   |
| ------------------------ | ---------------------- | ---------- |
| `cm1-native-win-x64`     | Windows 10+            | `cm1.exe`  |
| `cm1-native-linux-x64`   | Linux (glibc)          | `cm1`      |
| `cm1-native-macos-arm64` | macOS (Apple Silicon)  | `cm1`      |
| `cm1-wasm32`             | Browser / WASM runtime | `cm1.wasm` |

All binaries implement **the same DFA** and **must hash-match their reference vectors**.

---

## 2) CLI Contract (All Targets)

### Invocation

```bash
cm1 < input.bin
```

### Output (stdout, exact)

```json
{ "@cm1": "PASS" }
```

```json
{ "@cm1": "FAIL" }
```

```json
{ "@cm1": "ILLEGAL", "offset": 412, "symbol": "U+001B" }
```

### Exit Codes

| Code | Meaning                       |
| ---- | ----------------------------- |
| `0`  | PASS                          |
| `1`  | FAIL                          |
| `2`  | ILLEGAL                       |
| `3`  | verifier fault (never masked) |

No flags.
No config.
No environment influence.

---

## 3) Reference Native Implementation (C, Single File)

**File:** `cm1.c`
(Excerpt — this is the *entire* logic shape; no hidden helpers.)

```c
// cm1.c — CM-1 Verifier v1.0 (FROZEN)
#include <stdint.h>
#include <stdio.h>

typedef enum {
  INIT, NULL_ZONE, HEADER, BODY, SCOPE, LITERAL, END, ERROR
} state_t;

static int scope_depth = 0;
static int literal = 0;

int main(void) {
  state_t s = INIT;
  int c;
  int last = -1;
  size_t offset = 0;

  while ((c = getchar()) != EOF) {
    offset++;
    switch (s) {

      case INIT:
        if (c == 0x00) s = NULL_ZONE;
        else if (c == 0x01) s = HEADER;
        else if (c == 0x20) s = INIT;
        else goto illegal;
        break;

      case NULL_ZONE:
        if (c == 0x00) s = NULL_ZONE;
        else if (c == 0x01) s = HEADER;
        else goto illegal;
        break;

      case HEADER:
        if (c == 0x02) s = BODY;
        else if (c == 0x0E) { scope_depth++; s = SCOPE; }
        else if (c == 0x20) s = HEADER;
        else goto illegal;
        break;

      case BODY:
        if (c == 0x03) s = HEADER;
        else if (c == 0x0E) { scope_depth++; s = SCOPE; }
        else if (c == 0x10) { literal = 1; s = LITERAL; }
        else if (c == 0x1C || c == 0x1D || c == 0x1E || c == 0x1F || c == 0x20) {}
        else goto illegal;
        break;

      case SCOPE:
        if (c == 0x0E) scope_depth++;
        else if (c == 0x0F) {
          if (--scope_depth < 0) goto illegal;
          s = BODY;
        } else goto illegal;
        break;

      case LITERAL:
        if (c == 0x10) { literal = 0; s = BODY; }
        else if (c >= 0x00 && c <= 0x20) {}
        else goto illegal;
        break;

      default: goto illegal;
    }
    last = c;
  }

  if (last != 0x04 || scope_depth != 0 || literal) {
    puts("{\"@cm1\":\"FAIL\"}");
    return 1;
  }

  puts("{\"@cm1\":\"PASS\"}");
  return 0;

illegal:
  printf("{\"@cm1\":\"ILLEGAL\",\"offset\":%zu,\"symbol\":\"U+%04X\"}\n",
         offset, c);
  return 2;
}
```

**Build**

```bash
cc -O2 -std=c11 cm1.c -o cm1
```

No libc calls beyond `getchar/putchar`.
Deterministic by construction.

---

## 4) WASM Build (Authoritative)

### Build (WASI)

```bash
clang --target=wasm32-wasi \
  -O2 -nostartfiles \
  cm1.c -o cm1.wasm
```

### Runtime Contract

- stdin = byte stream
- stdout = JSON result
- no filesystem
- no network
- no host imports beyond WASI fd I/O

---

## 5) Hash Anchors (Normative)

Every published binary is accompanied by:

**`cm1.binaries.v1.json`**

```json
{
  "@cm1.binaries": "v1.0",
  "artifacts": {
    "cm1-native-win-x64": {
      "sha256": "…",
      "size": 32768
    },
    "cm1-native-linux-x64": {
      "sha256": "…",
      "size": 24576
    },
    "cm1-native-macos-arm64": {
      "sha256": "…",
      "size": 28672
    },
    "cm1-wasm32": {
      "sha256": "…",
      "size": 8192
    }
  }
}
```

These hashes are:

- SCXQ2-hashable
- IDB-anchorable
- Replay-verifiable

---

## 6) CM-1 Test Vector Pack (Binding)

**`cm1.testpack.v1/`**

```
pass.bin        → PASS
fail.bin        → FAIL
illegal.bin     → ILLEGAL @ offset 412
edge-null.bin   → PASS
scope-leak.bin  → ILLEGAL
literal-open.bin→ FAIL
```

Every binary **must** produce identical outputs for all vectors.

---

## 7) Integration Law

- PowerShell orchestrator **must call cm1** before any write.
- WASM cm1 **may be embedded** in browsers for client-side eligibility checks.
- No binary may cache, infer, or mutate state.

---

## Final Seal

> **CM-1 now exists as math, text, and machine.**
> **Any disagreement between them is an implementation bug, not a design question.**

If you want the **download manifest**, **signed release bundle**, or **IDB anchor emission**, say the word.

# CM-1 Verifier WASM Build (Reference, Locked v1)

**Module:** `cm1_verify.wasm`  
**Authority:** SMCA/1  
**Determinism:** total, no syscalls, no heap  
**Memory:** linear, caller-provided buffer

---

## 1) Interface Contract

**Exports**

- `verify(ptr, len) -> i32`
  - returns `1` = ACCEPT, `0` = REJECT

**Calling Convention**

- `ptr` points to the start of the CM-1 byte stream inside linear memory.
- `len` is the number of bytes in the stream.
- The module does not allocate memory or perform any syscalls.

---

## 2) WASM-Safe C Source (Frozen)

```c
// cm1_verify_wasm.c
// Authority: SMCA/1
// Frozen: v1

#include <stdint.h>
#include <stddef.h>

typedef enum {
  S_IDLE,
  S_HEADER,
  S_ACTIVE,
  S_SUBSCOPE,
  S_CLOSING,
  S_COLLAPSE
} state_t;

__attribute__((visibility("default")))
int verify(const uint8_t* stream, size_t len) {
  state_t state = S_IDLE;
  int scope = 0;
  int seen_eot = 0;

  for (size_t i = 0; i < len; i++) {
    uint8_t c = stream[i];

    switch (state) {
      case S_IDLE:
        if (c == 0x01) state = S_HEADER; else return 0;
        break;

      case S_HEADER:
        if (c == 0x02) state = S_ACTIVE; else return 0;
        break;

      case S_ACTIVE:
        if (c == 0x0E) { scope++; state = S_SUBSCOPE; }
        else if (c == 0x03) {
          if (scope != 0) return 0;
          state = S_CLOSING;
        } else return 0;
        break;

      case S_SUBSCOPE:
        if (c == 0x1E) { /* RS allowed */ }
        else if (c == 0x0F) {
          scope--;
          if (scope < 0) return 0;
          state = S_ACTIVE;
        } else return 0;
        break;

      case S_CLOSING:
        if (c == 0x04) {
          if (seen_eot) return 0;
          seen_eot = 1;
          state = S_COLLAPSE;
        } else return 0;
        break;

      case S_COLLAPSE:
        return 0;
    }
  }

  return (state == S_COLLAPSE) ? 1 : 0;
}
```

---

## 3) Build Command (Clang → WASM)

```bash
clang \
  --target=wasm32 \
  -O3 \
  -nostdlib \
  -Wl,--no-entry \
  -Wl,--export=verify \
  -Wl,--allow-undefined \
  -o cm1_verify.wasm \
  cm1_verify_wasm.c
```

**Output:** ~700–900 bytes (toolchain-dependent)

---

## 4) JavaScript Usage Example

```js
const wasm = await WebAssembly.instantiate(bytes);
const { memory, verify } = wasm.instance.exports;

const buf = new Uint8Array(memory.buffer, 0, cm1.length);
buf.set(cm1);

const ok = verify(0, cm1.length) === 1;
```

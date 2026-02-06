# CM-1 Verifier CLI (Reference)

The CM-1 verifier CLI is the canonical, deterministic gate for CM-1 control
streams. It accepts only the frozen CM-1 safe alphabet and returns one of the
three allowed JSON result envelopes.

## Build

```bash
rustc cm1-verify.rs -O
```

## Usage

```bash
./cm1-verify <path>
```

## Output Envelopes

### PASS

```json
{"@cm1":"PASS"}
```

### FAIL

```json
{"@cm1":"FAIL"}
```

### ILLEGAL

```json
{"@cm1":"ILLEGAL","offset":412,"symbol":"U+001B"}
```

`ILLEGAL` is emitted only when an input byte is outside the CM-1 safe
alphabet. Structural violations inside the safe alphabet resolve to `FAIL`.

## Exit Codes

- `0` → PASS
- `1` → FAIL
- `64` → usage error
- `65` → illegal input byte
- `66` → file read error

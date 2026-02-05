# π-GCCP Reference Engine v1 (NORMATIVE)

This is the **smallest lawful π-GCCP engine**.

---

## Execution Model

```text
CPU = authority
GPU = accelerator only
Proof = mandatory
Approximation = forbidden
```

---

## Core Loop (pseudocode)

```pseudo
load SCXQ2.pi-signal lane
verify hash
normalize phase
apply invariant (half-turn | mobius | saddle)
compute exact collapse
emit proof object
return scalar + proof
```

---

## GPU Optional Path

```text
WGSL kernels allowed ONLY IF:
- CPU result == GPU result bit-for-bit
- CPU result stored as ground truth
```

GPU failure → fallback, not error.

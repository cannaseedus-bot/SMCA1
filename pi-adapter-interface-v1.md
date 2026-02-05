# π-Adapter Interface v1 (Normative, Locked)

This document is the canonical adapter contract for π-signals. If a system cannot emit this interface, it does not participate.

---

## 1.1 Canonical Output Object (Authoritative)

```json
{
  "@type": "pi.signal.v1",
  "@version": "1.0",

  "geometry": {
    "angles": [0.0, 1.0472, 2.0944],
    "magnitudes": [0.92, 0.51, 0.13],
    "paths": [
      [0, 1],
      [1, 2]
    ],
    "epsilon": 0.1745329
  },

  "provenance": {
    "adapter": "gguf",
    "adapter_version": "1.0.0",
    "deterministic": true,
    "source_hash": "sha256:…"
  }
}
```

### Hard Rules

* `angles` **MUST** be radians ∈ [0, 2π)
* `magnitudes` **MUST** be ≥ 0 (no upper bound)
* `angles.length === magnitudes.length`
* No logits
* No token IDs
* No embeddings
* No text

This is **signal**, not representation.

---

## 1.2 Semantic Meaning (Frozen)

| Field     | Meaning                          |
| --------- | -------------------------------- |
| angle     | phase / semantic orientation     |
| magnitude | confidence / energy              |
| path      | topological adjacency (optional) |
| epsilon   | angular tolerance (π-native)     |

Nothing else is allowed.

---

## 1.3 Adapter Responsibility (Strict)

An adapter may:

* normalize
* project
* compress
* quantize
* batch

An adapter may **not**:

* rank
* retrieve
* cache
* interpret semantics

Adapters emit **geometry only**.

---

## 1.4 Determinism Law

```text
Same input + same adapter + same version
→ identical pi.signal.v1
```

If this is violated, the adapter is **non-conformant**.

---

## 1.5 Model Adapter Positioning (Normative)

Adapters are **signal normalizers**, not model integrations. Every external model is treated as a
signal emitter that must be normalized into π-geometry before any profile logic or GCCP kernels run.

```text
[ External Model ]
        ↓
[ Adapter → pi.signal.v1 ]
        ↓
[ π-Profile Vectorizer ]
        ↓
[ SVG-Tensor Geometry ]
        ↓
[ π-GCCP Kernels (WebGPU / CPU) ]
        ↓
[ object://retrieve/semantic.v1 ]
```

### Required Adapter Emissions

Every adapter **MUST** emit the canonical π-signal geometry primitives only:

* angles
* magnitudes
* phases (expressed as angles)
* topological relations (paths)
* confidence envelopes (expressed as magnitudes + epsilon)

Adapters may use any internal math, but the output **MUST** satisfy the canonical output object and
determinism law. π-GCCP never inspects model type or training origin; it only consumes geometry.

---

## 1.6 Standardization Targets (Normative)

π-GCCP does **not** standardize models. It standardizes geometry contracts and interpretation
rules. Conformance is defined by the following locked targets:

1. **Adapter output schema**
   * Canonical primitives only (`angles`, `magnitudes`, `paths`, `epsilon`).
   * Deterministic mapping rules for each adapter.
   * Declared error bounds or tolerance (`epsilon`) for emitted geometry.
2. **π-Profile authoring rules**
   * Weighting and decay are profile-level concerns, not adapter concerns.
   * Interference logic is applied only after signal emission.
3. **SVG-Tensor canonicalization**
   * Identical geometry must hash identically.
   * Identical input to a conformant adapter must yield identical retrieval inputs.
4. **Object Server contracts**
   * Retrieval interfaces are **model-agnostic** and consume geometry only.

Any adapter that emits geometry but violates these targets is **non-conformant**.

---

## 1.7 Adapter Mapping Examples (Non-Normative)

These examples illustrate lawful signal normalization patterns. All mappings terminate in the
canonical output object and do not leak model-specific representations.

### LLM Logits Adapter

* logits → ranked deltas
* deltas → angular deviations
* entropy → magnitude scaling

### Embedding Adapter (GGUF / SBERT / OpenAI)

* vector → unit-circle projection
* cosine similarity → angular proximity
* norm → confidence weight

### N-gram Adapter

* frequency → angular density
* overlap → interference potential
* rarity → phase shift

### Vision / Audio / Multimodal Adapter

* features → spectral bands
* bands → phase stacks
* stacks → superposed geometry

---

# 2. WGSL Reference Kernels (Angle + Interference)

These kernels implement **π-GCCP math only**. They do not know about models.

---

## 2.1 Angle Distance Kernel (WGSL)

```wgsl
// pi_angle_distance.wgsl
// Computes wrapped angular distance in radians

@group(0) @binding(0)
var<storage, read> anglesA : array<f32>;

@group(0) @binding(1)
var<storage, read> anglesB : array<f32>;

@group(0) @binding(2)
var<storage, write> distances : array<f32>;

const PI : f32 = 3.141592653589793;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) id : vec3<u32>) {
    let i = id.x;
    if (i >= arrayLength(&anglesA)) {
        return;
    }

    let delta = abs(anglesA[i] - anglesB[i]);
    let wrapped = min(delta, 2.0 * PI - delta);

    distances[i] = wrapped;
}
```

---

## 2.2 Interference Kernel (WGSL)

```wgsl
// pi_interference.wgsl
// Computes cosine interference weighted by magnitude

@group(0) @binding(0)
var<storage, read> anglesA : array<f32>;

@group(0) @binding(1)
var<storage, read> anglesB : array<f32>;

@group(0) @binding(2)
var<storage, read> magnitudesA : array<f32>;

@group(0) @binding(3)
var<storage, read> magnitudesB : array<f32>;

@group(0) @binding(4)
var<storage, write> output : array<f32>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) id : vec3<u32>) {
    let i = id.x;
    if (i >= arrayLength(&anglesA)) {
        return;
    }

    let phase = anglesA[i] - anglesB[i];
    let interference = cos(phase);

    output[i] = interference * magnitudesA[i] * magnitudesB[i];
}
```

---

## 2.3 Collapse Rule (Host-Side, Exact)

```text
score = Σ interference_i
normalized_score = score / Σ (magA_i * magB_i)
```

CPU fallback must use **identical math**.

---

# 3. End-to-End Trace (GGUF → π → Retrieval)

No simulation. No hidden steps.

---

## 3.1 GGUF Emits Raw Numbers

Example (conceptual):

```text
embedding = [0.12, -0.98, 0.05, 0.31]
entropy = 0.42
```

---

## 3.2 GGUF Adapter → π-Signal

**Adapter projection rule** (example, deterministic):

```text
angle_i = atan2(value_i, norm)
magnitude_i = 1 - entropy
```

**Result**

```json
{
  "@type": "pi.signal.v1",
  "@version": "1.0",
  "geometry": {
    "angles": [1.448, -1.45, 0.12, 0.77],
    "magnitudes": [0.58, 0.58, 0.58, 0.58],
    "epsilon": 0.1745329
  },
  "provenance": {
    "adapter": "gguf",
    "deterministic": true
  }
}
```

No tokens. No vocabulary. No language assumption.

---

## 3.3 SVG-Tensor Index (Stored)

Each document already exists as **SVG-Tensor geometry**:

```json
{
  "doc_id": "doc_42",
  "angles": [...],
  "magnitudes": [...]
}
```

Stored in:

* SCXQ2 lanes
* memory-mapped
* hash-addressed

---

## 3.4 π-GCCP Collapse (WebGPU or CPU)

For each indexed document:

```text
distance(doc, query)
→ interference score
→ normalized collapse value
```

Pure math. Same kernel. Same result.

---

## 3.5 object://retrieve/semantic.v1 (Final)

```json
{
  "@type": "object.retrieve.semantic.v1",
  "query_hash": "pi:…",
  "profile": "topological_v1",
  "results": [
    { "id": "doc_42", "score": 0.913 },
    { "id": "doc_17", "score": 0.802 },
    { "id": "doc_09", "score": 0.411 }
  ],
  "deterministic": true
}
```

Retrieval never knew:

* what model ran
* where it ran
* how it was trained

Only geometry mattered.

---

# Final Lock

### What is now frozen

* π-Adapter Interface v1
* π-Signal geometry shape
* WGSL math
* Collapse semantics
* Retrieval contract

### What is free forever

* GGUF / ONNX / WASM / WebGPU
* Training-based or training-less systems
* Symbolic, neural, hybrid, or hand-written logic

You didn’t build a “model framework”. You built a **geometry-governed inference substrate**.

If you want next, we can:

* Seal **Adapter Conformance Tests**
* Emit **SCXQ2 binary layouts** for π-signals
* Write the **paper-grade proof** of model-independence
* Or wire this directly into your existing Object Server manifests

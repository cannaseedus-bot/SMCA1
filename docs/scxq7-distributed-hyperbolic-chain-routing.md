# Distributed Hyperbolic Chain Routing (DHC-R)

Defines a routing layer that treats SCXQ7 objects and π-GCCP signals as nodes
in a hyperbolic manifold, allowing logarithmic fan-out and stable multi-hop
retrieval while preserving SCO/1 sovereignty.

## 1) Topology Model

```
@dhc_r.topology
  space: "hyperbolic"
  curvature: negative
  embedding: "π-gccp geometry → hyperbolic coordinates"
  nodes: "SCO/1 objects + object servers"
  edges: "causal adjacency + retrieval affinity"
```

**Key property:** hyperbolic distance grows logarithmically with node degree,
supporting large-scale routing without central coordination.

## 2) Routing Objective

```
@dhc_r.objective
  minimize: "geodesic hops"
  preserve: "domain legality"
  verify: "SCO/1 boundaries"
  avoid: "authority leakage"
```

Each hop must remain projection-only: data can be projected, never executed,
unless the destination SCO/1 explicitly accepts execution within its kernel.

## 3) Chain Assembly (Deterministic)

```
@dhc_r.chain
  input: "π-GCCP signal"
  step1: "embed → hyperbolic coordinate"
  step2: "select candidate nodes by geodesic proximity"
  step3: "filter by domain legality + projection-only rule"
  step4: "order into a deterministic chain"
  output: "route = [node_1, node_2, ... node_k]"
```

**Determinism rule:** given identical signal geometry and registry state, the
chain is identical.

## 4) Causal Safety (SCO/1 Compliant)

```
@dhc_r.safety
  authority: "internal only"
  projection: "read-only"
  side_effects: "forbidden"
  verifier: "local only"
```

Routing never grants execution authority. Only projection of outputs across
the chain is allowed.

---

# Temporal Saddle Decay (Memory)

Defines a bounded memory decay function for inference traces and symbolic
states. The decay preserves causal consistency while preventing unbounded
retention in long-running SCO/1 objects.

## 1) Decay Law

```
@tsd.decay
  base: "saddle"
  function: "exp(-α t^2) + β / (1 + t)"
  constraints:
    - "never erase kernel-relevant state"
    - "never delete hash-chain anchors"
```

* `t` is elapsed logical time in trace steps.
* `α` controls sharpness of the saddle.
* `β` preserves a low-energy long tail for weak recall.

## 2) Memory Classes

```
@tsd.memory.classes
  kernel: "immutable"
  law: "immutable"
  trace_anchors: "immutable"
  state: "decayable"
  caches: "decayable"
```

Only `state` and `caches` decay. Kernel and trace anchors are permanent.

## 3) Replay Guarantee

```
@tsd.replay
  rule: "trace anchors + deltas remain sufficient"
  effect: "replay possible despite decay"
```

The decay function may compress or drop low-energy state, but trace anchors
must guarantee deterministic replay of legal transitions.

---

# On-Device SCXQ2 Verifier (No Network)

Defines a verifier profile for offline, on-device validation of SCXQ2 payloads
without external registries or network calls.

## 1) Offline Constraints

```
@scxq2.offline.verifier
  network: "disabled"
  registry: "local snapshot only"
  attestations: "embedded or bundled"
  time: "monotonic local"
```

## 2) Verification Steps

```
@scxq2.offline.flow
  step1: "parse SCXQ2 symbol stream"
  step2: "validate symbol closure"
  step3: "verify domain legality"
  step4: "check persistence declarations"
  step5: "emit PASS/FAIL only"
```

## 3) Result Contract

```
@scxq2.offline.result
  pass: "SCXQ2 Verified (offline)"
  fail: "Non-Compliant SCXQ2"
```

No warnings, no partial credit. The device must never consult the network
for authority.

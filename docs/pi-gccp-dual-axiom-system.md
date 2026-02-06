# π-GCCP Dual-Axiom System (Declarative vs Executable Law)

## 1. Why two axiom languages exist

This mirrors mathematics and physics more closely than most software stacks:

| Domain               | Role                     |
| -------------------- | ------------------------ |
| Euclid / ZFC         | Declares what is true    |
| Calculus / Operators | Executes transformations |

The system already separates these concerns — it just needed a name.

---

## 2. TOML = Axiom Declaration Layer (Loadable, Non-Executable)

**Purpose**
Defines *what is lawful* — never *how* it runs.

### Properties

* Static
* Cacheable
* Hashable
* Network-distributable
* Human-auditable
* Zero side effects

### What TOML axioms define

* Invariants
* Bounds
* Domains
* Capabilities
* Allowed operators
* Constants (π, thresholds, limits)

### Example: `pi-gccp.axioms.toml`

```toml
[axiom.schema]
name = "pi-gccp"
version = "1.0.0"
authority = "pi-gccp.org"

[axiom.constants]
pi = 3.141592653589793
half_turn = 1.5707963267948966
quarter_turn = 0.7853981633974483

[axiom.bounds]
epsilon_min = 0.0
epsilon_max = "pi"
interference_min = 0.0
interference_max = "pi"

[axiom.geometry]
space = "unit_circle"
metric = "angular"
closure = true

[axiom.collapse]
output_range = [-1.0, 1.0]
deterministic = true
branching = false

[axiom.superposition]
linear = true
weight_sum = 1.0
max_terms = 16

[axiom.time]
explicit = true
implicit_state = false

[axiom.adapters]
external_authority = false
emit_mode = "signal_only"

[axiom.compression]
identity_preserving = true
proof_required = true
```

### What the system does with TOML

* Load once
* Hash it
* Treat it as **law**
* Reject anything that violates it

No execution. No order. No side effects.

---

## 3. Hard invariants (non-negotiable)

These are the rules that make the separation *provable* instead of merely configurable.

### Invariant A — TOML is inert

TOML **MUST NOT**:

* reference phases
* reference order
* reference execution targets
* reference kernels
* emit values
* depend on runtime state

TOML may only **constrain**. If TOML can “do” anything, the system is broken.

### Invariant B — XML is subordinate

XML **MUST**:

* name its TOML authority explicitly
* fail closed if axioms are missing or invalid
* never redefine constants
* never widen bounds
* never introduce new domains

XML may *apply* π. XML may not *define* π.

### Invariant C — One-way authority

```
TOML ─┬─▶ XML ─┬─▶ Kernels ─┬─▶ SCXQ2
      │        │           │
      └───X────┴────X──────┴── (no back edges)
```

There are **no feedback edges**. If execution can influence axioms, you have built a simulator, not a law-based system.

---

## 4. XML = Executable Axiom Plane (Ordered, Phase-Aware)

**Purpose**
Defines *how lawful transformations occur* — but only within TOML bounds.

### Properties

* Ordered
* Phase-explicit
* Deterministic
* Interpretable
* Replayable
* Side-effect constrained

### XML is not “logic” — it’s axiom application

### Example: `pi-gccp.runtime.xml`

```xml
<pi-gccp-runtime xmlns="urn:pi-gccp:runtime:v1">

  <axioms source="pi-gccp.axioms.toml" />

  <phase name="Pop">
    <load object="svg-tensor.index" />
  </phase>

  <phase name="Wo">
    <vectorize method="topological" dimensionality="2" />
  </phase>

  <phase name="Sek">
    <kernel type="pi-angle-collapse"
            epsilon="0.1745329"
            interference="1.5707963" />
  </phase>

  <phase name="Collapse">
    <reduce mode="scalar" />
    <emit target="object://retrieve/semantic.v1" />
  </phase>

</pi-gccp-runtime>
```

### What XML can do

* Order execution
* Bind kernels
* Apply π constants
* Route signals
* Emit collapse results

### What XML cannot do

* Change axioms
* Invent constants
* Override bounds
* Introduce hidden state

XML executes axioms; it does not define them.

---

## 4. Cluster invocation rule (federated execution, not leaked authority)

**Both TOML and XML may reference clusters, but only XML may invoke them.**

### TOML: cluster declaration only (non-executing)

TOML is allowed to name **which clusters are lawful to use**, but never *when* or *how* they run. This makes clusters part of the axiomatic universe, not active agents.

```toml
[axiom.clusters]

[axiom.clusters.python]
type = "external"
mode = "pure"
side_effects = false
deterministic = true
allowed_kernels = ["fft_mul", "bigint"]

[axiom.clusters.kuhul]
type = "native"
mode = "symbolic"
side_effects = false
deterministic = true
allowed_domains = ["scxq2", "geometry", "proof"]
```

**TOML can:**
* Declare which clusters may exist.
* Declare constraints they must obey.

**TOML cannot:**
* Spawn
* Call
* Schedule
* Stream
* Mutate

### XML: cluster invocation (ordered, phase-bound)

XML is where clusters are used, but only inside declared axioms.

```xml
<phase name="Sek">
  <cluster-call
      target="python"
      kernel="fft_mul"
      inputs="scxq2.block.A, scxq2.block.B"
      output="scxq2.block.C"
  />
</phase>
```

```xml
<phase name="Sek">
  <cluster-call
      target="kuhul"
      kernel="proof-collapse"
      domain="geometry"
  />
</phase>
```

**Key constraints**
* XML may only call clusters named in TOML.
* Kernel names must be whitelisted by TOML.
* Outputs re-enter the system as data, not authority.
* No cluster may call back into XML or TOML.

**Authority boundary**

```
TOML  → declares lawful clusters
XML   → schedules lawful calls
BOT   → executes math / proof
SCXQ2 → proves identity preserved
```

There is **no upward flow**. Clusters return results only, never decisions.

**One-line lock**

> **Clusters may execute computation, but only axioms may grant permission, and only XML may request it.**

---

## 5. Formal separation of authority

| Layer      | Authority             |
| ---------- | --------------------- |
| TOML       | What is true          |
| XML        | How truth is applied  |
| WGSL / CPU | How math is executed  |
| SCXQ2      | Proof of preservation |

This prevents semantic drift.

---

## 6. Where schemas fit now

JSON Schemas become **validation axioms**, subordinate to TOML:

```
TOML axioms
  ↓
JSON Schemas (structural legality)
  ↓
XML runtime (execution order)
  ↓
WGSL / CPU kernels (math)
  ↓
SCXQ2 proof (verification)
```

This is a complete logical stack.

---

## 7. Why this matters

Because now:

* You can change execution without changing truth
* You can prove correctness offline
* You can run headless
* You can swap runtimes
* You can ship axioms over DNS
* You can audit the system like a constitution

Most AI systems cannot do this because they mix axioms and execution. π-GCCP does not.

---

## 8. The one-line law

> **TOML declares what exists.**  
> **XML declares how it unfolds.**

That is formal system design, not style.

---

## 9. Next formalizations

If needed, extend with:

* A frozen π-GCCP axiom canon
* Illegal universes (what cannot be expressed)
* A machine-verifiable axiom hash
* Gödel boundaries of the system

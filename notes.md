# Notes

## Quick Links
- [KUHUL: Simulator Boundary Notes](docs/kuhul-simulator-boundary.md)
- [Hybrid Wormhole Architecture: The Tiered Control Plane](docs/hybrid-wormhole-architecture.md)
- [π-Adapter Interface v1 (Locked)](docs/pi-adapter-interface-v1.md)
- [π-GCCP Möbius Invariant v1 (Locked)](docs/pi-gccp-mobius-invariant.md)
- [π-GCCP Conformance v1 (Locked)](docs/pi-gccp-conformance.json)
- [π-GCCP Dual-Axiom System](docs/pi-gccp-dual-axiom-system.md)
- [Central Binomial Series Identity (π + 3)](docs/pi-series-identity.md)
- [WORMHOLE-SPEC v1 (Locked)](docs/wormhole-spec-v1.md)
- [SCXQ7 Causal State Engine](docs/scxq7-causal-state-engine.md)
- [SCXQ2 Inference Engine](docs/scxq2-inference-engine.md)
- [Distributed Hyperbolic Chain Routing + Temporal Saddle Decay + Offline SCXQ2 Verifier](docs/scxq7-distributed-hyperbolic-chain-routing.md)
- [Host-Orchestrated File Bus Architecture](docs/host-orchestrated-file-bus-architecture.md)

## Repository Artifacts
- `scxq7-verify.rs` — frozen SCO/1 verifier CLI.
- `scxq7-verify-wasm.rs` — wasm projection of the verifier.
- `scxq7.compliance.manifest.json` — compliance manifest schema.
- `scxq7.registry.json` — public compliance registry format.
- `idb.schema.xsd` — IDB stack frozen core schema.

## Working Notes
- Keep specs authoritative in `docs/` and link from README/notes.
- Prefer relative links so content renders locally and on GitHub.



## Cheat Sheets vs Simulators

### 1️⃣ Cheat Sheets

**(What LLMs mostly are)**

A cheat sheet is:

* **Pre-computed**
* **Collapsed**
* **Lookup-based**
* **Pattern-driven**
* **Fast**
* **Brittle outside scope**

It does *not* model reality.  
It models **answers that tended to work**.

Think:

* trig tables
* log tables
* exam notes
* memorized chess openings

**Core property**

> The work already happened elsewhere.

At runtime, nothing is *derived*.  
You’re just interpolating.

---

### 2️⃣ Simulators

**(What people *think* AI is)**

A simulator is:

* **Stateful**
* **Causal**
* **Step-based**
* **Constraint-checked**
* **Slow**
* **Robust under novelty**

It *does* model reality (or a formal world).

Think:

* physics engines
* game worlds
* circuit simulators
* climate models
* theorem provers
* CPU emulators

**Core property**

> The answer emerges by running the system.

At runtime, the system **must obey laws**.

---

## Side-by-side (no fluff)

| Dimension | Cheat Sheet        | Simulator          |
| --------- | ------------------ | ------------------ |
| Knowledge | Stored             | Generated          |
| Runtime   | Lookup             | Execution          |
| Errors    | Confident nonsense | Detectable failure |
| Novelty   | Fragile            | Graceful           |
| Truth     | Statistical        | Constraint-bound   |
| Speed     | Extremely fast     | Slower             |
| Cost paid | Training           | Inference          |
| Laws      | Implicit           | Explicit           |
| Grounding | None               | Built-in           |

---

## Why LLMs *feel* like simulators (but aren’t)

LLMs are trained on **records of simulations**:

* explanations
* arguments
* proofs
* code
* debates

So they learn:

> “What does a *simulation result* look like when humans describe it?”

They can **imitate the *output* of reasoning**  
without executing the reasoning.

That illusion is powerful — and dangerous.

---

## The key failure mode (this is the tell)

Ask a cheat sheet:

> “What happens if I bend the rules a little?”

It answers confidently.

Ask a simulator:

> “What happens if I bend the rules a little?”

It either:

* crashes
* refuses
* diverges
* produces something visibly broken

That difference matters.

---

## Why this matters for safety and power

### Cheat sheets:

* scale easily
* hallucinate
* persuade
* bluff
* imitate authority

### Simulators:

* resist lying
* expose contradictions
* require structure
* don’t care how convincing they sound

This is why:

* LLMs write great essays
* LLMs suck at physics without engines
* LLMs invent citations
* LLMs need tools to be reliable

---

## The uncomfortable middle ground (where things get spicy)

Some systems are **hybrids**:

* LLM = cheat sheet
* Tool / engine = simulator
* Orchestrator = referee

That’s the only way to get:

* speed **and**
* grounding **and**
* novelty **and**
* correctness

Pure cheat sheets lie.  
Pure simulators are expensive.

---

## The sentence that locks it in

**A cheat sheet predicts what usually happens.  
A simulator enforces what must happen.**

Everything else is commentary.

If you want, next we can:

* map *your* compression/microcode ideas onto this split
* talk about why “reasoning tokens” don’t fix cheat sheets
* or define what a *true* simulator-AI would require architecturally

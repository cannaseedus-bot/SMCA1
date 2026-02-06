# SCXQ7: Stateful Causal Engine (SCXQ7-CSE/1)

This document captures the SCXQ7 causal-state extension: a stepwise, validated execution model that preserves causality, state integrity, and novelty-safe constraint expansion.

## Kernel Extension: Causality Laws

```
@π scxq7.kernel.extended
  @law_4: STATEFUL EXECUTION
    "All execution proceeds from explicit state"
    "State transitions are causal steps"
    "Each step must validate before proceeding"
    "Novel inputs require validation, not rejection"

  @law_5: CAUSAL INTEGRITY
    "Effects follow verifiable causes"
    "Causal chains are preserved through compression"
    "State modifications trace to input events"
    "No acausal state transitions"

  @law_6: STEPWISE VALIDATION
    "Execution proceeds in validated steps"
    "Each step checks constraints before committing"
    "Backtracking is explicit, not implicit"
    "Validation failures preserve prior state"
```

## Causal State Engine

```
@π causal.state.engine
  @state.representation
    State {
      current: CompressedHeap,
      history: CausalDAG,
      constraints: ConstraintSet,
      validation: ValidationCache,
    }

  @execution.loop
    loop {
      input = await_next_event()
      validation = validate(input, state.constraints)

      if (is_novel(input)) {
        state.constraints = safely_expand(state.constraints, input)
      }

      candidate = apply_causally(state.current, input)

      if (validate_state(candidate, state.constraints)) {
        state.history.append(CausalStep {
          cause: input,
          effect: candidate,
          validation: validation,
        })
        state.current = candidate
        project_state_update(state.current)
      } else {
        handle_constraint_violation(input, state.current)
      }
    }
```

## Constraint System

```
@π constraint.system
  @constraint.types
    1. Structural: "SCXQ7 kernel laws (immutable)"
    2. Domain: "Execution context boundaries (.m, .π, etc.)"
    3. State: "Invariants that must hold"
    4. Causal: "Allowed cause-effect patterns"
    5. Novelty: "Patterns for handling the unexpected"

  @novelty.handling
    @step1: "Detect novelty (pattern mismatch)"
    @step2: "Isolate novel input"
    @step3: "Check against higher-order constraints"
    @step4: "If safe, expand constraint set"
    @step5: "Learn new pattern for future"
```

## Step-Based Execution Model

```
@π stepwise.execution
  @step.anatomy
    Step {
      id: CausalHash,
      input: CompressedEvent,
      validation: Proof,
      computation: MicroSteps,
      output: StateDelta,
      next_constraints: Set,
      duration: TimeBound,
    }

  @enforced.slowness
    min_step_time = 1ms
    max_steps_per_second = 100

  @step.lifecycle
    1. RECEIVE
    2. VALIDATE
    3. COMPUTE
    4. VERIFY
    5. COMMIT
    6. PROJECT
    7. PAUSE
```

## Causal DAG (Traceability)

```
@π causal.dag
  Node {
    state_hash: Hash,
    parent: Option<NodeId>,
    cause: EventHash,
    effect: StateDelta,
    validation_proof: Proof,
    constraints_active: ConstraintSet,
    timestamp: CausalTime,
  }

  @properties
    get_cause(state) → event
    get_effect(event) → state
    trace_path(current, target) → causal chain

  @recovery
    1. Find last valid state in DAG
    2. Replay causal steps from there
    3. Skip/remediate invalid steps
    4. Continue with preserved causality
```

## Implementation Strategy (Host Reference)

```
@π implement.causal.scxq7
  @additions
    1. Causal DAG storage (append-only)
    2. Constraint checking engine
    3. Novelty detection system
    4. Step timing enforcement
    5. State validation proofs

  @performance
    steps_per_second: 10-100 (enforced slow)
    state_size: compressed, versioned
    memory: proportional to causal history
    recovery_time: O(log steps) via DAG
```

## Tri-Modal Equivalence (OS / DB / Ledger)

SCXQ7-CSE is not merely analogous to an operating system, database engine, or ledger—it is structurally equivalent to all three simultaneously. Each step is a verified syscall, a committed transaction, and an auditable ledger entry.

### SCXQ7 ⇄ Operating System Kernel

| OS Kernel Concept | SCXQ7 Construct                     |
| ----------------- | ----------------------------------- |
| Process state     | `State.current (CompressedHeap)`    |
| Syscall           | `input = await_next_event()`        |
| Kernel mode       | `@law_* (immutable kernel laws)`    |
| User mode         | `.s7 domain execution`              |
| Scheduler         | `enforced step timing`              |
| Context switch    | `Step.lifecycle (RECEIVE → COMMIT)` |
| Fault handling    | `handle_constraint_violation()`     |
| Kernel panic      | **Impossible** (state preserved)    |
| Replay / resume   | `causal_dag.replay()`               |

**Key equivalence:** SCXQ7 is an OS kernel where every instruction is a verified syscall, and illegal transitions do not commit.

### SCXQ7 ⇄ Database Engine (ACID+++)

| DB Property     | SCXQ7 Mechanism                      |
| --------------- | ------------------------------------ |
| Atomicity       | Step commit is all-or-nothing        |
| Consistency     | Constraint validation before & after |
| Isolation       | Single-step execution, no races      |
| Durability      | Append-only Causal DAG               |
| Transactions    | `Step`                               |
| Write-ahead log | `CausalDAG`                          |
| Snapshot        | `State.current`                      |
| Rollback        | Replay from last valid node          |
| Schema          | ConstraintSet                        |
| Migration       | Safe constraint expansion            |

**Key equivalence:** Every SCXQ7 step is a transaction with a proof attached—ACID with explicit causality.

### SCXQ7 ⇄ Cryptographic / Financial Ledger

| Ledger Concept          | SCXQ7 Mechanism         |
| ----------------------- | ----------------------- |
| Block                   | `CausalStep`            |
| Chain                   | `CausalDAG`             |
| Transaction             | `input + state_delta`   |
| Validation              | `Proof`                 |
| Consensus               | Constraint satisfaction |
| Finality                | Commit                  |
| Fork resolution         | DAG replay rules        |
| Audit trail             | `trace_path()`          |
| Double-spend prevention | State invariants        |

**Critical insight:** SCXQ7 does not require external consensus because there is a single lawful executor and time is logical, not wall-clock. It behaves as a sovereign ledger with deterministic finality.

### The Unification

All three systems answer the same question: **Is this state change allowed, and can we prove it later?** SCXQ7 answers that question at every step, making execution, state, and history inseparable.

> **Formal definition:** SCXQ7 is a sovereign causal kernel that unifies operating system execution, database transactionality, and ledger-grade auditability into a single stepwise state machine.

## Manifesto (Causal, Stateful, Slow)

```
@π scxq7.causal.manifesto
  "We reject fast and fragile"
  "We embrace slow and robust"

  @principles
    1. Stateful: execution proceeds from explicit state
    2. Causal: effects trace to verifiable causes
    3. Step-based: progress in validated, committed steps
    4. Constraint-checked: everything verified before commit
    5. Slow: deliberate pacing enables correctness
    6. Robust under novelty: new patterns handled gracefully
```

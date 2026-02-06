# SCXQ7: Stateful Causal Engine (SCXQ7-CSE/1)

This document captures the SCXQ7 causal-state extension: a stepwise, validated
execution model that preserves causality, state integrity, and novelty-safe
constraint expansion.

## Kernel Extension: Causality Laws

```
@π scxq7.kernel.extended
  # Extend the 256-byte kernel with causality laws

  @law_0: NO IMPLICIT TRANSITION (64 bytes)
    "No state change may occur without an explicit causal step"
    "Computation does not imply mutation"
    "Proposal is not commitment"
    "Hidden transitions are illegal"

  @law_4: STATEFUL EXECUTION (64 bytes)
    "All execution proceeds from explicit state"
    "State transitions are causal steps"
    "Each step must validate before proceeding"
    "Novel inputs require validation, not rejection"

  @law_5: CAUSAL INTEGRITY (64 bytes)
    "Effects follow verifiable causes"
    "Causal chains are preserved through compression"
    "State modifications trace to input events"
    "No acausal state transitions"

  @law_6: STEPWISE VALIDATION (64 bytes)
    "Execution proceeds in validated steps"
    "Each step checks constraints before committing"
    "Backtracking is explicit, not implicit"
    "Validation failures preserve prior state"
```

## The Causal State Machine

```
@π causal.state.engine
  # The engine that makes SCXQ7 stateful and causal

  @state.representation
    # Compressed, causal, versioned
    State {
      current: CompressedHeap,      # Current state (SCXQ2 symbols)
      history: CausalDAG,           # Step-by-step history
      constraints: ConstraintSet,   # What's allowed
      validation: ValidationCache,  # What's already validated
    }

  @execution.loop
    loop {
      // 1. Wait for input (slow, deliberate)
      input = await_next_event()

      // 2. Validate against constraints (step-based)
      validation = validate(input, state.constraints)

      // 3. If novel, expand constraints (robust under novelty)
      if (is_novel(input)) {
        state.constraints = safely_expand(state.constraints, input)
      }

      // 4. Generate candidate next state (causal)
      candidate = apply_causally(state.current, input)

      // 5. Validate candidate (constraint-checked)
      if (validate_state(candidate, state.constraints)) {
        // 6. Commit as new causal step (stateful)
        state.history.append(CausalStep {
          cause: input,
          effect: candidate,
          validation: validation,
        })
        state.current = candidate

        // 7. Project changes (slow broadcast)
        project_state_update(state.current)
      } else {
        // 8. Fail gracefully, preserve state (robust)
        handle_constraint_violation(input, state.current)
      }
    }

  @properties
    speed: "slow_deliberate"        # Checks everything
    robustness: "high"              # Never corrupts state
    novelty: "tolerant"             # Learns new patterns
    causality: "preserved"          # Always traceable
```

## Constraint System

```
@π constraint.system
  # What makes SCXQ7 robust under novelty

  @constraint.types
    1. Structural: "SCXQ7 kernel laws (immutable)"
    2. Domain: "Execution context boundaries (.m, .π, etc.)"
    3. State: "Invariants that must hold"
    4. Causal: "Allowed cause-effect patterns"
    5. Novelty: "Patterns for handling the unexpected"

  @novelty.handling
    // When encountering something new:
    @step1: "Detect novelty (pattern mismatch)"
    @step2: "Isolate novel input"
    @step3: "Check against higher-order constraints"
    @step4: "If safe, expand constraint set"
    @step5: "Learn new pattern for future"

  @example
    // Starting constraint: "Numbers can be added"
    constraint = can_add(number, number)

    // Novel input: Add string to number
    input = apply("+", 5, "hello")

    // Process:
    1. Detect novelty: string not in number constraint
    2. Isolate: (string "hello") is novel
    3. Check higher-order: Is coercion allowed? No.
    4. Don't expand constraints (unsafe)
    5. Return: constraint violation
    6. State preserved, error logged
```

## Step-Based Execution Model

```
@π stepwise.execution
  # Replacing "fast" with "verifiably correct"

  @step.anatomy
    Step {
      id: CausalHash,          // Unique step identifier
      input: CompressedEvent,  // What triggered this step
      validation: Proof,       // Proof of constraint satisfaction
      computation: MicroSteps, // The μ-ops executed
      output: StateDelta,      // Change to state
      next_constraints: Set,   // Constraints for next step
      duration: TimeBound,     // Deliberately slow (enforced)
    }

  @enforced.slowness
    // Not just slow, but "slow with purpose"
    min_step_time = 1ms  // Enforced minimum
    max_steps_per_second = 100 // Deliberate limit

    // Why?
    1. Time for validation
    2. Time for constraint checking
    3. Time for novelty detection
    4. Time for causal logging
    5. Prevents race conditions

  @step.lifecycle
    1. RECEIVE: Wait for input (async, bounded wait)
    2. VALIDATE: Check all constraints (thorough)
    3. COMPUTE: Execute μ-ops (monitored)
    4. VERIFY: Check output against constraints (again)
    5. COMMIT: Append to causal DAG (atomic)
    6. PROJECT: Broadcast changes (reliable)
    7. PAUSE: Enforce step timing (deliberate)
```

## Causal DAG for Traceability

```
@π causal.dag
  # Every state change is causally linked

  Node {
    state_hash: Hash,
    parent: Option<NodeId>,
    cause: EventHash,
    effect: StateDelta,
    validation_proof: Proof,
    constraints_active: ConstraintSet,
    timestamp: CausalTime,  // Logical, not wall-clock
  }

  @properties
    // Always traceable
    get_cause(state) → event
    get_effect(event) → state
    trace_path(current, target) → causal chain

    // Always consistent
    no_causal_loops
    no_parent_cycles
    no_acausal_links

  @recovery
    // From any corruption:
    1. Find last valid state in DAG
    2. Replay causal steps from there
    3. Skip/remediate invalid steps
    4. Continue with preserved causality
```

## Example: Robust Financial Analytics

```
@π finance.causal.s7
  // A stateful, causal, stepwise financial system

  @initial.state
    accounts: {checking: 1000, savings: 5000}
    constraints: [
      "balance ≥ 0",
      "transfer amount > 0",
      "from ≠ to"
    ]

  @step1: "Receive transfer request"
    input: transfer(from=checking, to=savings, amount=200)

    // Slow validation phase
    validate: [
      checking.balance ≥ 200? ✓
      amount > 0? ✓
      from ≠ to? ✓
      // Also check novel patterns:
      - Is 200 a suspicious amount? (novelty check)
      - Is this transfer frequency normal?
    ]

    // If all valid, proceed to step 2

  @step2: "Execute transfer"
    // Causal application
    old_state = current_state
    new_state = apply_transfer(old_state, input)

    // Verify post-conditions
    verify: [
      total_money_conserved? ✓
      no_account_negative? ✓
      constraints_preserved? ✓
    ]

    // Commit as causal step
    causal_dag.append(
      cause: input,
      effect: new_state,
      validation: validation_proof
    )

  @step3: "Project update"
    // Broadcast to all projections
    project_http(account_balances_updated)
    project_ws(real_time_update)

    // Enforce slowness
    sleep(min_step_time)

  @novelty.example
    // What if someone tries to transfer "hello" dollars?
    input: transfer(from=checking, to=savings, amount="hello")

    // Novelty detection:
    1. Pattern mismatch: amount expected number, got string
    2. Isolate novel part: "hello"
    3. Check higher constraints: No string-to-number coercion
    4. Don't expand constraints (would violate type safety)
    5. Return: constraint violation
    6. State unchanged, error logged for analysis
```

## Advantages of the Model

```
@π why.stateful.causal.slow
  @robustness
    "State never corrupted"
    "All changes validated"
    "Causal trace for debugging"
    "Recovery from any point"

  @novelty.tolerance
    "New patterns detected"
    "Safe expansion of constraints"
    "Learning without corruption"
    "Graceful degradation"

  @verifiability
    "Every step proven correct"
    "Causal chains auditable"
    "Constraints explicit"
    "No hidden state transitions"

  @tradeoffs.accepted
    speed: "sacrificed for correctness"
    complexity: "embraced for robustness"
    latency: "increased for validation"
    throughput: "limited for traceability"
```

## Implementation Strategy

```
@π implement.causal.scxq7
  // Extend the 10KB reference host

  @additions
    1. Causal DAG storage (append-only)
    2. Constraint checking engine
    3. Novelty detection system
    4. Step timing enforcement
    5. State validation proofs

  @size.estimate
    base_host: 8.7KB
    causal_engine: +3.2KB
    constraint_system: +2.1KB
    novelty_detection: +1.5KB
    total: ~15.5KB

  @performance
    steps_per_second: 10-100 (enforced slow)
    state_size: compressed, versioned
    memory: proportional to causal history
    recovery_time: O(log steps) via DAG

  @guarantees
    1. State integrity always preserved
    2. Causality always maintained
    3. Constraints always checked
    4. Novelty always detected
    5. Steps always validated
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
    1. **Stateful**: All execution proceeds from explicit state
    2. **Causal**: All effects trace to verifiable causes
    3. **Step-based**: Progress in validated, committed steps
    4. **Constraint-checked**: Everything verified before commit
    5. **Slow**: Deliberate pacing enables correctness
    6. **Robust under novelty**: New patterns handled gracefully

  @contrast
    Old world: "Move fast and break things"
    SCXQ7: "Move deliberately and prove things"

    Old world: "Stateless for scalability"
    SCXQ7: "Stateful for correctness"

    Old world: "Optimistic execution"
    SCXQ7: "Pessimistic validation"

  @result
    Systems that:
    - Never corrupt their state
    - Always know why they changed
    - Handle the unexpected gracefully
    - Can recover from any failure
    - Are provably correct at each step

  @cost
    Speed: 10-100x slower than "fast" systems
    Complexity: More explicit, more verifiable
    Development: More deliberate, more proven

  @worth.it
    "For systems that must never fail"
    "For data that must never corrupt"
    "For causality that must never break"
    "For robustness that must never degrade"
```

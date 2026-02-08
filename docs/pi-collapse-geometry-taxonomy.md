# π Collapse Geometry Taxonomy + Binary-Splitting Microkernel (SCXQ7)

This note classifies π formulas by **collapse geometry** (the symmetry being collapsed, the state space, and the kernel), then formalizes **binary splitting** as the canonical SCXQ7 microkernel for associative causal collapse.

---

## I. Classifying π formulas by collapse geometry

This is **not** a convergence-speed classification. It is a classification by:

- **State space**
- **Group action / symmetry**
- **Collapse rule**
- **Natural kernel**

### 1) Path / Tree Collapse (Combinatorial geometry)

**Canonical example**

\[
\sum_{k=1}^\infty k \cdot \frac{2^k}{\binom{2k}{k}} = \pi + 3
\]

**Geometry**

- Discrete lattice paths
- Dyck paths / Catalan structures
- Central binomial symmetry

**Collapse**

- Pairwise aggregation of subtrees
- Limit of balanced path counts

**Invariants**

- Locality
- Associativity
- Structural transparency
- Linear information density

**Natural kernel** → **Binary splitting**

**Collapse type** → **Tree collapse → scalar invariant**

π appears as an emergent invariant of **discrete symmetry**, not as a function value.

---

### 2) Angle / Rotation Collapse (Trigonometric geometry)

**Canonical examples**

Machin, Euler, Gauss arctan formulas

\[
\frac{\pi}{4} = \sum_i a_i \arctan(1/p_i)
\]

**Geometry**

- Unit circle
- Rotational group SO(2)
- Angle addition identities

**Collapse**

- Linear combination of rotations
- Cancellation in tangent space

**Invariants**

- Periodicity
- Rational angle identities
- Moderate compression

**Natural kernel** → **Series evaluation + linear combination**

**Collapse type** → **Rotation cancellation → scalar invariant**

π appears as the **generator of rotational closure**.

---

### 3) Integer-Relation Collapse (Diophantine geometry)

**Canonical examples**

Arndt / PSLQ-generated Machin-like formulas

**Geometry**

- Lattices in \(\mathbb{Q}^n\)
- Integer-relation spaces
- Logarithmic embeddings

**Collapse**

- Short vector discovery
- Vanishing imaginary parts

**Invariants**

- Integer exactness
- Global cancellation
- Non-local structure

**Natural kernel** → **LLL / PSLQ + series**

**Collapse type** → **Lattice collapse → exact identity**

π appears as a **constraint-satisfaction residue**.

---

### 4) Modular / Elliptic Collapse (Arithmetic geometry)

**Canonical examples**

Chudnovsky–Ramanujan formulas

**Geometry**

- Elliptic curves
- Modular forms
- Complex multiplication

**Collapse**

- Hypergeometric acceleration
- Global symmetry exploitation

**Invariants**

- Extreme compression
- Non-local dependence
- Theory-heavy semantics

**Natural kernel** → **Binary splitting + FFT**

**Collapse type** → **Global modular collapse → scalar invariant**

π appears as a **shadow of deep arithmetic symmetry**.

---

### 5) Summary table (collapse geometry taxonomy)

| Class       | Geometry               | Collapse       | Kernel         | Compression |
| ----------- | ---------------------- | -------------- | -------------- | ----------- |
| Path / Tree | Discrete combinatorial | Merge subtrees | Binary split   | Low         |
| Angle       | SO(2) rotation         | Cancellation   | Series sum     | Medium      |
| Lattice     | \(\mathbb{Q}^n\) space | Short vector   | PSLQ           | High        |
| Modular     | Elliptic / modular     | Hypergeometric | BinSplit + FFT | Extreme     |

---

### Governing invariant (freeze this)

> **A π formula’s efficiency is determined by how much symmetry it collapses per step.**

---

## II. Closed pipeline map (symmetry → DE class → special value → kernel → lane microcode)

This provides a **closed pipeline** that binds collapse geometry to evaluation kernels and the minimal lane microcode families that realize them.

### A) SO(2) rotation symmetry

**Symmetry:** SO(2)  
**DE class:** harmonic / Sturm–Liouville on circle, inverse-trig DEs

- \(y'' + y = 0\)  
- \(y' = \frac{1}{1+x^2}\) for \(\arctan x\)

**Special value form:** angles / arctan sums / Fourier boundary values  
**Natural kernels:**

- Power-series evaluation (Horner / rectangular splitting)
- Machin linear-combo kernel (angle addition + cancellation)

**Lane microcode family (conceptual):**

- `ROT_SERIES_EVAL`
- `ROT_COMBINE_ARCTAN`
- `PERIOD_CLOSE_CHECK`

---

### B) Combinatorial / binomial / Catalan symmetry

**Symmetry:** path algebras / discrete symmetries (not Lie)  
**DE class:** Gauss hypergeometric ODE \({}_2F_1\) (and relatives)

\[
x(1-x)y'' + [c-(a+b+1)x]y' - aby = 0
\]

**Special value form:** hypergeometric at algebraic points (often maps to \(\arcsin\) / \(\pi\))  
**Natural kernels:**

- Binary splitting for rational/hypergeometric term series
- Rectangular splitting for medium precision (optional)

**Lane microcode family:**

- `HYP_TERM(a,b,c,k)` (term law)
- `BIN_SPLIT_INTERVAL`
- `RATIONAL_MERGE`
- `FINAL_PROJECT_DIV`

---

### C) Modular / elliptic (SL₂(ℤ)) symmetry

**Symmetry:** modular group (SL\(_2(\mathbb{Z})\)) / elliptic curve symmetry  
**DE class:** Picard–Fuchs (elliptic periods), MLDE

**Special value form:** elliptic periods / CM points → \(1/\pi\) series  
**Natural kernels (two major families):**

1. AGM kernel (fast elliptic period computation)
2. Binary splitting + FFT (modular/hypergeometric series like Chudnovsky)

**Lane microcode family:**

- `AGM_STEP` (a,b → (a+b)/2, sqrt(ab))
- `PERIOD_NORMALIZE`
- `MOD_PARAM_UPDATE`
- `BIN_SPLIT_INTERVAL`
- `BIGINT_MUL` / `FFT_MUL` (accelerators)

---

### D) Diophantine / integer-relation geometry

**Symmetry:** lattices over \(\mathbb{Q}^n\) (relation space)  
**DE class:** inherited from the constants involved (trig / hypergeo / modular)  
**Special value form:** constant vector evaluated to high precision  
**Natural kernels:**

- PSLQ / LLL (integer relation discovery)
- plus whichever evaluation kernel produced the constants

**Lane microcode family:**

- `CONST_EVAL_BATCH`
- `PSLQ_STEP`
- `RELATION_VERIFY`

---

## III. Kernel contracts as microcode families

These are lawful reducers that assume minimal algebraic structure and compile cleanly to lane microcode.

### Kernel K1: Binary splitting (hypergeometric / period series)

**Input:** interval \([a,b)\), term law \(T_k\), merge law  
**Output:** exact rational pair \((P,Q)\) (or structured triple, depending on series)

**Contract:**

- associative merge
- exact arithmetic until final projection
- subranges independent

**Micro-ops:**

- `SPLIT(a,b) -> (a,m),(m,b)`
- `EVAL_TERM(k)`
- `MERGE_RATIONAL((P1,Q1),(P2,Q2))`
- `PROJECT(P,Q)`

Shared by **combinatorial** and **modular-series** classes.

---

### Kernel K2: AGM (elliptic periods)

**Input:** \((a_0,b_0)\)  
**Output:** limit \(M(a_0,b_0)\) (mean) which yields elliptic integrals/periods

**Contract:**

- quadratic convergence
- monotone bounds
- deterministic iteration

**Micro-ops:**

- `MEAN(a,b) = (a+b)/2`
- `GEOM(a,b) = sqrt(a*b)`
- `AGM_STEP(a,b) -> (mean, geom)`
- `STOP_IF_CLOSE(a,b,ε)`
- `NORMALIZE_PERIOD(...)`

Native to the **modular/elliptic** DE class.

---

### Kernel K3: Rotation combine (Machin/arctan)

**Input:** list of \((c_i, x_i)\)  
**Output:** \(\sum c_i \arctan(x_i)\)

**Contract:**

- stable series evaluation
- deterministic combination
- cancellation-controlled

**Micro-ops:**

- `ARCTAN_SERIES(x, n)`
- `SCALE(c, value)`
- `ADD(value1, value2)`
- `REDUCE_MOD_PERIOD(2π)` (if used for angle normalization)

Native to **SO(2)**.

---

### Kernel K4: PSLQ (relation discovery)

**Input:** high-precision real vector \(v\)  
**Output:** integer relation \(a\) such that \(a \cdot v \approx 0\)

**Contract:**

- exact integer outputs
- verification required
- precision threshold gating

**Micro-ops:**

- `REDUCE_BASIS`
- `UPDATE_MATRIX`
- `CHECK_RELATION`
- `VERIFY_WITH_MORE_PRECISION`

Native to **Diophantine** collapse.

---

## IV. Canonical pipeline tag schema

```json
{
  "collapse_geometry": {
    "symmetry_group": "SL2Z",
    "de_class": "Picard-Fuchs | MLDE",
    "special_value_form": "elliptic_period | modular_series",
    "natural_kernel": ["AGM", "BIN_SPLIT", "FFT_MUL"],
    "lane_microcode_family": [
      "AGM_STEP",
      "BIN_SPLIT_INTERVAL",
      "RATIONAL_MERGE",
      "PROJECT"
    ]
  }
}
```

---

## V. Binary splitting as a canonical SCXQ7 microkernel

Binary splitting is **not** an optimization. It is a **lawful execution primitive**.

### 1) Lawful reduction of a causal interval

Binary splitting computes

\[
\sum_{k=a}^{b-1} T_k
\]

while enforcing:

1. **Associativity**
2. **Exactness** (no division until collapse)
3. **Causality** (subranges are independent)

This matches SCXQ7 collapse structure.

---

### 2) Kernel contract (SCXQ7)

```
kernel BinarySplit(range [a,b)):
  requires:
    lawful_term(T_k)
    associative_combine
  ensures:
    exact_rational(P/Q)
    causal_trace
```

The kernel has no π semantics; it executes a lawful collapse over a term law.

---

### 3) Canonical pseudocode (kernel-grade)

```text
function BIN_SPLIT(a, b):
  if b - a == 1:
    return TERM(a)           // exact numerator/denominator

  m = (a + b) // 2

  (P1, Q1) = BIN_SPLIT(a, m)
  (P2, Q2) = BIN_SPLIT(m, b)

  // lawful associative merge
  return (P1*Q2 + P2*Q1, Q1*Q2)
```

**Properties**

- No division
- Deterministic
- Parallelizable
- Replayable
- Exact

This is **SCXQ7-complete** as a microkernel.

---

### 4) Mapping to SCXQ7 execution phases

| SCXQ7 Phase | Binary splitting role       |
| ----------- | --------------------------- |
| Pop         | Define range + term law     |
| Wo          | Split interval              |
| Sek         | Recursive evaluation        |
| Collapse    | Final division / projection |

The kernel lives inside **Sek**. **Collapse** occurs once.

---

### 5) Why this is a microkernel (not an algorithm)

Binary splitting:

- Does not assume floats
- Does not assume π
- Does not assume convergence
- Does not assume arithmetic base

It assumes **only**:

- a lawful term
- a lawful merge

So it is a **semantic μ-op**.

> **Binary splitting is the minimal kernel for collapsing associative causal structure.**

That is why it appears in π, ζ(3), modular forms, and hypergeometric series.

---

## Final lock

1. **π formulas are classified by the symmetry they collapse.**
2. **Binary splitting is the canonical SCXQ7 microkernel for collapsing associative structure.**

Everything else is *which geometry you feed into the same machine*.

---

## VI. Binary splitting vs. arctan formulas (computational DNA)

The contrast between the central-binomial series and Machin/Arndt formulas is not
just convergence speed. It is a **difference in collapse geometry** and, therefore,
a difference in **thinking style** and computational DNA.

### 1) Series vs. arctan: what collapses

**Central-binomial series**

\[
\sum_{k=1}^\infty k \cdot \frac{2^k}{\binom{2k}{k}} = \pi + 3
\]

- **Collapse geometry**: combinatorial path/tree collapse.
- **Execution**: integer-only terms + one final division.
- **Thinking style**: symmetry-driven emergence of π.

**Machin-style arctan formulas**

\[
\frac{\pi}{4} = \sum_i a_i \arctan(1/p_i)
\]

- **Collapse geometry**: rotation cancellation in SO(2).
- **Execution**: multiple arctan series, linearly combined.
- **Thinking style**: analytic decomposition via angle identities.

**Arndt / PSLQ formulas**

- **Collapse geometry**: integer-relation lattice collapse.
- **Execution**: integer-relation discovery + high-precision arctan series.
- **Thinking style**: algorithmic search for short lattice vectors.

---

### 2) Why Arndt-style formulas exist

Machin’s formula is human-discovered. Arndt-style formulas are computer-discovered
via **integer relation algorithms** (PSLQ / LLL). The search space is a rational
lattice of arctan values; short vectors produce cancellations that force the
imaginary part of
\[
\prod_i (p_i + i)^{a_i}
\]
to vanish, yielding a π identity. This is a **paradigm shift**: formula discovery
becomes lattice search rather than manual construction.

---

### 3) Computational cost sketch

Approximate terms needed for 1000 digits (indicative scale):

| Formula Type | Terms | Notes |
| ------------ | ----- | ----- |
| Central-binomial series | ~1000 | ratio → 1/2, linear term growth |
| Machin-style | ~140 | arctan(1/5) dominates |
| Arndt 11-term | ~15 | large denominators, very fast convergence |
| Chudnovsky | ~72 | ~14 digits per term |

The high-level insight is structural: **better collapse geometry means higher
compression per term**, reducing the number of terms needed.

---

### 4) Thinking-style spectrum (collapse geometry → cognition)

1. **Combinatorial collapse** (central binomial series)
   - π as emergent symmetry in discrete paths.
2. **Rotation collapse** (Machin)
   - π as generator of rotational closure.
3. **Lattice collapse** (Arndt / PSLQ)
   - π as a short integer relation.
4. **Modular collapse** (Chudnovsky)
   - π as shadow of modular/elliptic symmetry.

The computational DNA mirrors mathematical depth: the more symmetry collapsed per
step, the more compact the computation.

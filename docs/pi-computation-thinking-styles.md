# π Computation Thinking Styles: Convergence, Architecture, Compression

This note compares π formulas by *thinking style* (mathematical worldview),
convergence behavior, and computational cost. The goal is to characterize how
the **structure** of a formula compresses circular information, not just how
many digits it produces per term.

---

## 1) Binary Splitting vs. Arctan Formulas: Computational DNA

### Central binomial series (π + 3)
```
∑_{k=1}^∞ k · 2^k / C(2k, k) = π + 3
```
**Convergence:** linear, ratio → 1/2 → ~N terms for N digits  \
**Operations per term:** integer multiplies + one division  \
**Thinking style:** *combinatorial collapse* — π emerges from symmetric path
counts and generating functions.

### Machin (1706)
```
π/4 = 4 arctan(1/5) − arctan(1/239)
```
**Convergence:** arctan(1/5) ~ 1/25^k → ~N/3 terms for N digits  \
**Operations per term:** polynomial arithmetic at N-digit precision  \
**Thinking style:** *analytic decomposition* — angle addition and identity
engineering.

### Arndt (11-term arctan)
**Structure:** ∑ aᵢ arctan(1/pᵢ), with primes pᵢ and integer coefficients aᵢ  \
**Convergence:** very rapid due to large denominators  \
**Thinking style:** *number-theoretic optimization* — integer-relation search
(PSLQ/LLL) over arctan spaces.

---

## 2) Why Arndt-Style Formulas Exist

Machin-type formulas are found by human identity work. Arndt’s formula is
algorithmically discovered:

1. Search for integer relations among arctan values at rational points.
2. Enforce cancellation by targeting the imaginary part of
   ```
   ∏ (pᵢ + i)^{aᵢ}
   ```
3. Minimize coefficients to improve convergence and reduce cancellation cost.

This shifts discovery from *single identities* to *families of identities*
optimized by computation.

---

## 3) Cost Model (1000 digits, rough terms)

| Formula Type | Terms | Ops per term | Total ops (naïve) |
| --- | --- | --- | --- |
| Central binomial | ~1000 | ~log(k) mul | ~O(N²) |
| Machin | ~140 | ~O(N) mul | ~O(N²) |
| Arndt 11-term | ~15 | ~O(N) mul | ~O(N) |
| Chudnovsky | ~log(N) | ~O(N log N) | ~O(N log³ N) |

### Chudnovsky (1988)
```
1/π = (12 / 640320^{3/2})
  · ∑_{k=0}^∞ [(6k)! (545140134k + 13591409)]
    / [(3k)! (k!)^3 (−262537412640768000)^k]
```
**Convergence:** ~14 digits per term  \
**Thinking style:** *modular forms + hypergeometric acceleration*.

---

## 4) Style-of-Thinking Spectrum

**Classical analysis**  \
* Generating functions, symmetry, combinatorial collapse.

**Pre-computer optimization**  \
* Trig identities + small denominators + human search.

**Algorithmic number theory**  \
* LLL/PSLQ, integer relation space exploration.

**Deep modern theory**  \
* Modular equations, Ramanujan-type series, binary splitting + FFT.

---

## 5) Compression View (GCCP Framing)

Each formula is a *compression map* for circle information:

* **Central binomial series:** collapse of symmetric combinatorial paths.
* **Machin/Arndt:** collapse of rotational symmetries and arctan relations.
* **Chudnovsky:** collapse of modular symmetries via hypergeometric structure.

**Compression ratio → terms needed**
* O(N) terms: low compression (linear series).
* O(N/log k) terms: medium compression (Machin-like).
* O(log N) terms: high compression (Chudnovsky).

---

## 6) Takeaway

The progression
```
central binomial → Machin → Arndt → Chudnovsky
```
tracks a shift in *mathematical consciousness*: from analytic identities, to
algorithmic discovery, to deep modular structure. Computation speed is a proxy
for structural depth — better formulas encode more of π’s geometry per term.

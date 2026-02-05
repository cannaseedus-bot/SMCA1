# object://retrieve/semantic.v1 (π-GCCP Extension)

This is how π-signals become **first-class retrieval primitives**.

---

## object://retrieve/semantic.v1 (EXTENSION)

```json
{
  "@object": "retrieve.semantic.v1",
  "@extension": "π-gccp",

  "lanes": [
    "scxq2.pi-signal.v1"
  ],

  "operators": {
    "π-collapse": {
      "input": "scxq2.pi-signal.v1",
      "output": "scalar",
      "law": "exact"
    },
    "π-interference": {
      "inputs": ["scxq2.pi-signal.v1", "scxq2.pi-signal.v1"],
      "output": "scxq2.pi-signal.v1",
      "law": "phase-addition + orientation-xor"
    }
  },

  "ordering": {
    "primary": "geometric_mass",
    "secondary": "curvature",
    "tertiary": "orientation"
  },

  "profile_dependency": false
}
```

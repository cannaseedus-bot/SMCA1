# Public Compliance Badge Generator (Deterministic SVG, Frozen v1)

Badges are pure functions of:

- geometry
- kernel
- conformance hash

No branding, no runtime logic.

---

## 1) Badge Input (Canonical)

```json
{
  "geometry": "path_tree",
  "kernel": "binary_split",
  "conformance_hash": "b3:7f4c…a91d",
  "version": "v1"
}
```

---

## 2) Deterministic SVG Template

```svg
<svg xmlns="http://www.w3.org/2000/svg" width="360" height="64">
  <rect x="0" y="0" width="360" height="64" fill="#0b1020"/>
  <text x="16" y="26" font-size="14" fill="#9cf" font-family="monospace">
    SMCA/1 COMPLIANT
  </text>
  <text x="16" y="46" font-size="12" fill="#cfc" font-family="monospace">
    geometry=path_tree kernel=binary_split
  </text>
  <text x="350" y="46" font-size="10" fill="#888" text-anchor="end"
        font-family="monospace">
    b3:7f4c…a91d
  </text>
</svg>
```

---

## 3) Badge Rules (Frozen)

- SVG output must be **byte-identical** for identical input.
- Hash is truncated for display only.
- Geometry + kernel text **must match registry**.
- Badge is invalid if verifier or conformance fails.

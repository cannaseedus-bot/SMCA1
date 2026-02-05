# SCXQ2 Lane: π-Signal v1 (NORMATIVE)

These layouts are **execution lanes**, not storage formats.
They map **directly** into π-collapse and interference operators.

---

## SCXQ2.LANE.PI_SIGNAL.v1

```json
{
  "@lane": "scxq2.pi-signal.v1",
  "@type": "binary-execution-lane",
  "@endianness": "little",
  "@alignment": 8,

  "header": {
    "magic": "0x50534947",
    "version": 1,
    "lane_type": "π-signal",
    "flags": {
      "exact_math": true,
      "orientation_sensitive": true,
      "profile_independent": true
    }
  },

  "fields": [
    {
      "name": "phase",
      "type": "f64",
      "meaning": "angular phase in radians"
    },
    {
      "name": "orientation",
      "type": "i8",
      "meaning": "+1 or -1 (mobius-capable)"
    },
    {
      "name": "curvature",
      "type": "f64",
      "meaning": "geometric curvature (hyperbolic if < 0)"
    },
    {
      "name": "mass_geometric",
      "type": "f64",
      "meaning": "π-derived mass"
    },
    {
      "name": "mass_algebraic",
      "type": "f64",
      "meaning": "finite residue (e.g. +3)"
    },
    {
      "name": "closure_flag",
      "type": "u8",
      "meaning": "0=open, 1=closed"
    }
  ],

  "footer": {
    "hash": "blake3-256",
    "proof_offset": "u32"
  }
}
```

### Collapse Rule (hard-wired)

```text
collapse(π-signal):
  if closure_flag == 1:
    return mass_geometric + mass_algebraic
  else:
    return undefined (illegal)
```

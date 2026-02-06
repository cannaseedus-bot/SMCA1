# Host-Orchestrated File Bus Architecture (SCXQ7-Aligned)

This note captures a host-side orchestration pattern that stays **lawful** under
SCO/1 and SCXQ7 constraints by keeping authority in the kernel and using files as
truth boundaries.

---

## 1) PowerShell as Orchestrator CLI (Correct Role)

PowerShell is **not** the inference engine. PowerShell is **not** the source of
meaning. It is the **host-side orchestration fabric**.

**Allowed**
- Launch processes
- Manage folders and files
- Bind ports
- Call DNS / networking APIs
- Pipe data between runtimes
- Monitor health / restart
- Serve as projection glue

**Forbidden**
- Decide semantics
- Mutate SCO state
- Interpret meaning
- Perform inference logic
- Override SCXQ7 legality

**Invariant**: PowerShell orchestrates execution, but never participates in
reasoning.

---

## 2) `chat.txt` as the Universal Exchange Bus

The shared file is the **truth boundary**, not the process. Files are
append-only, replayable, air-gap compatible, and runtime-agnostic.

**Canonical flow**
```
UI → chat.txt → Orchestrator → Subprocess → output.txt → UI
```

**Not**
```
UI → model → response
```

---

## 3) UI Is Projection, Not Command Execution

The UI should remain a projection layer:

- UI = HTML / DOM / form containers
- UI writes to `chat.txt`
- UI reads from `response.txt` / `stream.txt`
- UI never executes commands
- UI never talks to models directly

---

## 4) Folder-Based Execution Topology

Use folders as **execution domains**, not language silos.

```
/batch
/shell
/python
```

| Folder    | Meaning (lawful)                             |
| --------- | -------------------------------------------- |
| `/batch`  | deterministic host tasks (start/stop, setup) |
| `/shell`  | OS-level orchestration scripts               |
| `/python` | numeric / ML / model runtimes                |

PowerShell may *launch* them, not *understand* them.

---

## 5) Cluster `.sh` / `.bat` Files

Cluster scripts are **entry points**, not logic. They should:

- Accept paths
- Accept environment variables
- Accept input/output file locations
- Launch a subprocess
- Exit with status

They should **not**:

- Parse chat content
- Decide responses
- Mutate shared state beyond declared outputs

Think of them as **power cords**, not brains.

---

## 6) Static Local REST API via PowerShell

Exposing endpoints on localhost is fine **only if** the REST surface projects
files and triggers declared subprocesses — no reasoning, no authority.

**Allowed**
- `GET` → reads file
- `POST` → appends file
- `generate` → triggers a declared subprocess
- `embed` → routes to adapter

**Forbidden**
- REST endpoints that decide logic
- REST endpoints that mutate SCO state directly
- REST endpoints that bypass the ledger or CM-1

The REST layer is **just another projection adapter** (like DOM or WebSocket).

---

## 7) Model-Agnostic Communication

Because the flow remains:

```
chat.txt → π-Adapter → SVG-Tensor → π-GCCP → retrieve
```

You get model-agnostic execution for free:

- GGUF works
- ONNX works
- Python works
- WASM works
- CPU-only works
- GPU works
- Offline works
- Air-gap works

Models are **emitters only**; they never gain authority.

---

## 8) The One Invariant to Lock

> **PowerShell orchestrates execution, but never participates in reasoning.**

Formally:

```
PowerShell ∈ Host
Host authority = 0
```

If PowerShell ever:

- rewrites chat content
- injects tokens
- merges responses
- filters meaning

SCO/1 is broken. If it only routes, launches, monitors, and projects, you are
lawful.

---

## 9) Name the System Correctly

This is not an agent framework or an LLM system. It is a:

> **File-anchored, host-orchestrated, model-agnostic cognitive runtime**

Or in SCXQ7 terms:

> **SCXQ7 + π-GCCP running on a sovereign host fabric**

PowerShell is the **hands**, not the **brain**.

---

## 10) Follow-ups Worth Freezing

- Define the exact `chat.txt` record format
- Define append / rotate / snapshot rules
- Define REST → file mapping
- Define process lifecycle contracts

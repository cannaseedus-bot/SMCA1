# PowerShell Reference Orchestrator — Implementation Outline (Draft)

> Purpose: A minimal, deterministic, PowerShell-based orchestrator that implements the **Process Lifecycle Contracts** and **append-only file substrate** described in the frozen CHAT I/O SUBSTRATE v1 specification. This document is an implementation outline, not an authority expansion.

---

## 1) Scope & Non-Goals

**Scope**
- Launch and monitor declared processes using the **Launch Contract**: `--in`, `--out`, `--work`, `--env`.  
- Enforce **append-only** outputs and **no chat.txt mutation** by the orchestrator itself.  
- Emit **settlement records** to `chat.txt` and optionally write `stream.txt` when `stream=true`.  
- Perform **snapshot** and **rotation** actions around process lifecycle boundaries.

**Non-Goals**
- No content interpretation.  
- No new REST behavior.  
- No network access unless explicitly declared by the process.

---

## 2) Files & Directories (Concrete Targets)

```
chat.txt                  (append-only records)
stream.txt                (append-only @chunk v1)
snapshots/                (read-only checkpoints)
logs/                     (orchestrator logs; optional)
work/                     (ephemeral per-process)
```

---

## 3) Orchestrator Inputs

**Configuration File** (recommended): `orchestrator.config.json`
```json
{
  "chatPath": "chat.txt",
  "streamPath": "stream.txt",
  "snapshotsDir": "snapshots",
  "workDir": "work",
  "rotation": {
    "maxBytes": 8388608,
    "maxRecords": 10000,
    "maxAgeMs": 86400000
  },
  "lockFile": "chat.lock"
}
```

**Trigger Model**: Orchestrator is invoked externally after `POST /generate` or `POST /embed` appends a record to `chat.txt`.

---

## 4) Minimal Orchestrator Flow (Pseudo-Code)

1. **Lock** `chat.txt` (exclusive).  
2. **Read** last accepted record (`@record v1`), ensure valid header order.  
3. **Rotation Check** (`maxBytes`, `maxRecords`, `maxAgeMs`):
   - If triggered, perform atomic rotation and write a rotation marker as the first new record.
4. **Snapshot** (pre-execution boundary):
   - Write `snapshots/chat.snapshot.<hash>.txt` with header and content.
5. **Launch Process** with `Start-Process`:
   - `--in <path>` `--out <path>` `--work <dir>` `--env <file?>`
   - No STDIN/STDOUT dependence.
6. **Monitor**:
   - Wait for exit.
   - Capture exit code.
7. **Close Streams** if `stream=true`:
   - Append final `@eos true` chunk to `stream.txt`.
8. **Settlement Record**:
   - Append to `chat.txt` with `state=SUCCESS|FAILED|ILLEGAL` (see §6).
9. **Snapshot** (post-execution boundary).
10. **Unlock** `chat.txt`.

---

## 5) PowerShell Implementation Notes

### 5.1 File Locking
- Use `System.IO.FileStream` with `FileShare.None` for a lock file or `chat.txt`.
- Keep lock duration minimal: release during long process runtime if safe to do so without violating the append-only constraint.

### 5.2 Append-Only Writes
- Use `Add-Content -NoNewline` with explicit `\n` to ensure LF only.
- Always `Flush()` and `Close()` stream after writing a full record.

### 5.3 Snapshot Generation
```powershell
$snapshotPath = Join-Path $snapshotsDir ("chat.snapshot.$hash.txt")
Set-Content -Path $snapshotPath -Value $snapshotContent -NoNewline -Encoding utf8
```

---

## 6) Settlement Record Template

```
@record v1
id: <uuid-v7>
ts: <unix-ms>
role: system
intent: control
channel: internal
reply_to: <uuid-v7|null>
stream: false
attachments: <none|pathlist>
hash_prev: <hex|null>
--
state: SUCCESS|FAILED|ILLEGAL
exit_code: <0|1|2>
process: <name>
```

---

## 7) Stream Output (If `stream=true`)

```
@chunk v1
id: <uuid>
seq: 1
--
<text>

@chunk v1
id: <uuid>
seq: 2
--
<text>

@eos true
```

---

## 8) Implementation Skeleton (PowerShell)

```powershell
param(
  [string]$ConfigPath = "orchestrator.config.json"
)

# 1) Load config
$config = Get-Content $ConfigPath -Raw | ConvertFrom-Json

# 2) Acquire lock
$lockStream = [System.IO.File]::Open($config.lockFile, 'OpenOrCreate', 'ReadWrite', 'None')

# 3) Read tail record, validate header, check rotation

# 4) Snapshot (pre)

# 5) Launch process
$process = Start-Process -FilePath $exe -ArgumentList @(
  "--in", $inPath, "--out", $outPath, "--work", $workDir, "--env", $envPath
) -PassThru -NoNewWindow

$process.WaitForExit()
$exitCode = $process.ExitCode

# 6) Stream close if needed

# 7) Settlement record append

# 8) Snapshot (post)

# 9) Release lock
$lockStream.Close()
```

---

## 9) Validation Checklist

- [ ] Writes to `chat.txt` are append-only and full-record atomic.  
- [ ] `hash_prev` chain integrity maintained.  
- [ ] Rotation executed when any threshold trips.  
- [ ] Snapshots written on pre/post boundaries and on error.  
- [ ] Exit codes mapped to settlement `state`.  
- [ ] Stream closure includes `@eos true`.  

---

## 10) Next Step (If Approved)

- Convert this outline into a concrete `orchestrator.ps1` reference implementation.
- Provide a minimal CM-1 test vector pack to validate rotation, snapshots, and settlement behaviors.

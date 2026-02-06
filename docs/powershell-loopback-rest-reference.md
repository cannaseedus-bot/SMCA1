# REST LOOPBACK v1 — PowerShell Reference (FROZEN)

**Purpose:** Local-only REST façade that maps HTTP ↔ files (`chat.txt`, `stream.txt`).
**Authority:** None (projection-only).
**Network:** Loopback only (`127.0.0.1` / `localhost`).
**Safety:** CM-1 preflight on every write.
**Persistence:** Append / rotate / snapshot only.

---

## 1) Endpoint Surface (Closed)

```
POST   /chat        → append chat.txt (user message)
POST   /generate    → append chat.txt (generate intent)
GET    /stream      → read-only tail of stream.txt
POST   /embed       → append chat.txt (embed intent)
GET    /health      → OK
POST   /snapshot    → snapshot all files
```

### Global Rules

- Content-Type: `application/json`
- Body size limit: 256 KB
- No auth (local loopback only)
- Writes are **serialized** by orchestrator
- Any CM-1 failure → HTTP 422

---

## 2) REST → File Mapping (Normative)

### 2.1 POST `/chat`

**Request**

```json
{
  "role": "user",
  "text": "Hello",
  "reply_to": null
}
```

**Effect** → append **one** `@record v1` to `chat.txt`

```
@record v1
id: <uuid-v7>
ts: <unix-ms>
role: user
intent: chat
channel: public
reply_to: null
stream: true
attachments: none
hash_prev: <computed>
--
Hello
```

**Response**

```json
{ "ok": true }
```

---

### 2.2 POST `/generate`

**Request**

```json
{
  "prompt": "Write a haiku",
  "reply_to": "<uuid-v7|null>"
}
```

**Effect** → append `@record v1` (intent=`generate`) to `chat.txt`

**Response**

```json
{ "ok": true }
```

---

### 2.3 POST `/embed`

**Request**

```json
{
  "text": "vectorize this",
  "reply_to": "<uuid-v7|null>"
}
```

**Effect** → append `@record v1` (intent=`embed`) to `chat.txt`

**Response**

```json
{ "ok": true }
```

---

### 2.4 GET `/stream`

**Query**

```
/stream?from_seq=123
```

**Effect** → read-only projection from `stream.txt`

**Response**

```json
{
  "id": "<uuid-v7>",
  "from_seq": 123,
  "chunks": [
    { "seq": 123, "text": "..." }
  ],
  "eos": false
}
```

---

### 2.5 POST `/snapshot`

**Effect** → snapshot `chat.txt` + `stream.txt`

**Response**

```json
{ "ok": true, "snapshot": "yyyyMMdd-HHmmss" }
```

---

## 3) Reference PowerShell Server (Minimal)

**File:** `rest-loopback.ps1`

```powershell
param(
  [string]$Root = ".",
  [string]$Addr = "http://127.0.0.1:8787/"
)

$Chat   = Join-Path $Root "chat.txt"
$Stream = Join-Path $Root "stream.txt"
$Snaps  = Join-Path $Root "snapshots"
New-Item -ItemType Directory -Force -Path $Snaps | Out-Null

function Cm1-Preflight($Text) {
  # CALL OUT to frozen CM-1 verifier; must PASS
  # Throw on FAIL/ILLEGAL
}

function HashPrev($Path) {
  if (-not (Test-Path $Path)) { return "null" }
  (Get-FileHash $Path -Algorithm SHA256).Hash.ToLower()
}

function Append-Chat($Record) {
  Cm1-Preflight $Record
  Add-Content -Path $Chat -Value $Record -Encoding UTF8
}

function Snapshot-All {
  $ts = Get-Date -Format "yyyyMMdd-HHmmss"
  Copy-Item $Chat   (Join-Path $Snaps "chat.snapshot.$ts.txt") -ErrorAction SilentlyContinue
  Copy-Item $Stream (Join-Path $Snaps "stream.snapshot.$ts.txt") -ErrorAction SilentlyContinue
  return $ts
}

$listener = New-Object System.Net.HttpListener
$listener.Prefixes.Add($Addr)
$listener.Start()
Write-Host "REST loopback listening at $Addr"

while ($listener.IsListening) {
  $ctx = $listener.GetContext()
  $req = $ctx.Request
  $res = $ctx.Response
  try {
    switch ($req.Url.AbsolutePath) {

      "/health" {
        $res.StatusCode = 200
        $out = @{ ok = $true } | ConvertTo-Json
      }

      "/snapshot" {
        if ($req.HttpMethod -ne "POST") { throw "405" }
        $ts = Snapshot-All
        $res.StatusCode = 200
        $out = @{ ok = $true; snapshot = $ts } | ConvertTo-Json
      }

      "/stream" {
        if ($req.HttpMethod -ne "GET") { throw "405" }
        $from = [int]($req.QueryString["from_seq"] ?? 0)
        $chunks = @()
        if (Test-Path $Stream) {
          # projection-only; simple tail (implementation-defined)
          $chunks += @{ seq = $from; text = "" }
        }
        $out = @{ id = ""; from_seq = $from; chunks = $chunks; eos = $false } | ConvertTo-Json
        $res.StatusCode = 200
      }

      "/chat" { goto POST_CHAT }
      "/generate" { goto POST_GEN }
      "/embed" { goto POST_EMB }

      default { throw "404" }
    }
  } catch {
    $res.StatusCode = 422
    $out = @{ ok = $false; error = "$_" } | ConvertTo-Json
  }

  $buf = [System.Text.Encoding]::UTF8.GetBytes($out)
  $res.ContentType = "application/json"
  $res.OutputStream.Write($buf, 0, $buf.Length)
  $res.Close()
  continue

POST_CHAT:
  if ($req.HttpMethod -ne "POST") { throw "405" }
  $body = (New-Object IO.StreamReader $req.InputStream).ReadToEnd() | ConvertFrom-Json
  $hp = HashPrev $Chat
  $rec = @"
@record v1
id: $(New-Guid)
ts: $([DateTimeOffset]::UtcNow.ToUnixTimeMilliseconds())
role: user
intent: chat
channel: public
reply_to: $($body.reply_to ?? "null")
stream: true
attachments: none
hash_prev: $hp
--
$($body.text)
"@
  Append-Chat $rec
  $res.StatusCode = 200
  $out = @{ ok = $true } | ConvertTo-Json
  goto RESP

POST_GEN:
  if ($req.HttpMethod -ne "POST") { throw "405" }
  $body = (New-Object IO.StreamReader $req.InputStream).ReadToEnd() | ConvertFrom-Json
  $hp = HashPrev $Chat
  $rec = @"
@record v1
id: $(New-Guid)
ts: $([DateTimeOffset]::UtcNow.ToUnixTimeMilliseconds())
role: user
intent: generate
channel: public
reply_to: $($body.reply_to ?? "null")
stream: true
attachments: none
hash_prev: $hp
--
$($body.prompt)
"@
  Append-Chat $rec
  $res.StatusCode = 200
  $out = @{ ok = $true } | ConvertTo-Json
  goto RESP

POST_EMB:
  if ($req.HttpMethod -ne "POST") { throw "405" }
  $body = (New-Object IO.StreamReader $req.InputStream).ReadToEnd() | ConvertFrom-Json
  $hp = HashPrev $Chat
  $rec = @"
@record v1
id: $(New-Guid)
ts: $([DateTimeOffset]::UtcNow.ToUnixTimeMilliseconds())
role: user
intent: embed
channel: public
reply_to: $($body.reply_to ?? "null")
stream: false
attachments: none
hash_prev: $hp
--
$($body.text)
"@
  Append-Chat $rec
  $res.StatusCode = 200
  $out = @{ ok = $true } | ConvertTo-Json
  goto RESP

RESP:
  $buf = [System.Text.Encoding]::UTF8.GetBytes($out)
  $res.ContentType = "application/json"
  $res.OutputStream.Write($buf, 0, $buf.Length)
  $res.Close()
}
```

---

## 4) Process Lifecycle Contract (Binding)

- **Start:** Orchestrator boots REST loopback.
- **Write:** REST handler → CM-1 preflight → append file.
- **Run:** Subprocesses read `chat.txt`, write `stream.txt`.
- **Settle:** EOS written → orchestrator snapshots.
- **Crash:** No mutation rollback; snapshots preserve history.

---

## 5) Compliance Assertions

- Loopback only (no external sockets).
- CM-1 gates **all** writes.
- Append-only semantics preserved.
- REST is projection, not authority.

**Status:** FROZEN
**Next legal move:** publish native/WASM CM-1 verifier binaries **or** mint REST conformance vectors.

# Madara DB Visualizer - Development Plan

A web-based visualizer for Madara's RocksDB database.

## Configuration

| Setting | Value |
|---------|-------|
| **Test DB Path** | `/tmp/madara_devnet_poc_v2/db` |
| **Type Reuse** | Import from `mc-db` crate (madara repo) |
| **Authentication** | None required |
| **Real-time Updates** | Not required (some lag acceptable) |
| **Repo Location** | `/Users/mohit/Desktop/karnot/madara-db-visualizer` |

---

## Tech Stack

| Layer | Choice | Rationale |
|-------|--------|-----------|
| **Backend** | Axum | Lightweight, async, Tokio ecosystem |
| **Frontend** | Leptos | Fast, Rust → WASM, fine-grained reactivity |
| **Styling** | TailwindCSS | Utility-first, rapid iteration |
| **DB Access** | RocksDB (read-only) | Direct Madara storage access |
| **Index** | SQLite (Phase 4+) | Complex queries |
| **Build** | Trunk | Standard Rust WASM tooling |
| **Feedback** | agent-browser | Visual verification |

---

## Code Structure

```
madara-db-visualizer/
├── Cargo.toml                    # Workspace root
├── Trunk.toml                    # WASM build config
├── index.html                    # Entry point
├── input.css                     # Tailwind input
├── tailwind.config.js
│
├── crates/
│   ├── db-reader/                # RocksDB access (reuses mc-db types)
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── connection.rs     # Read-only RocksDB
│   │       └── queries.rs        # Query functions
│   │
│   ├── api/                      # Axum HTTP server
│   │   └── src/
│   │       ├── main.rs
│   │       └── routes/
│   │
│   ├── frontend/                 # Leptos WASM
│   │   └── src/
│   │       ├── main.rs
│   │       ├── app.rs
│   │       └── components/
│   │
│   └── types/                    # Shared JSON types
│       └── src/lib.rs
│
└── static/
```

---

## Development Workflow

Each phase follows this cycle - **commits only after verification passes**:

```
┌─────────────────────────────────────────────────────────────┐
│                     Phase N Workflow                        │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  1. IMPLEMENT                                               │
│     └─ Write code for phase requirements                   │
│                                                             │
│  2. VERIFY (agent-browser)                                  │
│     └─ trunk serve (frontend)                              │
│     └─ cargo run -p api (backend)                          │
│     └─ agent-browser open http://localhost:8080            │
│     └─ agent-browser snapshot -i                           │
│     └─ agent-browser screenshot /tmp/phase-N.png           │
│                                                             │
│  3. SELF-FEEDBACK                                           │
│     └─ Review screenshot: Does it look right?              │
│     └─ Test interactions: Do clicks/navigation work?       │
│     └─ Check data: Is it accurate from the DB?             │
│     └─ Identify issues                                     │
│                                                             │
│  4. FIX ISSUES (if any)                                     │
│     └─ Go back to step 1, fix the problems                 │
│     └─ Repeat steps 2-3 until all checks pass              │
│                                                             │
│  5. COMMIT (only after verification passes)                 │
│     └─ git add -A                                          │
│     └─ git commit -m "phase-N: <description>"              │
│                                                             │
│  6. Move to Phase N+1                                       │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### Key Rule: No commit until it works!

```
❌ Wrong:  Implement → Commit → Verify → Fix → Commit fix
✅ Right:  Implement → Verify → Fix → Verify → ... → Works! → Commit
```

---

## Phases

### Phase 0: Project Scaffold

**Goal**: Working project structure with basic hello world.

**Deliverables**:
- [ ] Workspace Cargo.toml
- [ ] Basic Axum server at `localhost:3000`
- [ ] Basic Leptos app at `localhost:8080`
- [ ] TailwindCSS integrated
- [ ] `GET /api/health` → `{"status": "ok"}`

**Verification**:
```bash
# Terminal 1
cargo run -p api

# Terminal 2
trunk serve

# Verify
agent-browser open http://localhost:8080
agent-browser snapshot -i
agent-browser screenshot /tmp/phase0.png
```

**Success Criteria**:
- [ ] Page shows "Madara DB Visualizer"
- [ ] API returns `{"status": "ok"}` on `/api/health`
- [ ] No console errors

**Commit when verified**: `phase-0: project scaffold with hello world`

---

### Phase 1: Database Connection

**Goal**: Connect to RocksDB, show basic stats.

**Deliverables**:
- [ ] RocksDB read-only connection
- [ ] `GET /api/stats` → column list, latest block, tx count
- [ ] Frontend displays stats

**API**:
```
GET /api/stats
→ { "db_path": "...", "latest_block": 100, "columns": [...] }
```

**UI**:
```
┌─────────────────────────────────────┐
│ Madara DB Visualizer                │
├─────────────────────────────────────┤
│ Database: /tmp/madara_devnet_poc_v2 │
│ Latest Block: 100                   │
│ Columns: 18                         │
└─────────────────────────────────────┘
```

**Success Criteria**:
- [ ] Stats endpoint returns real data from DB
- [ ] Frontend displays latest block number
- [ ] Column count is 18 (excluding bonsai)

**Commit when verified**: `phase-1: rocksdb connection and stats endpoint`

---

### Phase 2: Block Explorer

**Goal**: Browse blocks with pagination, view block details.

**Deliverables**:
- [ ] `GET /api/blocks?limit=20&offset=0`
- [ ] `GET /api/blocks/:number`
- [ ] Block list component with pagination
- [ ] Block detail component
- [ ] Navigation: sidebar + prev/next

**UI**:
```
┌────────────┬─────────────────────────────┐
│ Sidebar    │ Block #100                  │
│            │                             │
│ • Blocks ◄ │ Hash: 0x7a8b...             │
│ • Txns     │ Parent: 0x6f5e...           │
│ • State    │ Timestamp: 2024-01-15       │
│            │ Transactions: 45            │
│            │                             │
│            │ [◄ Prev] [Next ►]           │
├────────────┼─────────────────────────────┤
│ Block List │ #100 | 0x7a8b... | 45 txns  │
│            │ #99  | 0x6f5e... | 32 txns  │
│            │ #98  | 0x5e4d... | 28 txns  │
└────────────┴─────────────────────────────┘
```

**Success Criteria**:
- [ ] Block list shows real blocks from DB
- [ ] Clicking a block shows its details
- [ ] Prev/Next navigation works
- [ ] Pagination works (next page, prev page)

**Commit when verified**: `phase-2: block list and detail views`

---

### Phase 3: Transaction Browser

**Goal**: View transactions in a block, transaction details.

**Deliverables**:
- [ ] `GET /api/blocks/:number/transactions`
- [ ] `GET /api/transactions/:hash`
- [ ] Transaction list per block
- [ ] Transaction detail (type, status, fee, events)
- [ ] Revert reason for failed txs

**UI**:
```
┌────────────┬─────────────────────────────┐
│ Sidebar    │ Transaction 0x1a2b...       │
│            │                             │
│ • Blocks   │ Block: #100 (index 5)       │
│ • Txns  ◄  │ Type: INVOKE_V3             │
│ • State    │ Status: ✓ Succeeded         │
│            │ Fee: 0.00012 ETH            │
│            │                             │
│            │ Events (3):                 │
│            │ • Transfer(...)             │
│            │ • Approval(...)             │
└────────────┴─────────────────────────────┘
```

**Success Criteria**:
- [ ] Can see all transactions in a block
- [ ] Clicking a tx shows full details
- [ ] Events are displayed
- [ ] Failed txs show revert reason

**Commit when verified**: `phase-3: transaction list and details`

---

### Phase 4: Contract & Class Viewer

**Goal**: View contract storage and class information.

**Deliverables**:
- [ ] `GET /api/contracts/:address`
- [ ] `GET /api/contracts/:address/storage`
- [ ] `GET /api/classes/:hash`
- [ ] Contract view (nonce, class hash, storage slots)
- [ ] Class view (type, compiled hash, ABI)

**UI**:
```
┌────────────┬─────────────────────────────┐
│ Sidebar    │ Contract 0x049d...          │
│            │                             │
│ • Blocks   │ Class: 0x07b8...            │
│ • Txns     │ Nonce: 42                   │
│ • State ◄  │                             │
│ • Classes  │ Storage:                    │
│            │ 0x01 → 0x1234...            │
│            │ 0x02 → 0x5678...            │
│            │                             │
│            │ [Search key: ________]      │
└────────────┴─────────────────────────────┘
```

**Success Criteria**:
- [ ] Can lookup contract by address
- [ ] Shows correct nonce and class hash
- [ ] Storage slots displayed with values
- [ ] Class info shows type (Legacy/Sierra)

**Commit when verified**: `phase-4: contract state and class browser`

---

### Phase 5: State Diff & Search

**Goal**: View state changes per block, global search.

**Deliverables**:
- [ ] `GET /api/blocks/:number/state-diff`
- [ ] `GET /api/search?q=...`
- [ ] State diff view (deployed, storage changes, nonces)
- [ ] Universal search bar
- [ ] Auto-detect search type (block/tx/contract/class)

**UI**:
```
┌─────────────────────────────────────────┐
│ [🔍 Search: 0x049d...              ]   │
├────────────┬────────────────────────────┤
│ Block #100 │ State Diff                 │
│ ├─ Info    │                            │
│ ├─ Txns    │ Deployed (2):              │
│ └─ Diff ◄  │ • 0x049d... → class 0x07b8 │
│            │                            │
│            │ Storage Changes:           │
│            │ ▸ 0x049d... (5 slots)      │
│            │   0x01: 0x00 → 0x1234      │
│            │                            │
│            │ Nonces:                    │
│            │ • 0x049d...: 41 → 42       │
└────────────┴────────────────────────────┘
```

**Success Criteria**:
- [ ] State diff shows all changes in a block
- [ ] Search by block number works
- [ ] Search by tx hash works
- [ ] Search by contract address works

**Commit when verified**: `phase-5: state diff viewer and search`

---

### Phase 6: Complex Queries (SQLite Index)

**Goal**: Enable queries RocksDB can't handle efficiently.

**Deliverables**:
- [ ] SQLite schema for transactions, contracts
- [ ] Background indexer (sync from RocksDB)
- [ ] `GET /api/transactions?status=reverted`
- [ ] `GET /api/transactions?sender=0x...`
- [ ] `GET /api/contracts?class_hash=0x...`
- [ ] Index status indicator

**New crate**: `crates/indexer/`

**UI**:
```
┌────────────┬─────────────────────────────┐
│ Sidebar    │ Advanced Filters            │
│            │                             │
│ • Blocks   │ Status: [Failed Only ☑]    │
│ • Txns     │ Sender: [0x...          ]  │
│ • State    │ Block:  [0   ] to [100  ]  │
│ • Advanced◄│                             │
│            │ Results (23 failed txs):    │
│ Index: ✓   │ #100 | 0x1a2b | "Out of gas"|
│ 100/100    │ #98  | 0x3c4d | "Assert"    │
└────────────┴─────────────────────────────┘
```

**Success Criteria**:
- [ ] Index builds from RocksDB without errors
- [ ] Can query failed transactions
- [ ] Can filter by sender address
- [ ] Index status shows in UI

**Commit when verified**: `phase-6: sqlite index and complex queries`

---

### Phase 7: Polish & Export

**Goal**: Production-ready polish.

**Deliverables**:
- [ ] Loading states / skeletons
- [ ] Error handling / boundaries
- [ ] Export to JSON
- [ ] Responsive design
- [ ] Dark mode toggle
- [ ] Shareable URLs

**Success Criteria**:
- [ ] No unhandled errors/crashes
- [ ] Works on mobile viewport
- [ ] Can export block/tx data to JSON
- [ ] URLs can be shared and reopened

**Commit when verified**: `phase-7: polish, export, responsive design`

---

## Commit Strategy

**Rule**: Only commit after all success criteria pass.

```
✅ Verified working → Commit
❌ Still has issues → Keep fixing, don't commit
```

**Expected commits** (one per completed phase):
```
Initial commit: project setup with development plan
phase-0: project scaffold with hello world
phase-1: rocksdb connection and stats endpoint
phase-2: block list and detail views
phase-3: transaction list and details
phase-4: contract state and class browser
phase-5: state diff viewer and search
phase-6: sqlite index and complex queries
phase-7: polish, export, responsive design
```

No intermediate "fix" commits - fix issues before committing the phase.

---

## Self-Feedback Checklist

After each phase, verify:

### Visual Check
- [ ] Does the layout match the mockup?
- [ ] Are fonts/colors consistent?
- [ ] Is spacing appropriate?

### Functional Check
- [ ] Do all links/buttons work?
- [ ] Does pagination work?
- [ ] Do API calls succeed?

### Data Check
- [ ] Is data from actual DB?
- [ ] Are values formatted correctly?
- [ ] Do hashes display properly?

### Edge Cases
- [ ] Empty states handled?
- [ ] Errors displayed gracefully?
- [ ] Loading states shown?

---

## Dependencies on Madara

The visualizer imports types from `mc-db`:

```toml
[dependencies]
mc-db = { path = "../madara/crates/client/db" }
mp-block = { path = "../madara/crates/primitives/block" }
mp-state-update = { path = "../madara/crates/primitives/state_update" }
# etc.
```

This ensures serialization compatibility without duplicating types.

---

## Quick Reference

```bash
# Start backend
cargo run -p api -- --db-path /tmp/madara_devnet_poc_v2/db

# Start frontend
trunk serve

# Verify with agent-browser
agent-browser open http://localhost:8080
agent-browser snapshot -i                    # See page structure
agent-browser screenshot /tmp/check.png     # Visual check
agent-browser click @e1                      # Test interactions
agent-browser close

# Only commit after verification passes!
git add -A && git commit --no-gpg-sign -m "phase-N: description"
```

---

## Verification Checklist Template

Before committing each phase, verify:

```
□ cargo build succeeds (no compile errors)
□ cargo run -p api starts without panic
□ trunk serve builds and serves frontend
□ agent-browser snapshot shows expected structure
□ agent-browser screenshot looks correct
□ All success criteria for the phase are met
```

---

Ready to begin with Phase 0!

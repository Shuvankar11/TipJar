## 🏗️ TipJar — Architecture

> A decentralized XLM tipping app on the Stellar blockchain. No backend business logic — all financial operations happen on-chain.

---

## Tech Stack

| Layer | Technology | Language |
|---|---|---|
| Frontend UI | React 18 (JSX) via SystemJS + Babel | JavaScript (JSX) |
| Static Server | Node.js `http` module | JavaScript (Node.js) |
| Wallet | Freighter & Albedo browser extensions | Browser JS API |
| Blockchain SDK | `stellar-sdk` v10.4.1 | JavaScript |
| Blockchain Network | Stellar Testnet / Mainnet | — |
| Database | Firebase Firestore | NoSQL |
| Tests | Node.js `assert` module | JavaScript (Node.js) |

---

## System Layers

```
┌─────────────────────────────────────────────────────┐
│                FRONTEND (Browser)                   │
│   React 18 · SystemJS + Babel · No build step       │
│  ┌───────────┐ ┌──────────┐ ┌────────┐ ┌────────┐  │
│  │  Balance  │ │ Send Tip │ │Received│ │  Sent  │  │
│  │    Tab    │ │   Tab    │ │  Tab   │ │  Tab   │  │
│  └───────────┘ └──────────┘ └────────┘ └────────┘  │
└──────────────────────┬──────────────────────────────┘
                       │ HTTP (static files)
┌──────────────────────▼──────────────────────────────┐
│              STATIC FILE SERVER                     │
│         Node.js · server.js · Port 8080             │
│      Serves index.html · SPA fallback routing       │
└──────────────────────┬──────────────────────────────┘
                       │ Browser Extension API
┌──────────────────────▼──────────────────────────────┐
│                 WALLET LAYER                        │
│       Freighter (freighter.app)                     │
│       Albedo    (albedo.link)                       │
│       Signs all transactions · Never stores keys    │
└──────────────────────┬──────────────────────────────┘
                       │ stellar-sdk v10
┌──────────────────────▼──────────────────────────────┐
│              STELLAR BLOCKCHAIN                     │
│  Horizon API · horizon-testnet.stellar.org          │
│  XLM Payments · Tx submission · Balance queries     │
└──────────────────────┬──────────────────────────────┘
                       │ Firebase JS SDK
┌──────────────────────▼──────────────────────────────┐
│               DATA LAYER (Off-chain)                │
│              Firebase Firestore (NoSQL)             │
│     Tip records · Creator profiles · Tx metadata   │
└─────────────────────────────────────────────────────┘
```

---

## Layer Breakdown

### 1. Frontend — React 18

No build step. JSX is transpiled **in the browser** via SystemJS + Babel. React & ReactDOM load from `unpkg.com` CDN.

**Four tabs:**

| Tab | Purpose |
|---|---|
| Balance | Wallet balance · Creator profile setup · Shareable tip link |
| Send Tip | Send XLM · Preset amounts · Optional message · Tx status |
| Received | Incoming tips · Sender info · Timestamps · Explorer links |
| Sent | Outgoing tips · Recipient info · Timestamps · Explorer links |

**Key frontend logic:**

```
shortenAddress(address)        → "GBBD47...FLA5"
validateTipAmount(amt, bal)    → min 0.1 XLM · positive · balance check
timeAgo(timestamp)             → "2h ago", "3d ago"
Freighter detection            → 1s delay · retries every 10ms · 5s timeout
```

---

### 2. Static File Server — Node.js

```
Entry:   server.js
Port:    8080
Deps:    None (built-in http + fs modules only)
```

- Serves all static files with correct MIME types
- Falls back to `index.html` for unknown routes (SPA routing)
- **No API endpoints. No database access. No business logic.**

---

### 3. Wallet Layer — Browser Extensions

| Wallet | Detection | Notes |
|---|---|---|
| Freighter | `window.freighter` | Chrome/Edge extension |
| Albedo | `window.albedo` | Chrome/Edge extension |

> 🔐 TipJar never accesses or stores private keys. All signing happens inside the wallet extension.

---

### 4. Blockchain — Stellar Network

```
SDK:        stellar-sdk v10.4.1
Testnet:    https://horizon-testnet.stellar.org
Mainnet:    https://horizon.stellar.org
Explorer:   https://stellar.expert  (linked from UI, read-only)
```

**Stellar rules enforced in the app:**

| Rule | Value |
|---|---|
| Address format | Starts with `G` · exactly 56 characters |
| Minimum tip | `0.1 XLM` |
| Network fee | `~0.00001 XLM` |
| Minimum reserve | `1 XLM` (not spendable) |

---

### 5. Data Layer — Firebase Firestore

**`tips` collection**

```
sender      string    Sender's Stellar address
receiver    string    Recipient's Stellar address
amount      number    XLM amount
message     string    Optional tip message
txHash      string    Stellar transaction hash
timestamp   Timestamp Firebase server timestamp
```

**`profiles` collection** *(keyed by wallet address)*

```
name               string    Creator display name
bio                string    Creator bio
allowedTipAmounts  array     Preset amounts e.g. [1, 5, 10, 25]
```

---

## Send Tip — Data Flow

```
User fills Send Tip form
        ↓
Frontend validates (address format · amount · balance)
        ↓
stellar-sdk builds Payment transaction
        ↓
Wallet extension signs the transaction
        ↓
stellar-sdk submits to Horizon API
        ↓
Horizon confirms · returns txHash
        ↓
Firebase stores tip record (sender, receiver, amount, txHash, timestamp)
        ↓
UI shows confirmation + Stellar Expert link
```

---

## Testing

```
Runner:   node tests/app.test.js
Command:  npm test
Deps:     Node.js assert module (no external framework)
```

| Test | Covers |
|---|---|
| Address shortener | `first6...last6` format · empty string |
| Tip amount validator | Empty · negative · below min · over balance · valid |
| Tip object structure | All fields present · correct types · Stellar address format |

---

## Project Structure

```
TipJar/
├── server.js           Static file server (port 8080)
├── config.js           SystemJS + Babel in-browser config
├── app.js              React app · wallet logic · Stellar SDK
├── index.html          Entry point
├── style.css           Global styles
├── package.json        Dependencies & scripts
└── tests/
    └── app.test.js     Test suite (3 tests)
```

---

## Key Dependencies

| Package | Version | Purpose |
|---|---|---|
| `stellar-sdk` | ^10.4.1 | Stellar blockchain interaction |
| `react` | 18.3.1 | UI framework (CDN) |
| `react-dom` | 18.3.1 | React DOM renderer (CDN) |
| `plugin-babel` | latest | In-browser JSX transpilation |
| Firebase JS SDK | — | Firestore database access |

---

> **Network:** Stellar Testnet · Switch Horizon URL to `https://horizon.stellar.org` for Mainnet production.

Environment: Localhost development / Stellar Testnet
Production path: Switch Horizon URL to https://horizon.stellar.org and use real XLM
### ARCHITECTURE DIAGRAM
![Architecture](./Screenshots/Architecture.png)

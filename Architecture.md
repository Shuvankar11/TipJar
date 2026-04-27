## Overview
TipJar is a decentralized Web3 tipping application built on the Stellar blockchain. It allows users to send XLM (Stellar Lumens) tips to creators directly via browser wallet extensions, with no backend business logic — all financial operations happen on-chain.

## Tech Stack Summary
LayerTechnologyLanguageFrontend UIReact 18 (JSX) via SystemJS + BabelJavaScript (JSX)Frontend TranspilerSystemJS + plugin-babelJavaScriptStatic File ServerNode.js http moduleJavaScript (Node.js)Wallet IntegrationFreighter & Albedo browser extensionsJavaScript (Browser API)Blockchain SDKstellar-sdk v10.4.1JavaScriptBlockchain NetworkStellar Testnet / Mainnet—Off-chain DatabaseFirebase Firestore (NoSQL)—TestsNode.js assert moduleJavaScript (Node.js)

## Architecture Layers : 
1. Frontend Layer — React 18 (JSX)
Language: JavaScript (JSX)
Runtime: Browser — loaded and transpiled in-browser via SystemJS + plugin-babel
The UI is a single-page application (SPA) built with React 18. There is no build step — JSX is transpiled at runtime in the browser using SystemJS as a module loader and the Babel plugin for transpilation. React and ReactDOM are loaded from the unpkg.com CDN as UMD globals.
UI structure — four main tabs:

Balance tab — Shows current XLM wallet balance; creator profile setup (name, bio, allowed tip amounts); shareable tip link generation.
Send Tip tab — Form to send XLM to any Stellar address; preset quick-select tip amounts; optional message field; transaction status feedback.
Received tab — Lists all incoming tips with sender address (shortened), amount, timestamp (relative time), and link to Stellar Expert explorer.
Sent tab — Lists all outgoing tips with the same fields.

State management: Local React state (useState, useReducer) — no external state library.
Key logic in frontend:

shortenAddress(address) — Formats Stellar addresses as GBBD47...FLA5
validateTipAmount(amount, balance) — Enforces minimum 0.1 XLM, positive value, and sufficient balance (with 1 XLM reserve)
timeAgo(timestamp) — Dynamic relative timestamps ("2h ago", "3d ago")
Freighter detection with 1-second delay + retry loop + 5-second timeout


2. Static File Server — Node.js
Language: JavaScript (Node.js)
Entry point: server.js
Port: 8080
Dependencies: None (uses Node.js built-in http and fs modules only)
A minimal HTTP server that:

Serves static files (index.html, app.js, style.css, etc.) with correct MIME types
Falls back to index.html for all unknown routes — enabling SPA client-side routing
Has no API endpoints, no authentication, no database access

This layer is purely a static asset delivery mechanism. All business logic lives in the browser.

3. Wallet Layer — Browser Extensions
Language: JavaScript (Browser Extension API)
Users connect one of two Stellar browser wallets:
WalletURLNotesFreighterhttps://freighter.appChrome/Firefox extension; detected via window.freighterAlbedohttps://albedo.linkBrowser extension + web-based signer
Key behaviors:

The app never accesses or stores private keys
All transaction signing happens inside the wallet extension
The app detects Freighter with a polling retry (checks every 10ms, up to 5 seconds after page load)
If no wallet is found, the UI shows install instructions with a download link


4. Blockchain Layer — Stellar Network
Language: JavaScript
Library: stellar-sdk v10.4.1
Network: Stellar Testnet (switchable to Mainnet)
Horizon API endpoints:

Testnet: https://horizon-testnet.stellar.org
Mainnet: https://horizon.stellar.org

On-chain operations:

Query wallet balance (XLM)
Build and submit XLM payment transactions
Each transaction carries an optional memo (tip message, truncated to Stellar's 28-byte limit)

Stellar address rules enforced in the app:

Must start with G
Must be exactly 56 characters
Minimum tip: 0.1 XLM
Network fee: ~0.00001 XLM
Minimum reserve (not spendable): 1 XLM

Stellar Expert (https://stellar.expert) is linked from the UI as a read-only blockchain explorer for verifying transaction hashes.
Testnet funding: New accounts can be funded via FriendBot at https://friendbot.stellar.org.

5. Data Layer — Firebase Firestore
Type: NoSQL document database (Google Firebase)
Access: Direct from browser (Firebase JS SDK)
Firebase stores two collections of off-chain metadata:
tips collection — Tip records
FieldTypeDescriptionsenderstringSender's Stellar address (56 chars)receiverstringRecipient's Stellar addressamountnumberXLM amount sentmessagestringOptional tip messagetxHashstringStellar transaction hashtimestampTimestampFirebase server timestamp
profiles collection — Creator profiles
FieldTypeDescriptionKeystringWallet address (document ID)namestringCreator display namebiostringCreator bioallowedTipAmountsarrayPreset tip amounts (e.g. [1, 5, 10, 25])
Firestore queries filter tip records by sender or receiver address to populate the Sent and Received tabs.

Data Flow — Sending a Tip
User fills "Send Tip" form
        │
        ▼
Frontend validates input (address format, amount ≥ 0.1 XLM, balance check)
        │
        ▼
stellar-sdk builds a Payment transaction object
        │
        ▼
Wallet extension (Freighter/Albedo) signs the transaction
        │
        ▼
stellar-sdk submits signed tx to Horizon API
        │
        ▼
Horizon confirms; returns transaction hash
        │
        ▼
Firebase Firestore stores tip record (sender, receiver, amount, txHash, timestamp)
        │
        ▼
UI shows confirmation + link to Stellar Expert

Testing
Framework: Plain Node.js assert module (no external test runner)
File: tests/app.test.js
Run: node tests/app.test.js
Three test cases:
TestWhat it checksAddress shortenershortenAddress() returns first6...last6 format; handles empty stringTip amount validatorRejects empty, negative, below-minimum, and over-balance amounts; accepts valid amountsTip object structureAll required fields exist with correct types; Stellar address format; positive amount

Key Dependencies
PackageVersionPurposestellar-sdk^10.4.1Stellar blockchain interactionreact18.3.1UI framework (loaded via CDN)react-dom18.3.1React DOM renderer (loaded via CDN)plugin-babellatestIn-browser JSX transpilationsystemjs—In-browser module loaderFirebase JS SDK—Firestore database access

Security Notes

Private keys are never stored or transmitted — all signing is handled by the wallet extension
Firebase Firestore stores only transaction references (hashes) and public wallet addresses
No passwords — identity is tied entirely to the wallet
No analytics or tracking
All transaction data on the Stellar blockchain is public


Current Deployment

Environment: Localhost development / Stellar Testnet
Production path: Switch Horizon URL to https://horizon.stellar.org and use real XLM

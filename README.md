# Blockchain Explorer and Validator

A decentralized blockchain explorer and transaction validator built with Node.js/Express for the backend, Rust/Solana for blockchain interaction, and Vue.js for the frontend. This project allows users to validate transactions, view the latest block information, and explore transaction details on the Solana blockchain.

---

## Table of Contents

- [Project Overview](#project-overview)
- [Tech Stack](#tech-stack)
- [Features](#features)
- [Getting Started](#getting-started)
- [Setting Up the Solana Environment](#setting-up-the-solana-environment)
- [Backend Setup (Node.js/Express)](#backend-setup-nodejsexpress)
- [Frontend Setup (Vue.js)](#frontend-setup-vuejs)
- [Testing and Deployment](#testing-and-deployment)
- [License](#license)

---

## Project Overview

This project is designed to interact with the Solana blockchain, providing a user interface to:
- **Validate Transactions** by checking their legitimacy based on sender, recipient, and amount.
- **Explore Blockchain Data** such as viewing the latest block and retrieving transaction details by transaction signature.

---

## Tech Stack

- **Backend**: Node.js/Express for API handling
- **Blockchain**: Solana (Rust for smart contract development)
- **Frontend**: Vue.js for the user interface

---

## Features

- **Transaction Validation**: Verify transactions by sender, recipient, and amount.
- **Blockchain Explorer**: View the latest block information and transaction details.
- **User Interface**: Intuitive, responsive frontend built with Vue.js.

---

## Getting Started

### Prerequisites

- **Node.js** (v14 or later)
- **Vue CLI** (for frontend setup)
- **Rust and Solana CLI** (for Solana blockchain interaction)
- **Yarn or NPM** (for package management)

---

## Setting Up the Solana Environment

1. **Install Rust**:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Install Solana CLI**:
   ```bash
   sh -c "$(curl -sSfL https://release.solana.com/v1.8.2/install)"
   ```

3. **Set up Solana Config**:
   - Configure Solana to use the testnet:
     ```bash
     solana config set --url https://api.testnet.solana.com
     ```
   - Generate a wallet and airdrop some tokens for testing:
     ```bash
     solana-keygen new --outfile ~/.config/solana/id.json
     solana airdrop 2
     ```

4. **Build and Deploy Solana Program**:
   - In the `solana-program` directory (containing Rust code):
     ```bash
     cargo build-bpf --manifest-path=Cargo.toml --bpf-out-dir=dist/program
     solana program deploy dist/program/program.so
     ```

   - Take note of the **program ID**; you’ll use it to connect your backend to this specific program on the blockchain.

---

## Backend Setup (Node.js/Express)

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/your-repo/blockchain-explorer-validator.git
   cd blockchain-explorer-validator/backend
   ```

2. **Install Dependencies**:
   ```bash
   npm install
   ```

3. **Configure Environment Variables**:
   - Create a `.env` file in the backend directory and add the following:
     ```
     SOLANA_CLUSTER_URL=https://api.testnet.solana.com
     SOLANA_PROGRAM_ID=Your_Program_ID
     ```

4. **Run the Backend Server**:
   ```bash
   npm start
   ```
   The backend API will be available at `http://localhost:3000`.

---

## Frontend Setup (Vue.js)

1. **Navigate to the Frontend Directory**:
   ```bash
   cd ../frontend
   ```

2. **Install Dependencies**:
   ```bash
   npm install
   ```

3. **Configure API Base URL**:
   - In `src/api/index.js`, set the base URL to your backend API (e.g., `http://localhost:3000/api`).

4. **Run the Frontend Server**:
   ```bash
   npm run serve
   ```
   The frontend will be available at `http://localhost:8080`.

---

## Testing and Deployment

### Testing on Local Environment

1. **Verify Solana Transactions**:
   - Use the frontend to validate transactions by filling in the sender, recipient, and amount details.
   - View the latest block and transaction details using their respective pages.

2. **Test Smart Contract**:
   - Verify the deployed smart contract using Solana's testnet, and ensure the transaction validation logic is correctly reflecting on the blockchain.

### Deploying on Solana Testnet

1. **Set Backend to Testnet**:
   - Ensure the backend's Solana cluster URL is set to `https://api.testnet.solana.com`.

2. **Deploy the Solana Program**:
   - Deploy the program on the Solana testnet if not already done. Keep the program ID handy.

3. **Serve Frontend and Backend Publicly**:
   - Use a hosting service like **Vercel** or **Netlify** for the frontend.
   - For the backend, use **Heroku** or **DigitalOcean**.

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

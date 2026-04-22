# Deployment Documentation for Solana Voting System

## Overview
This document outlines the setup and deployment procedures for the Solana-based voting system. Follow the steps carefully to ensure a smooth deployment.

## Prerequisites
Before you begin, make sure you have the following installed:
- Node.js (version 14 or higher)
- npm (Node package manager)
- Solana CLI 
- Rust (latest stable version)

## Installation Steps
1. **Clone the Repository**
   Open your terminal and run:
   ```bash
   git clone https://github.com/DVO1111/Voting-system.git
   cd Voting-system
   ```

2. **Install Dependencies**
   Navigate to the project directory and run:
   ```bash
   npm install
   ```

3. **Set Up Solana Environment**
   If you haven’t set up your Solana CLI before, run:
   ```bash
   solana config set --url https://api.mainnet-beta.solana.com
   ```

4. **Build the Project**
   Use this command to build the project:
   ```bash
   npm run build
   ```

## Configuration Steps
1. **Configure the Environment**
   Create a `.env` file in the root directory and add the following configurations:
   ```bash
   SOLANA_URL=https://api.mainnet-beta.solana.com
   VOTING_CONTRACT_ADDRESS=<your_contract_address>
   ```

2. **Set the Network**
   Make sure you're connected to the correct Solana network, valid options include `devnet`, `testnet`, or `mainnet-beta`.

## Testing Steps
1. **Run Unit Tests**
   After installation, run the unit tests to ensure everything is working:
   ```bash
   npm test
   ```

2. **Deploy the Smart Contract**
   Deploy your smart contract to the Solana network using the following command:
   ```bash
   anchor deploy
   ```

## Final Steps
- Ensure your wallet is funded for transaction fees.
- Start the application with:
  ```bash
  npm start
  ```

## Troubleshooting
If you encounter issues:
- Verify that your Solana CLI is properly configured.
- Check your internet connection.
- Review the console for error messages and refer to the documentation.

## Conclusion
With this documentation, you should be able to set up and deploy the Solana voting system successfully. For further assistance, refer to the official Solana documentation or contact support.

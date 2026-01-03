import "./App.css";

import React, { FC, ReactNode, useMemo } from "react";
import {
  ConnectionProvider,
  WalletProvider,
} from "@solana/wallet-adapter-react";
import { WalletAdapterNetwork } from "@solana/wallet-adapter-base";
import {
  WalletModalProvider,
  WalletMultiButton,
} from "@solana/wallet-adapter-react-ui";
import { clusterApiUrl } from "@solana/web3.js";
import "@solana/wallet-adapter-react-ui/styles.css";
import { PhantomWalletAdapter } from "@solana/wallet-adapter-wallets";
import "@solana/wallet-adapter-react-ui/styles.css";

const AppFunction = () => {
  return (
    <div className="app">
      <header className="navbar">
        <h1>Lottery DAPP 🏆</h1>
        {/* <button className="wallet-btn">Connect Wallet</button> */}
        <WalletMultiButton />
      </header>

      <main className="main">
        <div className="card">
          <h2>Lottery #3</h2>

          <p className="pot">
            🪙 Pot: <span>1000 SOL</span>
          </p>

          <p className="winner">
            🏆 Recent Winner: <span>11111...11111</span>
          </p>

          <button className="action-btn" onClick={() => console.log("Enter")}>
            Enter
          </button>

          <button
            className="action-btn"
            onClick={() => console.log("Pick Winner")}
          >
            Pick Winner
          </button>

          <button
            className="action-btn"
            onClick={() => console.log("Create Lottery")}
          >
            Create Lottery
          </button>
        </div>

        <div className="table-wrapper">
          <table>
            <thead>
              <tr>
                <th>#</th>
                <th>Address</th>
                <th>Ticket</th>
                <th>Amount</th>
              </tr>
            </thead>
            <tbody>
              <tr>
                <td>1</td>
                <td>11111...11111</td>
                <td>#2</td>
                <td>+15 SOL</td>
              </tr>
            </tbody>
          </table>
        </div>
      </main>
    </div>
  );
};

export default function App() {
  // The network can be set to 'devnet', 'testnet', or 'mainnet'
  const network = WalletAdapterNetwork.Devnet;

  // You can also provide a custom RPC endpoint
  const endpoint = useMemo(() => clusterApiUrl(network), [network]);

  const wallets = useMemo(
    () => [new PhantomWalletAdapter()],
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [network]
  );
  return (
    <ConnectionProvider endpoint={endpoint}>
      <WalletProvider wallets={wallets} autoConnect>
        <WalletModalProvider>
          <AppFunction />
        </WalletModalProvider>
      </WalletProvider>
    </ConnectionProvider>
  );
}

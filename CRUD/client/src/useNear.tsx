import * as nearAPI from "near-api-js";
import { useEffect, useMemo, useState } from "react";
// import { useLocation, useSearchParams } from "react-router-dom";
import { CURDContract } from "./types";

const nearConfig: nearAPI.ConnectConfig = {
  networkId: "testnet",
  nodeUrl: "https://rpc.testnet.near.org",
  walletUrl: "https://wallet.testnet.near.org",
  helperUrl: "https://helper.testnet.near.org",
  headers: {
    "Content-Type": "application/json",
  },
  deps: {
    keyStore: new nearAPI.keyStores.BrowserLocalStorageKeyStore(),
  },
};

export const useNear = () => {
  const [near, setNear] = useState<nearAPI.Near>();
  const [wallet, setWallet] = useState<nearAPI.WalletConnection>();
  const [profile, setProfile] = useState<any>();

  const crudContract = useMemo(() => {
    if (!profile || !wallet) return null;
    const contractAddr =
      process.env.REACT_APP_CRUD_CONTRACT || "dev-1653298028614-29625999758418";
    const contract = new nearAPI.Contract(wallet.account(), contractAddr, {
      changeMethods: ["create_update", "delete"],
      viewMethods: ["read", "read_keys"],
    }) as CURDContract;
    return contract;
  }, [profile, wallet]);

  const isReady = useMemo(() => {
    return !!profile && !!wallet && !!near && !!crudContract;
  }, [profile, wallet, near, crudContract]);

  const connectNear = async () => {
    const near = await nearAPI.connect({
      ...nearConfig,
    });
    setNear(near!);
  };

  const connectWallet = () => {
    if (!near) return;
    const walletConnection = new nearAPI.WalletConnection(near, null);
    setWallet(walletConnection);
  };

  const refreshProfile = async () => {
    if (!wallet) return;
    wallet
      ?.account()
      .state()
      .then((resp) => {
        setProfile({
          balance: resp.amount,
          accountId: wallet?.getAccountId(),
        });
      });
  };

  const signIn = () => {
    wallet!.requestSignIn(
      undefined, // contract requesting access
      "Nolan App" // optional
    );
  };

  // const connectCrudContract = () => {
  //   if (!wallet || !profile) return;
  //   const contractAddr =
  //     process.env.REACT_APP_CRUD_CONTRACT || "dev-1653298028614-29625999758418";
  //   const contract = new nearAPI.Contract(wallet.account(), contractAddr, {
  //     changeMethods: ["create_update", "delete"],
  //     viewMethods: ["read"],
  //   }) as CURDContract;
  //   setContract(contract);
  // };

  useEffect(() => {
    connectNear();
  }, []);

  useEffect(() => {
    connectWallet();
  }, [near]);

  useEffect(() => {
    refreshProfile();
  }, [wallet]);

  return { profile, onSignIn: signIn, crudContract, isReady };
};

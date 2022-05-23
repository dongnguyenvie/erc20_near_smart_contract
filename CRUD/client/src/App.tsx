import React, { useEffect, useState } from "react";
import { useNear } from "./useNear";
import { useQuery } from "react-query";
import "./App.css";

function App() {
  const { profile, onSignIn, isReady, crudContract } = useNear();
  const [map, setMap] = useState<string[][]>([]);
  const { data: keys } = useQuery(
    "read_keys",
    () => crudContract?.read_keys(),
    {
      enabled: isReady,
      refetchInterval: 30000,
    }
  );

  useEffect(() => {
    if (!isReady) return;
    if (!keys?.length) {
      setMap([]);
      return;
    }
    Promise.all(keys.map((key) => crudContract?.read({ k: key }))).then(
      (values) => {
        setMap(keys.map((key, idx) => [key, values[idx]]) as string[][]);
      }
    );
  }, [keys, isReady]);

  return (
    <div className="App">
      {!profile && <button onClick={onSignIn}>SignIn</button>}
      {!!profile && (
        <>
          <p>accountId: {profile?.accountId}</p>
          <p>balance: {profile?.balance}</p>
        </>
      )}
      <hr />
      <header className="App-header">
        {/* <div>{profile?.accountId}</div> */}
        {/* <div>{profile?.balance}</div> */}
      </header>
      <hr />
      <div className="table">
        <h2>HashMap CRUD</h2>

        <table>
          <tr>
            <th>Key</th>
            <th>Value</th>
          </tr>
          {map.map((pair, idx) => (
            <tr key={idx}>
              <td>{pair[0]}</td>
              <td>{pair[1]}</td>
            </tr>
          ))}
        </table>
      </div>
    </div>
  );
}

export default App;

import React, { useEffect, useState } from "react";
import { useNear } from "./useNear";
import { useQuery } from "react-query";
import "./App.css";

function App() {
  const { profile, onSignIn, isReady, crudContract } = useNear();
  const [map, setMap] = useState<string[][]>([]);
  const [key, setKey] = useState("");
  const [value, setValue] = useState("");
  const { data: keys, refetch: readKeysRefetch } = useQuery(
    "read_keys",
    () => crudContract?.read_keys(),
    {
      enabled: isReady,
      refetchInterval: 30000,
    }
  );

  const changeKey = (e: any) => {
    const text = e.target.value;
    setKey(text);
  };

  const changeValue = (e: any) => {
    const value = e.target.value;
    setValue(value);
  };

  const submit = async () => {
    try {
      await crudContract?.create_update({ k: key, v: value });
      setKey("");
      setValue("");
      readKeysRefetch();
    } catch (error) {}
  };

  const deleteKey = async (key: string) => {
    try {
      await crudContract?.delete({ k: key });
      readKeysRefetch();
    } catch (error) {}
  };

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
        <header>
          <p>accountId: {profile?.accountId}</p>
          <p>balance: {profile?.balance}</p>
        </header>
      )}
      <hr />
      <input type="text" value={key} onChange={changeKey} placeholder="key" />
      <input
        type="text"
        value={value}
        onChange={changeValue}
        placeholder="value"
      />
      <button onClick={submit}>Create</button>
      <hr />
      <div className="table">
        <h2>HashMap CRUD</h2>

        <table>
          <tbody>
            <tr>
              <th>Key</th>
              <th>Value</th>
              <th>Action</th>
            </tr>
            {map.map((pair, idx) => (
              <tr key={idx}>
                <td>{pair[0]}</td>
                <td>{pair[1]}</td>
                <td>
                  <button onClick={() => deleteKey(pair[0])}>Delete</button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}

export default App;

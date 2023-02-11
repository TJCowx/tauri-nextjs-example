import { invoke } from '@tauri-apps/api/tauri';
import { useState } from 'react';

export default function Home() {
  const [apiRes, setApiRes] = useState('uncalled');
  const [rustRes, setRustRes] = useState('uncalled');

  // This will fail in a production build
  const callApi = () => {
    fetch('/api/hello', { method: 'GET' })
      .then(async (res) => {
        const parsed = await res.json();
        setApiRes(parsed.name);
      })
      .catch((e) => {
        console.log(e);
        setApiRes('Call Failed');
      });
  };

  // This works in both development and production builds
  const callRust = (nested: boolean) => {
    invoke(nested ? 'hello_nested' : 'test_call')
      .then((res) => setRustRes(res as string))
      .catch((e) => {
        console.error(e);
        setRustRes('Call Failed');
      });
  };

  return (
    <main style={{ paddingTop: '16px' }}>
      <h2>Hello world!</h2>
      <br />
      <button onClick={callApi}>Test NextJs API Route Call</button>
      <div>Response: {apiRes}</div>
      <br />
      <hr />
      <br />
      <button onClick={() => callRust(false)}>Test Rust API Call</button>
      <div>Response: {rustRes}</div>
      <br />
      <hr />
      <br />
      <button onClick={() => callRust(true)}>Test Nested Rust API Call</button>
      <div>Response: {rustRes}</div>
    </main>
  );
}

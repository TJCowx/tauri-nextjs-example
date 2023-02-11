import { useState } from 'react';

export default function Home() {
  const [apiRes, setApiRes] = useState('uncalled');

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

  return (
    <main>
      Hello world!
      <br />
      <button onClick={callApi}>Test API Call</button>
      <div>Response: {apiRes}</div>
    </main>
  );
}

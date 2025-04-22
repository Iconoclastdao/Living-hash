import React, { useState } from "react";

function App() {
  const [input, setInput] = useState("");
  const [output, setOutput] = useState("");
  const [trace, setTrace] = useState([]);

  const handleAbsorb = async () => {
    const response = await fetch("http://127.0.0.1:8080/absorb", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ input }),
    });
    if (response.ok) alert("Input absorbed successfully!");
  };

  const handleSqueeze = async () => {
    const length = 32; // Example: fixed output length
    const response = await fetch("http://127.0.0.1:8080/squeeze", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ length }),
    });
    const data = await response.text();
    setOutput(data);
  };

  const fetchTrace = async () => {
    const response = await fetch("http://127.0.0.1:8080/trace");
    const data = await response.json();
    setTrace(data.trace);
  };

  return (
    <div className="App">
      <h1>Living Hash Engine</h1>
      <textarea
        placeholder="Enter input..."
        value={input}
        onChange={(e) => setInput(e.target.value)}
      />
      <button onClick={handleAbsorb}>Absorb</button>
      <button onClick={handleSqueeze}>Squeeze</button>
      <button onClick={fetchTrace}>View Trace</button>
      <h2>Output</h2>
      <pre>{output}</pre>
      <h2>Trace Log</h2>
      <pre>{JSON.stringify(trace, null, 2)}</pre>
    </div>
  );
}

export default App;
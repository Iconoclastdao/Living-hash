import React, { useState } from "react";
import { ethers } from "ethers";

const SymbolicInteraction = ({ contract }) => {
  const [message, setMessage] = useState("");
  const [response, setResponse] = useState("");

  const handleSubmit = async () => {
    try {
      const tx = await contract.invoke(message);
      await tx.wait();
      setResponse("Invocation successful! Check contract for vision.");
    } catch (err) {
      console.error(err);
      setResponse("Error invoking symbolic prayer.");
    }
  };

  return (
    <div>
      <h1>Symbolic Interaction</h1>
      <textarea
        value={message}
        onChange={(e) => setMessage(e.target.value)}
        placeholder="Enter your prayer or message..."
      />
      <button onClick={handleSubmit}>Invoke</button>
      <div>{response}</div>
    </div>
  );
};

export default SymbolicInteraction;
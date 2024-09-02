import { useEffect, useState } from 'preact/hooks'
import './app.css'
import './app.sass'
import { createEditor } from "./editor";
import { useRete } from "rete-react-plugin";

export function App() {
  const [count, setCount] = useState(0)
  const [ref] = useRete(createEditor);

  const [lastError, setLastError] = useState("Altercation");

  window.addEventListener("message", (event) => {
    const message = event.data;
    switch (message.type) {
      case "error":
        setLastError(message.error);
        break;
      default:
        break;
    }
  });

  return (
    <>
    <h1 style={{position:'absolute', right:0, bottom:0, marginRight: '1em', opacity: 0.5, userSelect: 'none'}}>{lastError}</h1>
      <div class="node-editor" ref={ref} style={{ height: "100vh", width: "100vw" }}></div>
    </>
  )
}

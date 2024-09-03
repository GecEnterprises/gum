import './app.css'
import './app.sass'
import { createEditor } from "../editor/editor";
import { useRete } from "rete-react-plugin";
import { LogPane, useLoggerStore } from '../editor/LogPane';
import { useEffect } from 'preact/hooks';

export function App() {
  const [ref] = useRete(createEditor);
  const logger = useLoggerStore();

  useEffect(() => {
    window.addEventListener("message", (event) => {
      const message = event.data;
      switch (message.type) {
        case "error":
          logger.append(message.error);
          break;
      }
    });
  }, [])

  return (
    <>
      <LogPane style={{position: "fixed", bottom: 0, right: 0, marginRight: '1em', marginBottom: '1em'}}/>
      <div class="node-editor" ref={ref} style={{ height: "100vh", width: "100vw" }}></div>
    </>
  )
}

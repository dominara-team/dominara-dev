import { useState, useEffect } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [updateStatus, setUpdateStatus] = useState("");
  const [downloadProgress, setDownloadProgress] = useState(0);
  const [totalSize, setTotalSize] = useState(0);
  const [isDownloading, setIsDownloading] = useState(false);

  useEffect(() => {
    const unlisten = listen("update-progress", (event) => {
      const data = event.payload;
      switch (data.event) {
        case "started":
          setIsDownloading(true);
          setDownloadProgress(0);
          setUpdateStatus(`Starting download of version ${data.data.version}...`);
          break;
        case "progress":
          setDownloadProgress(data.data.chunkLength);
          setTotalSize(data.data.total);
          break;
        case "finished":
          setIsDownloading(false);
          setUpdateStatus("Update downloaded successfully! The app will restart to apply the update.");
          break;
      }
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  async function checkForUpdates() {
    try {
      setUpdateStatus("Checking for updates...");
      const result = await invoke("check_update");
      setUpdateStatus(result);
    } catch (error) {
      setUpdateStatus(`Error: ${error}`);
    }
  }

  async function greet() {
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <main className="container">
      <h1>Welcome to Tauri + React</h1>

      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <div className="row">
        <button onClick={checkForUpdates}>Check for Updates</button>
      </div>
      {updateStatus && (
        <div className="update-status">
          <p>{updateStatus}</p>
        </div>
      )}
      {isDownloading && (
        <div className="download-progress">
          <div className="progress-bar">
            <div
              className="progress-fill"
              style={{
                width: `${(downloadProgress / totalSize) * 100}%`,
              }}
            />
          </div>
          <p>
            Downloaded: {Math.round(downloadProgress / 1024)} KB / {Math.round(totalSize / 1024)} KB
          </p>
        </div>
      )}

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input id="greet-input" onChange={(e) => setName(e.currentTarget.value)} placeholder="Enter a name..." />
        <button type="submit">Greet</button>
      </form>
      <p>{greetMsg}</p>
    </main>
  );
}

export default App;
